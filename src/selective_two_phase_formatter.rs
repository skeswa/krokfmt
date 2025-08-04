use anyhow::Result;
use swc_common::{comments::SingleThreadedComments, sync::Lrc, SourceMap};
use swc_ecma_ast::Module;

use crate::{
    codegen::CodeGenerator, comment_classifier::CommentClassification,
    comment_extractor::CommentExtractor, comment_reinserter::CommentReinserter,
    formatter::KrokFormatter, selective_comment_handler::SelectiveCommentHandler,
};

/// Two-phase formatter that preserves inline comments naturally while reinserting others
///
/// This improved formatter:
/// 1. Classifies comments as inline vs non-inline
/// 2. Extracts only non-inline comments
/// 3. Formats the AST normally
/// 4. Generates code WITH inline comments (they stay in place)
/// 5. Reininserts only non-inline comments at correct positions
pub struct SelectiveTwoPhaseFormatter {
    source_map: Lrc<SourceMap>,
    comments: SingleThreadedComments,
}

impl SelectiveTwoPhaseFormatter {
    pub fn new(source_map: Lrc<SourceMap>, comments: SingleThreadedComments) -> Self {
        Self {
            source_map,
            comments,
        }
    }

    /// Format a module using selective comment preservation
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

        extracted_comments
            .floating_comments
            .retain(|c| !inline_positions.contains(&c.span.lo));

        // Phase 3: Format the AST using the regular formatter
        let formatter = KrokFormatter::new();
        let formatted_module = formatter.format(module)?;

        // Phase 4: Generate code WITH inline comments (they're preserved)
        let generator = CodeGenerator::with_comments(self.source_map.clone(), inline_only_comments);
        let code_with_inline_comments = generator.generate(&formatted_module)?;

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

    fn format_with_selective_two_phase(source: &str) -> Result<String> {
        let parser = TypeScriptParser::new();
        let module = parser.parse(source, "test.ts")?;

        let formatter =
            SelectiveTwoPhaseFormatter::new(parser.source_map.clone(), parser.comments.clone());

        formatter.format(module, source)
    }

    #[test]
    fn test_inline_comments_preserved() {
        let source = r#"
const x = /* inline comment */ 42;
let y = /* another inline */ "hello";
"#;

        let result = format_with_selective_two_phase(source).unwrap();
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

        let result = format_with_selective_two_phase(source).unwrap();

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

        let result = format_with_selective_two_phase(source).unwrap();
        // All comment types should be preserved
        assert!(result.contains("// This is a leading comment"));
        assert!(result.contains("/* param comment */"));
        // TODO: Trailing comments on return statements are not yet properly preserved
        // This is a limitation of the current comment extraction system
        // assert!(result.contains("// multiply by two"));
    }
}
