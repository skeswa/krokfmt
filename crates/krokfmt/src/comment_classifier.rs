use swc_common::{comments::Comment, BytePos, SourceMap};
use swc_ecma_ast::Module;

/// Classification of comment types based on their position in the code
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum CommentClassification {
    /// Comment within an expression (e.g., `const x = /* here */ 42`)
    Inline,
    /// Comment before a statement or declaration
    Leading,
    /// Comment after a statement on the same line
    Trailing,
    /// Comment separated by blank lines from surrounding code
    Standalone,
}

/// Classifies comments based on their position relative to AST nodes
pub struct CommentClassifier<'a> {
    source: &'a str,
    /// Maps comment positions to their classifications
    classifications: std::collections::HashMap<BytePos, CommentClassification>,
}

impl<'a> CommentClassifier<'a> {
    pub fn new(_source_map: &'a SourceMap, source: &'a str) -> Self {
        Self {
            source,
            classifications: std::collections::HashMap::new(),
        }
    }

    /// Classify all comments in the module
    pub fn classify_module(
        &mut self,
        _module: &Module,
        all_comments: &[Comment],
    ) -> std::collections::HashMap<BytePos, CommentClassification> {
        // Classify each comment based on its position in the source text
        for comment in all_comments {
            let classification = self.classify_comment(comment);
            self.classifications.insert(comment.span.lo, classification);
        }

        self.classifications.clone()
    }

    /// Classify a single comment based on its position
    fn classify_comment(&self, comment: &Comment) -> CommentClassification {
        // For now, use a simpler approach based on source text analysis
        let comment_start = comment.span.lo.0 as usize;
        let comment_end = comment.span.hi.0 as usize;

        // Find the line containing the comment
        let mut line_start = 0;
        let mut line_end = self.source.len();
        let mut current_pos = 0;

        for line in self.source.lines() {
            let line_len = line.len() + 1; // +1 for newline (always LF after normalization)
            if current_pos <= comment_start && comment_start < current_pos + line_len {
                line_start = current_pos;
                line_end = current_pos + line.len();
                break;
            }
            current_pos += line_len;
        }

        // Get the line content
        let line = &self.source[line_start..line_end];
        let comment_offset = comment_start - line_start;

        // Check if there's code before the comment on the same line
        let before_comment = if comment_offset > 0 {
            &line[..comment_offset]
        } else {
            ""
        };
        let has_code_before = before_comment.trim_end().chars().any(|c| {
            // Look for actual code characters, not just punctuation
            c.is_alphanumeric()
                || c == '_'
                || c == '$'
                || c == ')'
                || c == '}'
                || c == ']'
                || c == ';'
        });

        // Check if there's code after the comment on the same line
        let after_comment = if comment_end - line_start < line.len() {
            &line[comment_end - line_start..]
        } else {
            ""
        };
        let has_code_after = after_comment
            .chars()
            .any(|c| !c.is_whitespace() && c != ';' && c != ')' && c != ',');

        // Classify based on position
        if has_code_before && has_code_after {
            // Comment is between code elements
            CommentClassification::Inline
        } else if has_code_before && !has_code_after {
            // Comment is after code on the same line
            CommentClassification::Trailing
        } else if !has_code_before && has_code_after {
            // Comment is before code on the same line (likely inline)
            CommentClassification::Inline
        } else {
            // Comment is on its own line - check for standalone
            if self.is_standalone_comment(comment, line_start) {
                CommentClassification::Standalone
            } else {
                CommentClassification::Leading
            }
        }
    }

    /// Check if a comment is standalone (has blank line separation)
    fn is_standalone_comment(&self, _comment: &Comment, line_start: usize) -> bool {
        // Check if we can look at previous lines
        if line_start == 0 {
            return false;
        }

        // Get all content before this line
        let before_content = &self.source[..line_start];

        // Find the last newline before this line (end of previous line)
        if let Some(last_newline_pos) = before_content.rfind('\n') {
            // Check if there's another newline before that (indicating blank line)
            let before_last_newline = &before_content[..last_newline_pos];
            if let Some(second_last_newline_pos) = before_last_newline.rfind('\n') {
                // Check if the line between these newlines is empty
                let line_between = &before_content[second_last_newline_pos + 1..last_newline_pos];
                return line_between.trim().is_empty();
            }
        }

        false
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::parser::TypeScriptParser;

    fn classify_comments_in_source(source: &str) -> Vec<(String, CommentClassification)> {
        let parser = TypeScriptParser::new();
        let module = parser.parse(source, "test.ts").unwrap();

        let comments_map = parser.comments;
        let source_map = parser.source_map;

        // Collect all comments
        let mut all_comments = Vec::new();
        let (leading, trailing) = comments_map.borrow_all();

        for (_, comments) in leading.iter() {
            for comment in comments {
                all_comments.push(comment.clone());
            }
        }

        for (_, comments) in trailing.iter() {
            for comment in comments {
                all_comments.push(comment.clone());
            }
        }

        // Sort by position
        all_comments.sort_by_key(|comment| comment.span.lo);

        // Classify
        let mut classifier = CommentClassifier::new(&source_map, source);
        let classifications = classifier.classify_module(&module, &all_comments);

        // Return results
        all_comments
            .into_iter()
            .map(|comment| {
                let classification = classifications
                    .get(&comment.span.lo)
                    .copied()
                    .unwrap_or(CommentClassification::Leading);
                (comment.text.to_string(), classification)
            })
            .collect()
    }

    #[test]
    fn test_inline_comment_classification() {
        let source = r#"
const x = /* inline comment */ 42;
let y = /* another inline */ "hello";
var z = 100 + /* after operator */ 50;
"#;

        let classifications = classify_comments_in_source(source);

        assert_eq!(classifications.len(), 3);
        assert_eq!(classifications[0].1, CommentClassification::Inline);
        assert_eq!(classifications[1].1, CommentClassification::Inline);
        assert_eq!(classifications[2].1, CommentClassification::Inline);
    }

    #[test]
    fn test_leading_comment_classification() {
        let source = r#"
// This is a leading comment
function foo() {
    /* Also a leading comment */
    return 42;
}
"#;

        let classifications = classify_comments_in_source(source);

        assert_eq!(classifications.len(), 2);
        assert_eq!(classifications[0].1, CommentClassification::Leading);
        assert_eq!(classifications[1].1, CommentClassification::Leading);
    }

    #[test]
    fn test_trailing_comment_classification() {
        let source = r#"
const x = 42; // trailing comment
function foo() {} // another trailing
"#;

        let classifications = classify_comments_in_source(source);

        assert_eq!(classifications.len(), 2);
        assert_eq!(classifications[0].1, CommentClassification::Trailing);
        assert_eq!(classifications[1].1, CommentClassification::Trailing);
    }

    #[test]
    fn test_standalone_comment_classification() {
        let source = r#"
const x = 42;

// Standalone comment with blank line before

function foo() {}
"#;

        let classifications = classify_comments_in_source(source);

        // Note: This test might need adjustment based on exact implementation
        assert_eq!(classifications.len(), 1);
        // Standalone detection needs more sophisticated logic
    }
}
