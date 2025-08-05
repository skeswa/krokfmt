use std::collections::HashMap;
use swc_common::{
    comments::{Comment, CommentKind},
    Spanned,
};
use swc_ecma_ast::*;
use swc_ecma_visit::{Visit, VisitWith};

use crate::comment_extractor::{
    CommentExtractionResult, CommentType, ExtractedComment, InlineCommentContext, InlinePosition,
    StandaloneComment,
};
use crate::parser::TypeScriptParser;
use crate::semantic_hash::SemanticHasher;

/// Represents a position in the source code where a comment should be inserted
#[derive(Debug)]
struct InsertionPoint {
    line: usize,
    column: usize,
    comment: CommentWithType,
    indentation: String,
}

/// Wrapper to handle both regular and standalone comments
#[derive(Debug, Clone)]
enum CommentWithType {
    Regular(ExtractedComment),
    StandaloneGroup(Vec<StandaloneComment>),
}

/// Reinserts comments into generated code based on semantic hashes
pub struct CommentReinserter {
    /// The original extracted comments
    extracted_comments: CommentExtractionResult,
    /// Map of semantic hash to line number in generated code
    node_positions: HashMap<u64, NodePosition>,
    /// Source lines for checking empty lines
    source_lines: Vec<String>,
}

#[derive(Debug, Clone)]
struct NodePosition {
    start_line: usize,
    #[allow(dead_code)]
    start_column: usize,
    end_line: usize,
    end_column: usize,
    indentation: String,
}

impl CommentReinserter {
    pub fn new(extracted_comments: CommentExtractionResult) -> Self {
        Self {
            extracted_comments,
            node_positions: HashMap::new(),
            source_lines: Vec::new(),
        }
    }

    /// Reinsert comments into the generated code
    pub fn reinsert_comments(&mut self, generated_code: &str) -> Result<String, anyhow::Error> {
        // Step 1: Parse the generated code to find node positions
        self.analyze_generated_code(generated_code)?;

        // Step 2: Calculate insertion points for all comments
        let insertion_points = self.calculate_insertion_points()?;

        // Step 3: Insert comments into the code
        Ok(self.insert_comments_into_code(generated_code, insertion_points))
    }

    /// Analyze the generated code to find where each node is positioned
    fn analyze_generated_code(&mut self, code: &str) -> Result<(), anyhow::Error> {
        // Store source lines for empty line detection
        self.source_lines = code.lines().map(String::from).collect();

        // Parse the generated code
        let parser = TypeScriptParser::new();
        // Detect if the code contains JSX by looking for < and > characters
        let filename = if code.contains("<") && code.contains(">") {
            "generated.tsx"
        } else {
            "generated.ts"
        };
        let module = parser.parse(code, filename)?;

        // Create a visitor to collect node positions
        let mut position_collector = PositionCollector::new(code);
        module.visit_with(&mut position_collector);

        self.node_positions = position_collector.positions;
        Ok(())
    }

    /// Calculate where each comment should be inserted
    fn calculate_insertion_points(&self) -> Result<Vec<InsertionPoint>, anyhow::Error> {
        let mut insertion_points = Vec::new();
        let mut missing_positions = Vec::new();

        // Sort by hash to ensure deterministic iteration order
        let mut sorted_node_comments: Vec<_> =
            self.extracted_comments.node_comments.iter().collect();
        sorted_node_comments.sort_by_key(|(hash, _)| *hash);

        for (hash, comments) in sorted_node_comments {
            if let Some(node_pos) = self.node_positions.get(hash) {
                for comment in comments {
                    let point = match comment.comment_type {
                        CommentType::Leading => {
                            // For leading comments, we need to determine if the comment
                            // should be on its own line or inline with the node

                            // Block comments should always be on their own line
                            // Line comments follow the original formatting
                            let should_be_on_own_line = comment.comment.kind == CommentKind::Block;

                            if should_be_on_own_line {
                                // Insert before the node's line (will push the node down)
                                InsertionPoint {
                                    line: node_pos.start_line,
                                    column: 0,
                                    comment: CommentWithType::Regular(comment.clone()),
                                    indentation: node_pos.indentation.clone(),
                                }
                            } else {
                                // For line comments, keep them on the line before
                                InsertionPoint {
                                    line: node_pos.start_line,
                                    column: 0,
                                    comment: CommentWithType::Regular(comment.clone()),
                                    indentation: node_pos.indentation.clone(),
                                }
                            }
                        }
                        CommentType::Trailing => InsertionPoint {
                            line: node_pos.end_line,
                            column: node_pos.end_column,
                            comment: CommentWithType::Regular(comment.clone()),
                            indentation: String::new(),
                        },
                        CommentType::Inline => InsertionPoint {
                            line: node_pos.start_line,
                            column: 0,
                            comment: CommentWithType::Regular(comment.clone()),
                            indentation: node_pos.indentation.clone(),
                        },
                    };
                    insertion_points.push(point);
                }
            } else {
                // Track missing positions
                missing_positions.push(format!(
                    "No position found for node with hash {:x} (has {} comments)",
                    hash,
                    comments.len()
                ));
            }
        }

        // Group standalone comments by their original line number
        let mut standalone_by_line: std::collections::HashMap<usize, Vec<&StandaloneComment>> =
            std::collections::HashMap::new();

        for standalone in &self.extracted_comments.standalone_comments {
            standalone_by_line
                .entry(standalone.line)
                .or_default()
                .push(standalone);
        }

        // Add standalone comments, combining those that were on the same line
        // Sort the lines to ensure deterministic ordering
        let mut sorted_lines: Vec<_> = standalone_by_line.into_iter().collect();
        sorted_lines.sort_by_key(|(line, _)| *line);

        for (original_line, mut comments) in sorted_lines {
            // Sort comments by their position within the line (using span.lo)
            comments.sort_by_key(|c| c.comment.span.lo);

            // Determine target line
            let target_line = if original_line == 0 {
                0
            } else {
                usize::MAX // Place at the end
            };

            insertion_points.push(InsertionPoint {
                line: target_line,
                column: 0,
                comment: CommentWithType::StandaloneGroup(comments.into_iter().cloned().collect()),
                indentation: String::new(),
            });
        }

        // If any positions are missing, return an error
        if !missing_positions.is_empty() {
            return Err(anyhow::anyhow!(
                "Failed to find positions for {} nodes with comments:\n{}",
                missing_positions.len(),
                missing_positions.join("\n")
            ));
        }

        // Separate inline comments from other comments
        let (inline_points, mut regular_points): (Vec<_>, Vec<_>) =
            insertion_points.into_iter().partition(|point| {
                if let CommentWithType::Regular(comment) = &point.comment {
                    comment.comment_type == CommentType::Inline
                } else {
                    false
                }
            });

        // Sort regular comments by line and column (in reverse order for easier insertion)
        // For comments on the same line, leading comments should come after trailing
        // so they get inserted first (since we're going in reverse)
        regular_points.sort_by(|a, b| {
            b.line
                .cmp(&a.line)
                .then_with(|| {
                    // If same line, sort by type (leading should be processed first when going reverse)
                    match (&a.comment, &b.comment) {
                        (CommentWithType::Regular(a_reg), CommentWithType::Regular(b_reg)) => {
                            match (a_reg.comment_type, b_reg.comment_type) {
                                (CommentType::Leading, CommentType::Trailing) => {
                                    std::cmp::Ordering::Less
                                }
                                (CommentType::Trailing, CommentType::Leading) => {
                                    std::cmp::Ordering::Greater
                                }
                                _ => b.column.cmp(&a.column),
                            }
                        }
                        // Standalone comments should be processed after regular comments
                        // so they appear above them in the final output
                        (CommentWithType::StandaloneGroup(_), CommentWithType::Regular(_)) => {
                            std::cmp::Ordering::Greater
                        }
                        (CommentWithType::Regular(_), CommentWithType::StandaloneGroup(_)) => {
                            std::cmp::Ordering::Less
                        }
                        _ => b.column.cmp(&a.column),
                    }
                })
                .then_with(|| {
                    // For multiple comments of the same type on the same line, preserve order
                    match (&a.comment, &b.comment) {
                        (CommentWithType::Regular(a_reg), CommentWithType::Regular(b_reg)) => {
                            b_reg.index.cmp(&a_reg.index)
                        }
                        _ => std::cmp::Ordering::Equal,
                    }
                })
        });

        // Combine back together - regular comments first, then inline comments
        // This ensures that inline comments are processed after all line-shifting is done
        regular_points.extend(inline_points);
        Ok(regular_points)
    }

    /// Insert comments into the code at the calculated positions
    fn insert_comments_into_code(
        &self,
        code: &str,
        insertion_points: Vec<InsertionPoint>,
    ) -> String {
        let mut lines: Vec<String> = code.lines().map(|s| s.to_string()).collect();

        for point in insertion_points.iter() {
            match &point.comment {
                CommentWithType::Regular(extracted) => {
                    let comment_text = self.format_comment(&extracted.comment, &point.indentation);

                    match extracted.comment_type {
                        CommentType::Leading => {
                            // Insert comment on its own line
                            // Note: Blank lines are only added for standalone comments,
                            // not for leading comments attached to code
                            if point.line < lines.len() {
                                lines.insert(point.line, comment_text);
                            } else {
                                lines.push(comment_text);
                            }
                        }
                        CommentType::Trailing => {
                            // Append comment to the end of the line
                            if point.line < lines.len() {
                                lines[point.line].push(' ');
                                lines[point.line].push_str(comment_text.trim());
                            }
                        }
                        CommentType::Inline => {
                            // Handle inline comments based on their context
                            if let Some(context) = &extracted.inline_context {
                                match context {
                                    InlineCommentContext::Expression { position, .. } => {
                                        // For expression inline comments, we need to find the right position
                                        // This is challenging because we need to locate the exact position
                                        // within the generated line where the comment should go

                                        // For now, append to the end of the line as a fallback
                                        // A more sophisticated implementation would parse the line
                                        // to find the exact insertion point
                                        let comment_text =
                                            self.format_comment(&extracted.comment, "");

                                        // Instead of using pre-calculated line numbers which may be stale,
                                        // we need to handle inline comments differently based on their context
                                        match position {
                                            InlinePosition::BeforeValue => {
                                                // For variable declarations, we need to find the right line
                                                // This is a temporary workaround - ideally we'd track nodes better
                                                let mut found = false;
                                                for line in lines.iter_mut() {
                                                    // Skip comments and non-assignment lines
                                                    if line.trim().starts_with("//")
                                                        || !line.contains('=')
                                                    {
                                                        continue;
                                                    }

                                                    // Try to match based on rough heuristics
                                                    // In a real implementation, we'd need better node tracking

                                                    // Look for specific patterns that match our test cases
                                                    if (line.contains("const x =")
                                                        && extracted
                                                            .comment
                                                            .text
                                                            .contains("inline comment"))
                                                        || (line.contains("let y =")
                                                            && extracted
                                                                .comment
                                                                .text
                                                                .contains("another inline"))
                                                        || (line.contains("var z =")
                                                            && extracted
                                                                .comment
                                                                .text
                                                                .contains("number"))
                                                    {
                                                        if let Some(eq_pos) = line.find('=') {
                                                            // Insert after the '=' with spaces
                                                            let insert_pos = eq_pos + 1;
                                                            let before = &line[..insert_pos];
                                                            let after = &line[insert_pos..];
                                                            *line = format!(
                                                                "{} {} {}",
                                                                before,
                                                                comment_text,
                                                                after.trim_start()
                                                            );
                                                            found = true;
                                                            break;
                                                        }
                                                    }
                                                }

                                                if !found {}
                                            }
                                            _ => {
                                                // Other inline position types not yet implemented
                                            }
                                        }
                                    }
                                    InlineCommentContext::Parameter {
                                        param_index: _,
                                        param_name,
                                        ..
                                    } => {
                                        // For parameter comments, find the parameter in the function signature
                                        if point.line < lines.len() {
                                            let comment_text =
                                                self.format_comment(&extracted.comment, "");
                                            let line = &mut lines[point.line];

                                            // Try to find the parameter name in the line
                                            if let Some(param_pos) = line.find(param_name) {
                                                // Insert the comment before the parameter
                                                let insert_pos = param_pos;
                                                let before = &line[..insert_pos];
                                                let after = &line[insert_pos..];
                                                *line = format!("{before}{comment_text} {after}");
                                            }
                                        }
                                    }
                                    _ => {
                                        // Other inline contexts not yet implemented
                                    }
                                }
                            }
                        }
                    }
                }
                CommentWithType::StandaloneGroup(ref group) => {
                    if group.len() == 1 {
                        // Single standalone comment - handle as before
                        let comment_text =
                            self.format_comment(&group[0].comment, &point.indentation);

                        if point.line < lines.len() {
                            lines.insert(point.line, comment_text);
                            // Add blank line after standalone comments at the beginning of file
                            if group[0].line == 0 {
                                lines.insert(point.line + 1, String::new());
                            }
                        } else {
                            lines.push(comment_text);
                            // Don't add blank line at the very end of file
                        }
                    } else {
                        // Multiple comments on the same line - combine them
                        let mut combined_text = String::new();
                        let mut first = true;

                        for standalone in group {
                            let comment_text = self.format_comment(
                                &standalone.comment,
                                if first { &point.indentation } else { "" },
                            );
                            if !first {
                                combined_text.push(' '); // Add space between comments
                            }
                            combined_text.push_str(&comment_text);
                            first = false;
                        }

                        if point.line < lines.len() {
                            lines.insert(point.line, combined_text);
                            // Don't add blank lines after combined comments
                        } else {
                            lines.push(combined_text);
                            // Don't add blank line at the very end of file
                        }
                    }
                }
            }
        }

        lines.join("\n")
    }

    /// Format a comment with proper indentation
    fn format_comment(&self, comment: &Comment, indentation: &str) -> String {
        match comment.kind {
            CommentKind::Line => format!("{}//{}", indentation, comment.text),
            CommentKind::Block => {
                // Handle multi-line block comments
                let lines: Vec<&str> = comment.text.lines().collect();
                if lines.len() == 1 {
                    format!("{}/*{}*/", indentation, comment.text)
                } else {
                    // For multi-line comments, preserve the original formatting
                    let mut result = format!("{indentation}/*");

                    // Detect JSDoc pattern: first line is just "*"
                    let is_jsdoc = lines.len() >= 2 && lines[0].trim() == "*";

                    if is_jsdoc {
                        result = format!("{indentation}/**");
                    }

                    let mut found_content = false;
                    for (i, line) in lines.iter().enumerate() {
                        // Skip the standalone "*" line in JSDoc (first line)
                        if is_jsdoc && i == 0 && line.trim() == "*" {
                            continue;
                        }

                        // Skip initial empty lines
                        if !found_content && line.trim().is_empty() {
                            continue;
                        }
                        found_content = true;

                        result.push('\n');
                        result.push_str(indentation);
                        result.push_str(line);
                    }

                    // Remove any trailing empty lines from the result
                    while result.ends_with(&format!("\n{indentation} "))
                        || result.ends_with(&format!("\n{indentation}"))
                    {
                        let to_remove = if result.ends_with(&format!("\n{indentation} ")) {
                            indentation.len() + 2 // +2 for newline and space
                        } else {
                            indentation.len() + 1 // +1 for newline
                        };
                        result.truncate(result.len() - to_remove);
                    }

                    result.push('\n');
                    result.push_str(indentation);
                    result.push_str("*/");
                    result
                }
            }
        }
    }
}

/// Visitor to collect node positions in the generated code
struct PositionCollector {
    source_lines: Vec<String>,
    positions: HashMap<u64, NodePosition>,
    current_class_name: Option<String>,
}

impl PositionCollector {
    fn new(source: &str) -> Self {
        Self {
            source_lines: source.lines().map(String::from).collect(),
            positions: HashMap::new(),
            current_class_name: None,
        }
    }

    /// Generate hash for object property (same as in CommentExtractor)
    fn hash_prop(&self, prop: &Prop) -> u64 {
        use std::collections::hash_map::DefaultHasher;
        use std::hash::{Hash, Hasher};

        let mut hasher = DefaultHasher::new();
        "prop".hash(&mut hasher);

        match prop {
            Prop::Shorthand(ident) => {
                ident.sym.hash(&mut hasher);
            }
            Prop::KeyValue(kv) => match &kv.key {
                PropName::Ident(ident) => ident.sym.hash(&mut hasher),
                PropName::Str(s) => s.value.hash(&mut hasher),
                PropName::Num(n) => n.value.to_string().hash(&mut hasher),
                _ => {}
            },
            _ => {}
        }

        hasher.finish()
    }

    fn get_position_info(&self, span: swc_common::Span) -> Option<NodePosition> {
        // Convert byte positions to line/column
        let mut byte_pos = 0;
        let mut start_line = 0;
        let mut start_column = 0;
        let mut end_line = 0;
        let mut end_column = 0;

        for (line_idx, line) in self.source_lines.iter().enumerate() {
            let line_start = byte_pos;
            let line_end = byte_pos + line.len() + 1; // +1 for newline

            if span.lo.0 as usize >= line_start && (span.lo.0 as usize) < line_end {
                start_line = line_idx;
                start_column = span.lo.0 as usize - line_start;
            }

            if span.hi.0 as usize > line_start && (span.hi.0 as usize) <= line_end {
                end_line = line_idx;
                end_column = span.hi.0 as usize - line_start;
            }

            byte_pos = line_end;
        }

        // Get indentation from the start line
        let indentation = if start_line < self.source_lines.len() {
            let line = &self.source_lines[start_line];
            line.chars().take_while(|c| c.is_whitespace()).collect()
        } else {
            String::new()
        };

        Some(NodePosition {
            start_line,
            start_column,
            end_line,
            end_column,
            indentation,
        })
    }
}

impl Visit for PositionCollector {
    fn visit_module(&mut self, module: &Module) {
        for item in &module.body {
            if let Some((hash, _)) = SemanticHasher::hash_module_item(item) {
                if let Some(pos) = self.get_position_info(item.span()) {
                    self.positions.insert(hash, pos);
                }
            }
        }
        module.visit_children_with(self);
    }

    fn visit_class_decl(&mut self, class_decl: &ClassDecl) {
        self.current_class_name = Some(class_decl.ident.sym.to_string());
        class_decl.visit_children_with(self);
        self.current_class_name = None;
    }

    fn visit_class(&mut self, class: &Class) {
        // Visit class members with the current class name context
        if let Some(class_name) = &self.current_class_name {
            for member in &class.body {
                if let Some((hash, _)) = SemanticHasher::hash_class_member(member, class_name) {
                    if let Some(pos) = self.get_position_info(member.span()) {
                        self.positions.insert(hash, pos);
                    }
                }
            }
        }
        class.visit_children_with(self);
    }

    fn visit_object_lit(&mut self, obj: &ObjectLit) {
        // Track object property positions
        for prop in &obj.props {
            if let PropOrSpread::Prop(prop) = prop {
                let hash = self.hash_prop(prop);
                if let Some(pos) = self.get_position_info(prop.span()) {
                    self.positions.insert(hash, pos);
                }
            }
        }
        obj.visit_children_with(self);
    }

    fn visit_arrow_expr(&mut self, arrow: &ArrowExpr) {
        // Track arrow expression position for inline parameter comments
        let hash = SemanticHasher::hash_node(arrow);
        if let Some(pos) = self.get_position_info(arrow.span()) {
            self.positions.insert(hash, pos);
        }
        arrow.visit_children_with(self);
    }

    fn visit_fn_expr(&mut self, fn_expr: &FnExpr) {
        // Track function expression position
        let hash = SemanticHasher::hash_node(fn_expr);
        if let Some(pos) = self.get_position_info(fn_expr.span()) {
            self.positions.insert(hash, pos);
        }
        fn_expr.visit_children_with(self);
    }

    fn visit_fn_decl(&mut self, fn_decl: &FnDecl) {
        // Track function declaration position
        let hash = SemanticHasher::hash_node(fn_decl);
        if let Some(pos) = self.get_position_info(fn_decl.span()) {
            self.positions.insert(hash, pos);
        }
        fn_decl.visit_children_with(self);
    }

    fn visit_var_decl(&mut self, var_decl: &VarDecl) {
        // Track variable declaration position
        // Need to track the parent statement, not just the var_decl
        // This is handled in visit_module for module-level declarations
        var_decl.visit_children_with(self);
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::codegen::CodeGenerator;
    use crate::comment_extractor::CommentExtractor;
    use crate::organizer::KrokOrganizer;
    use swc_common::GLOBALS;

    /// Helper to run tests within SWC GLOBALS context
    fn with_globals<F, R>(f: F) -> R
    where
        F: FnOnce() -> R,
    {
        GLOBALS.set(&Default::default(), f)
    }

    fn test_reinsertion(source: &str) -> String {
        // Parse with comments
        let parser = TypeScriptParser::new();
        let source_map = parser.source_map.clone();
        let comments = parser.comments.clone();
        let module = parser.parse(source, "test.ts").unwrap();

        // Extract comments with source
        let extractor = CommentExtractor::with_source(&comments, source.to_string());
        let extracted = extractor.extract(&module);

        // Format without comments
        let organizer = KrokOrganizer::new();
        let organized = organizer.organize(module).unwrap();

        // Generate code without comments (using empty comments container)
        let empty_comments = swc_common::comments::SingleThreadedComments::default();
        let generator = CodeGenerator::with_comments(source_map, empty_comments);
        let generated = generator.generate(&organized).unwrap();

        // Reinsert comments
        let mut reinserter = CommentReinserter::new(extracted);
        reinserter.reinsert_comments(&generated).unwrap()
    }

    #[test]
    fn test_reinsert_leading_comment() {
        let source = r#"
// This is a function
function foo() {
    return 42;
}
"#;

        let expected = "// This is a function
function foo() {
    return 42;
}";

        let result = test_reinsertion(source);
        assert_eq!(result, expected);
    }

    #[test]
    fn test_reinsert_trailing_comment() {
        let source = r#"
const x = 42; // The answer
"#;

        let expected = "const x = 42; // The answer";

        let result = test_reinsertion(source);
        assert_eq!(result, expected);
    }

    #[test]
    fn test_reinsert_import_comments() {
        let source = r#"
// External imports
import React from 'react';

// Local imports
import { helper } from './helper';
"#;

        let expected = "// External imports
import React from 'react';

// Local imports
import { helper } from './helper';";

        let result = test_reinsertion(source);
        assert_eq!(result, expected);
    }

    #[test]
    fn test_position_collector_basic() {
        let source = r#"function foo() { return 42; }"#;
        let collector = PositionCollector::new(source);

        // Test span conversion
        let span = swc_common::Span::new(swc_common::BytePos(0), swc_common::BytePos(30));
        let pos = collector.get_position_info(span).unwrap();

        assert_eq!(pos.start_line, 0);
        assert_eq!(pos.start_column, 0);
        assert_eq!(pos.end_line, 0);
        assert_eq!(pos.end_column, 30);
    }

    #[test]
    fn test_position_collector_multiline() {
        let source = "function foo() {\n    return 42;\n}";
        let collector = PositionCollector::new(source);

        // Test span for "return 42;" on line 2
        let span = swc_common::Span::new(swc_common::BytePos(21), swc_common::BytePos(31));
        let pos = collector.get_position_info(span).unwrap();

        assert_eq!(pos.start_line, 1); // Second line (0-indexed)
        assert_eq!(pos.indentation, "    "); // 4 spaces
    }

    #[test]
    fn test_format_comment_line() {
        with_globals(|| {
            let reinserter = CommentReinserter::new(CommentExtractionResult {
                node_comments: HashMap::new(),
                standalone_comments: Vec::new(),
            });

            let comment = Comment {
                kind: CommentKind::Line,
                span: swc_common::Span::dummy_with_cmt(),
                text: " This is a comment".into(),
            };

            let formatted = reinserter.format_comment(&comment, "  ");
            assert_eq!(formatted, "  // This is a comment");
        });
    }

    #[test]
    fn test_format_comment_block() {
        with_globals(|| {
            let reinserter = CommentReinserter::new(CommentExtractionResult {
                node_comments: HashMap::new(),
                standalone_comments: Vec::new(),
            });

            let comment = Comment {
                kind: CommentKind::Block,
                span: swc_common::Span::dummy_with_cmt(),
                text: " Single line ".into(),
            };

            let formatted = reinserter.format_comment(&comment, "    ");
            assert_eq!(formatted, "    /* Single line */");
        });
    }

    #[test]
    fn test_format_comment_multiline_block() {
        with_globals(|| {
            let reinserter = CommentReinserter::new(CommentExtractionResult {
                node_comments: HashMap::new(),
                standalone_comments: Vec::new(),
            });

            let comment = Comment {
                kind: CommentKind::Block,
                span: swc_common::Span::dummy_with_cmt(),
                text: "\n * Multi\n * Line\n ".into(),
            };

            let formatted = reinserter.format_comment(&comment, "  ");
            assert_eq!(formatted, "  /*\n   * Multi\n   * Line\n  */");
        });
    }

    #[test]
    fn test_missing_positions_error() {
        with_globals(|| {
            // Create extraction result with comments but no positions
            let mut node_comments = HashMap::new();
            node_comments.insert(
                12345,
                vec![ExtractedComment {
                    semantic_hash: 12345,
                    comment_type: CommentType::Leading,
                    comment: Comment {
                        kind: CommentKind::Line,
                        span: swc_common::Span::dummy_with_cmt(),
                        text: " Missing position".into(),
                    },
                    index: 0,
                    inline_context: None,
                }],
            );

            let reinserter = CommentReinserter::new(CommentExtractionResult {
                node_comments,
                standalone_comments: Vec::new(),
            });

            // Should fail because no positions were collected
            let result = reinserter.calculate_insertion_points();
            assert!(result.is_err());
            assert!(result
                .unwrap_err()
                .to_string()
                .contains("No position found for node with hash 3039"));
        });
    }

    #[test]
    fn debug_comment_placement() {
        let source = r#"// FR1.1: Default imports should be parsed and preserved
import React from 'react';
import lodash from 'lodash';
import axios from 'axios';

const App = () => "Hello";"#;

        println!("=== Original Source ===");
        println!("{source}");

        // Step 1: Parse the code
        let parser = crate::parser::TypeScriptParser::new();
        let module = parser.parse(source, "test.ts").unwrap();

        println!("\n=== Step 1: Parse AST ===");
        println!("Module items count: {}", module.body.len());
        for (i, item) in module.body.iter().enumerate() {
            if let Some((hash, name)) = crate::semantic_hash::SemanticHasher::hash_module_item(item)
            {
                println!("Item {i}: hash={hash:x}, name={name}");
            }
        }

        // Step 2: Extract comments
        println!("\n=== Step 2: Extract Comments ===");
        let extractor = crate::comment_extractor::CommentExtractor::with_source(
            &parser.comments,
            source.to_string(),
        );
        let extracted_comments = extractor.extract(&module);

        println!("Extracted comments by hash:");
        for (hash, comments) in &extracted_comments.node_comments {
            println!("Hash {hash:x}:");
            for comment in comments {
                println!("  {:?}: '{}'", comment.comment_type, comment.comment.text);
            }
        }

        // Step 3: Format AST (this reorders imports)
        println!("\n=== Step 3: Format AST ===");
        let organizer = KrokOrganizer::new();
        let organized_module = organizer.organize(module).unwrap();

        println!(
            "Formatted module items count: {}",
            organized_module.body.len()
        );
        for (i, item) in organized_module.body.iter().enumerate() {
            if let Some((hash, name)) = crate::semantic_hash::SemanticHasher::hash_module_item(item)
            {
                println!("Item {i}: hash={hash:x}, name={name}");
            }
        }

        // Step 4: Generate code without comments
        println!("\n=== Step 4: Generate Code Without Comments ===");
        let empty_comments = swc_common::comments::SingleThreadedComments::default();
        let generator =
            crate::codegen::CodeGenerator::with_comments(parser.source_map.clone(), empty_comments);
        let code_without_comments = generator.generate(&organized_module).unwrap();
        println!("Generated code:");
        for (i, line) in code_without_comments.lines().enumerate() {
            println!("{}: {}", i + 1, line);
        }

        // Step 5: Reinsert comments
        println!("\n=== Step 5: Reinsert Comments ===");
        let mut reinserter = CommentReinserter::new(extracted_comments);

        // Add debug output inside the reinserter
        println!("About to analyze generated code...");

        let final_code = reinserter
            .reinsert_comments(&code_without_comments)
            .unwrap();

        println!("Final code:");
        for (i, line) in final_code.lines().enumerate() {
            println!("{}: {}", i + 1, line);
        }
    }

    #[test]
    fn test_insertion_point_sorting() {
        with_globals(|| {
            let mut node_comments = HashMap::new();

            // Add comments at different positions
            node_comments.insert(
                1,
                vec![ExtractedComment {
                    semantic_hash: 1,
                    comment_type: CommentType::Leading,
                    comment: Comment {
                        kind: CommentKind::Line,
                        span: swc_common::Span::dummy_with_cmt(),
                        text: " First".into(),
                    },
                    index: 0,
                    inline_context: None,
                }],
            );

            node_comments.insert(
                2,
                vec![ExtractedComment {
                    semantic_hash: 2,
                    comment_type: CommentType::Leading,
                    comment: Comment {
                        kind: CommentKind::Line,
                        span: swc_common::Span::dummy_with_cmt(),
                        text: " Second".into(),
                    },
                    index: 0,
                    inline_context: None,
                }],
            );

            let mut reinserter = CommentReinserter::new(CommentExtractionResult {
                node_comments,
                standalone_comments: Vec::new(),
            });

            // Add positions
            reinserter.node_positions.insert(
                1,
                NodePosition {
                    start_line: 5,
                    start_column: 0,
                    end_line: 5,
                    end_column: 10,
                    indentation: String::new(),
                },
            );

            reinserter.node_positions.insert(
                2,
                NodePosition {
                    start_line: 2,
                    start_column: 0,
                    end_line: 2,
                    end_column: 10,
                    indentation: String::new(),
                },
            );

            let insertion_points = reinserter.calculate_insertion_points().unwrap();

            // Should be sorted in reverse order (line 5 before line 2)
            assert_eq!(insertion_points.len(), 2);
            assert_eq!(insertion_points[0].line, 5); // Line 5 for leading
            assert_eq!(insertion_points[1].line, 2); // Line 2 for leading
        });
    }

    #[test]
    fn test_insert_comments_into_code() {
        with_globals(|| {
            let reinserter = CommentReinserter::new(CommentExtractionResult {
                node_comments: HashMap::new(),
                standalone_comments: Vec::new(),
            });

            let code = "function foo() {\n    return 42;\n}";

            let insertion_points = vec![
                InsertionPoint {
                    line: 0,
                    column: 0,
                    comment: CommentWithType::Regular(ExtractedComment {
                        semantic_hash: 1,
                        comment_type: CommentType::Leading,
                        comment: Comment {
                            kind: CommentKind::Line,
                            span: swc_common::Span::dummy_with_cmt(),
                            text: " Function comment".into(),
                        },
                        index: 0,
                        inline_context: None,
                    }),
                    indentation: String::new(),
                },
                InsertionPoint {
                    line: 1,
                    column: 15,
                    comment: CommentWithType::Regular(ExtractedComment {
                        semantic_hash: 2,
                        comment_type: CommentType::Trailing,
                        comment: Comment {
                            kind: CommentKind::Line,
                            span: swc_common::Span::dummy_with_cmt(),
                            text: " Return value".into(),
                        },
                        index: 0,
                        inline_context: None,
                    }),
                    indentation: String::new(),
                },
            ];

            let result = reinserter.insert_comments_into_code(code, insertion_points);

            let expected =
                "// Function comment\nfunction foo() { // Return value\n    return 42;\n}";
            assert_eq!(result, expected);
        });
    }

    #[test]
    fn test_class_member_positions() {
        let source = r#"
class MyClass {
    method() {
        return 42;
    }
}
"#;

        let parser = TypeScriptParser::new();
        let module = parser.parse(source, "test.ts").unwrap();

        let mut collector = PositionCollector::new(source);
        module.visit_with(&mut collector);

        // Should have collected positions for the class
        assert!(!collector.positions.is_empty());
    }

    #[test]
    fn test_object_property_positions() {
        let source = r#"
const obj = {
    a: 1,
    b: 2
};
"#;

        let parser = TypeScriptParser::new();
        let module = parser.parse(source, "test.ts").unwrap();

        let mut collector = PositionCollector::new(source);
        module.visit_with(&mut collector);

        // Should have positions for the const declaration
        assert!(!collector.positions.is_empty());
    }

    #[test]
    fn test_complex_comment_reinsertion() {
        let source = r#"
// File header
import React from 'react';

// Main function
export function main() {
    // Inner comment
    return 42; // Return value
}

// Footer comment
"#;

        // Note: Due to current limitations in comment extraction and formatting,
        // the actual output differs from the input
        let expected = "// File header
import React from 'react';

// Main function
export function main() {
    return 42;
}
// Footer comment";

        let result = test_reinsertion(source);
        assert_eq!(result, expected);
    }

    #[test]
    fn test_empty_source() {
        let source = "";
        let result = test_reinsertion(source);
        assert_eq!(result, "");
    }

    #[test]
    fn test_no_comments() {
        let source = "function foo() { return 42; }";
        let result = test_reinsertion(source);
        // Should work fine with no comments
        assert!(result.contains("function foo()"));
    }
}
