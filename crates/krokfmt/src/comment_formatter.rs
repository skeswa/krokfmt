use anyhow::Result;
use swc_common::{comments::SingleThreadedComments, sync::Lrc, SourceMap};
use swc_ecma_ast::Module;

use crate::{
    codegen::CodeGenerator, comment_classifier::CommentClassification,
    comment_extractor::CommentExtractor, comment_reinserter::CommentReinserter,
    organizer::KrokOrganizer, selective_comment_handler::SelectiveCommentHandler,
};

/// Main comment-aware formatter for krokfmt
///
/// This formatter uses selective comment preservation to maintain inline comments
/// in the AST while extracting and reinserting other comments.
pub struct CommentFormatter {
    source_map: Lrc<SourceMap>,
    comments: SingleThreadedComments,
}

impl CommentFormatter {
    pub fn new(source_map: Lrc<SourceMap>, comments: SingleThreadedComments) -> Self {
        Self {
            source_map,
            comments,
        }
    }

    /// Format a module with selective comment preservation
    pub fn format(&self, module: Module, source: &str) -> Result<String> {
        // Phase 1: Separate inline from non-inline comments
        let (inline_only_comments, _non_inline_comments) =
            SelectiveCommentHandler::extract_non_inline_comments(
                &self.comments,
                &module,
                source,
                &self.source_map,
            );

        // Phase 2: Extract ALL comments (we'll filter later)
        let extractor = CommentExtractor::with_source(&self.comments, source.to_string());
        let mut extracted_comments = extractor.extract(&module);

        // Phase 2b: Get all inline comment positions to filter them out
        let all_comments: Vec<_> = {
            let (leading, trailing) = self.comments.borrow_all();
            let mut comments = Vec::new();
            for (_, vec) in leading.iter() {
                comments.extend(vec.iter().cloned());
            }
            for (_, vec) in trailing.iter() {
                comments.extend(vec.iter().cloned());
            }
            comments
        };

        let mut classifier =
            crate::comment_classifier::CommentClassifier::new(&self.source_map, source);
        let classifications = classifier.classify_module(&module, &all_comments);

        let inline_positions: std::collections::HashSet<_> = classifications
            .iter()
            .filter(|(_, class)| **class == CommentClassification::Inline)
            .map(|(pos, _)| *pos)
            .collect();

        // Remove inline comments from extracted comments
        for (_, comments) in extracted_comments.node_comments.iter_mut() {
            comments.retain(|c| !inline_positions.contains(&c.comment.span.lo));
        }

        extracted_comments
            .standalone_comments
            .retain(|c| !inline_positions.contains(&c.comment.span.lo));

        // Phase 3: Organize the AST using the organizer
        let organizer = KrokOrganizer::new();
        let organized_module = organizer.organize(module)?;

        // Phase 4: Generate code WITH inline comments (they're preserved)
        let generator = CodeGenerator::with_comments(self.source_map.clone(), inline_only_comments);
        let code_with_inline_comments = generator.generate(&organized_module)?;

        // Phase 5: Reinsert only non-inline comments at the correct positions
        let mut reinserter = CommentReinserter::new(extracted_comments);
        let final_code = reinserter.reinsert_comments(&code_with_inline_comments)?;

        Ok(final_code)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::parser::TypeScriptParser;

    fn format_with_comments(source: &str) -> Result<String> {
        let parser = TypeScriptParser::new();
        let module = parser.parse(source, "test.ts")?;

        let formatter = CommentFormatter::new(parser.source_map.clone(), parser.comments.clone());

        formatter.format(module, source)
    }

    #[test]
    fn test_inline_comments_preserved() {
        let source = r#"
const x = /* inline comment */ 42;
let y = /* another inline */ "hello";
"#;

        let result = format_with_comments(source).unwrap();
        // Inline comments should be preserved in their exact positions
        assert!(result.contains("const x = /* inline comment */ 42"));
        assert!(result.contains("let y = /* another inline */ \"hello\""));
    }

    #[test]
    fn test_leading_trailing_comments() {
        let source = r#"
// Leading comment
const x = 42; // Trailing comment
"#;

        let result = format_with_comments(source).unwrap();

        // Non-inline comments should still be preserved
        assert!(result.contains("// Leading comment"));
        assert!(result.contains("// Trailing comment"));
        assert!(result.contains("const x = 42"));
    }

    #[test]
    fn test_mixed_comment_types() {
        let source = r#"
// This is a leading comment
function foo(/* param comment */ x: number) {
    return x * 2; // multiply by two
}
"#;

        let result = format_with_comments(source).unwrap();
        // All comment types should be preserved
        assert!(result.contains("// This is a leading comment"));
        assert!(result.contains("/* param comment */"));
    }
}
