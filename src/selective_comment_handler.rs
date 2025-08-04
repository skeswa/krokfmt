use std::collections::HashSet;
use swc_common::{
    comments::{Comment, Comments, SingleThreadedComments},
    BytePos,
};
use swc_ecma_ast::Module;

use crate::comment_classifier::{CommentClassification, CommentClassifier};

/// Handles selective extraction and preservation of comments
pub struct SelectiveCommentHandler;

impl SelectiveCommentHandler {
    /// Extract non-inline comments from the comment map and return them separately
    /// Returns: (inline_comments_only, extracted_non_inline_comments)
    pub fn extract_non_inline_comments(
        comments: &SingleThreadedComments,
        module: &Module,
        source: &str,
        source_map: &swc_common::SourceMap,
    ) -> (
        SingleThreadedComments,
        Vec<(BytePos, Comment, CommentClassification)>,
    ) {
        // First, classify all comments
        let mut all_comments = Vec::new();
        let (leading, trailing) = comments.borrow_all();

        // Collect all comments with their positions
        for (&pos, comment_vec) in leading.iter() {
            for comment in comment_vec {
                all_comments.push((pos, comment.clone(), true)); // true = leading
            }
        }

        for (&pos, comment_vec) in trailing.iter() {
            for comment in comment_vec {
                all_comments.push((pos, comment.clone(), false)); // false = trailing
            }
        }

        // Sort by position for consistent ordering
        all_comments.sort_by_key(|(_, comment, _)| comment.span.lo);

        // Classify comments
        let mut classifier = CommentClassifier::new(source_map, source);
        let comment_refs: Vec<Comment> = all_comments.iter().map(|(_, c, _)| c.clone()).collect();
        let classifications = classifier.classify_module(module, &comment_refs);

        // Separate inline from non-inline comments
        let mut inline_positions = HashSet::new();
        let mut non_inline_comments = Vec::new();

        for (pos, comment, is_leading) in all_comments {
            let classification = classifications
                .get(&comment.span.lo)
                .copied()
                .unwrap_or(CommentClassification::Leading);

            match classification {
                CommentClassification::Inline => {
                    inline_positions.insert((pos, is_leading, comment.span.lo));
                }
                _ => {
                    non_inline_comments.push((pos, comment, classification));
                }
            }
        }

        // Create new SingleThreadedComments with only inline comments
        let inline_only = SingleThreadedComments::default();
        let (leading, trailing) = comments.borrow_all();

        // Copy only inline comments to the new container
        for (&pos, comment_vec) in leading.iter() {
            let inline_comments: Vec<Comment> = comment_vec
                .iter()
                .filter(|c| inline_positions.contains(&(pos, true, c.span.lo)))
                .cloned()
                .collect();

            for comment in inline_comments {
                inline_only.add_leading(pos, comment);
            }
        }

        for (&pos, comment_vec) in trailing.iter() {
            let inline_comments: Vec<Comment> = comment_vec
                .iter()
                .filter(|c| inline_positions.contains(&(pos, false, c.span.lo)))
                .cloned()
                .collect();

            for comment in inline_comments {
                inline_only.add_trailing(pos, comment);
            }
        }

        (inline_only, non_inline_comments)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::parser::TypeScriptParser;

    #[test]
    fn test_selective_extraction() {
        let source = r#"
// Leading comment
const x = /* inline */ 42; // trailing comment

/* standalone comment */

function foo() {
    return /* inline return */ true;
}
"#;

        let parser = TypeScriptParser::new();
        let module = parser.parse(source, "test.ts").unwrap();

        let (inline_only, non_inline) = SelectiveCommentHandler::extract_non_inline_comments(
            &parser.comments,
            &module,
            source,
            &parser.source_map,
        );

        // Check that we extracted the right number of comments
        let (inline_leading, inline_trailing) = inline_only.borrow_all();
        let inline_count = inline_leading.values().map(|v| v.len()).sum::<usize>()
            + inline_trailing.values().map(|v| v.len()).sum::<usize>();

        assert_eq!(inline_count, 2); // Two inline comments
        assert_eq!(non_inline.len(), 3); // Three non-inline comments

        // Verify classifications
        for (_, _, classification) in &non_inline {
            assert_ne!(*classification, CommentClassification::Inline);
        }
    }
}
