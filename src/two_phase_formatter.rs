use anyhow::Result;
use swc_common::{comments::SingleThreadedComments, sync::Lrc, SourceMap};
use swc_ecma_ast::Module;

use crate::{
    codegen::CodeGenerator, comment_extractor::CommentExtractor,
    comment_reinserter::CommentReinserter, formatter::KrokFormatter,
};

/// Two-phase formatter that preserves comment positions correctly
///
/// This formatter works around SWC's comment attachment issues by:
/// 1. Extracting comments with semantic hashes
/// 2. Formatting the AST without comments
/// 3. Generating code without comments
/// 4. Re-parsing to find node positions
/// 5. Reinserting comments at the correct positions
pub struct TwoPhaseFormatter {
    source_map: Lrc<SourceMap>,
    comments: SingleThreadedComments,
}

impl TwoPhaseFormatter {
    pub fn new(source_map: Lrc<SourceMap>, comments: SingleThreadedComments) -> Self {
        Self {
            source_map,
            comments,
        }
    }

    /// Format a module using the two-phase approach
    pub fn format(&self, module: Module) -> Result<String> {
        self.format_with_source(module, String::new())
    }

    /// Format a module using the two-phase approach with source code for smart comment extraction
    pub fn format_with_source(&self, module: Module, source: String) -> Result<String> {
        // Phase 1: Extract comments with semantic hashes
        let extractor = if source.is_empty() {
            CommentExtractor::new(&self.comments)
        } else {
            CommentExtractor::with_source(&self.comments, source)
        };
        let extracted_comments = extractor.extract(&module);

        // Phase 2: Format the AST using the regular formatter
        let formatter = KrokFormatter::new();
        let formatted_module = formatter.format(module)?;

        // Phase 3: Generate code without comments
        let generator = CodeGenerator::new(self.source_map.clone());
        let code_without_comments = generator.generate_without_comments(&formatted_module)?;

        // Phase 4: Reinsert comments at the correct positions
        let mut reinserter = CommentReinserter::new(extracted_comments);
        let final_code = reinserter.reinsert_comments(&code_without_comments)?;

        Ok(final_code)
    }

    /// Check if the module has any comments that would benefit from two-phase formatting
    pub fn should_use_two_phase(_module: &Module) -> bool {
        // For now, always use two-phase if there are comments
        // In the future, we could be smarter about detecting when it's needed
        true
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::parser::TypeScriptParser;

    fn format_with_two_phase(source: &str) -> Result<String> {
        let parser = TypeScriptParser::new();
        let module = parser.parse(source, "test.ts")?;

        let formatter = TwoPhaseFormatter::new(parser.source_map.clone(), parser.comments.clone());

        formatter.format(module)
    }

    #[test]
    fn test_two_phase_basic() {
        let source = r#"
// This is a comment
function foo() {
    return 42;
}
"#;

        let result = format_with_two_phase(source).unwrap();

        // Should contain the function
        assert!(result.contains("function foo()"));

        // Comment extraction and reinsertion is working (even if positioning is simplified)
        assert!(result.contains("// This is a comment"));
    }

    #[test]
    fn test_two_phase_multiple_comments() {
        let source = r#"
// First comment
// Second comment
function foo() {
    return 42;
}
"#;

        let result = format_with_two_phase(source).unwrap();

        // Should preserve multiple comments
        assert!(result.contains("// First comment"));
        assert!(result.contains("// Second comment"));
        assert!(result.contains("function foo()"));
    }

    #[test]
    fn test_two_phase_preserves_content() {
        let source = r#"
/* Block comment */
const x = 42;
"#;

        let result = format_with_two_phase(source).unwrap();

        // Should preserve block comments
        assert!(result.contains("/* Block comment */"));
        assert!(result.contains("const x = 42"));
    }
}
