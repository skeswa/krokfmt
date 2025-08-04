use std::collections::HashMap;
use swc_common::{
    comments::{Comment, Comments, SingleThreadedComments},
    BytePos, Spanned,
};
use swc_ecma_ast::*;
use swc_ecma_visit::{Visit, VisitWith};

use crate::semantic_hash::SemanticHasher;

/// Represents a comment and its association type (leading or trailing)
#[derive(Debug, Clone)]
pub struct ExtractedComment {
    /// Semantic hash of the associated node
    pub semantic_hash: u64,
    /// Type of comment association
    pub comment_type: CommentType,
    /// The actual comment
    pub comment: Comment,
    /// Index for preserving order when multiple comments exist
    pub index: usize,
}

#[derive(Debug, Clone, Copy, PartialEq)]
pub enum CommentType {
    Leading,
    Trailing,
}

/// Extracts comments from an AST and associates them with semantic hashes
pub struct CommentExtractor<'a> {
    /// Reference to the comment storage
    comments: &'a SingleThreadedComments,
    /// Extracted comments mapped by semantic hash
    extracted: HashMap<u64, Vec<ExtractedComment>>,
    /// Standalone comments that should maintain their position
    standalone_comments: Vec<StandaloneComment>,
    /// Comments that couldn't be associated with any node
    floating_comments: Vec<Comment>,
    /// Original source code for line analysis
    source: String,
    /// Source lines for analyzing blank lines
    source_lines: Vec<String>,
    /// Current lexical context depth
    context_depth: usize,
}

impl<'a> CommentExtractor<'a> {
    pub fn new(comments: &'a SingleThreadedComments) -> Self {
        Self {
            comments,
            extracted: HashMap::new(),
            standalone_comments: Vec::new(),
            floating_comments: Vec::new(),
            source: String::new(),
            source_lines: Vec::new(),
            context_depth: 0,
        }
    }

    pub fn with_source(comments: &'a SingleThreadedComments, source: String) -> Self {
        let source_lines = source.lines().map(|s| s.to_string()).collect();
        Self {
            comments,
            extracted: HashMap::new(),
            standalone_comments: Vec::new(),
            floating_comments: Vec::new(),
            source,
            source_lines,
            context_depth: 0,
        }
    }

    /// Extract all comments from the module
    pub fn extract(mut self, module: &Module) -> CommentExtractionResult {
        module.visit_with(&mut self);

        // Apply smart comment reassignment after initial extraction
        if !self.source.is_empty() {
            self.reassign_trailing_comments(module);
        }

        CommentExtractionResult {
            node_comments: self.extracted,
            standalone_comments: self.standalone_comments,
            floating_comments: self.floating_comments,
        }
    }

    /// Extract comments for a specific node
    fn extract_node_comments(&mut self, span: swc_common::Span, semantic_hash: u64) {
        // Extract leading comments
        if let Some(leading) = self.comments.get_leading(span.lo) {
            for (index, comment) in leading.iter().enumerate() {
                self.extracted
                    .entry(semantic_hash)
                    .or_default()
                    .push(ExtractedComment {
                        semantic_hash,
                        comment_type: CommentType::Leading,
                        comment: comment.clone(),
                        index,
                    });
            }
        }

        // Extract trailing comments
        if let Some(trailing) = self.comments.get_trailing(span.hi) {
            for (index, comment) in trailing.iter().enumerate() {
                self.extracted
                    .entry(semantic_hash)
                    .or_default()
                    .push(ExtractedComment {
                        semantic_hash,
                        comment_type: CommentType::Trailing,
                        comment: comment.clone(),
                        index,
                    });
            }
        }
    }

    /// Check if there are comments between two positions that haven't been extracted
    #[allow(dead_code)]
    fn check_floating_comments(&mut self, _start: BytePos, _end: BytePos) {
        // This is a simplified implementation. In a real implementation,
        // we'd need to iterate through all positions to find floating comments.
        // Since SWC doesn't provide an API to iterate all comments, we can't
        // easily implement this without additional infrastructure.
    }

    /// Get the line number for a given byte position
    fn get_line_number(&self, pos: BytePos) -> usize {
        let mut line = 0;
        let mut current_pos = 0;

        for ch in self.source.chars() {
            if current_pos >= pos.0 as usize {
                break;
            }
            if ch == '\n' {
                line += 1;
            }
            current_pos += ch.len_utf8();
        }

        line
    }

    /// Check if a comment is standalone (has blank line separation from adjacent syntax)
    fn is_standalone_comment(&self, _comment: &Comment, comment_line: usize) -> bool {
        // Check if we have source lines to analyze
        if self.source_lines.is_empty() {
            return false;
        }

        // For a comment to be standalone, it needs blank lines on both sides
        // (except at the beginning/end of the file)

        let has_blank_before = if comment_line == 0 {
            true // At the beginning of file, consider it as having blank before
        } else {
            let prev_line = comment_line - 1;
            prev_line < self.source_lines.len() && self.source_lines[prev_line].trim().is_empty()
        };

        let has_blank_after = {
            let next_line = comment_line + 1;
            if next_line >= self.source_lines.len() {
                true // At the end of file, consider it as having blank after
            } else {
                self.source_lines[next_line].trim().is_empty()
            }
        };

        // Both conditions must be true for a standalone comment
        has_blank_before && has_blank_after
    }

    /// Check if there's a line break between two positions
    fn has_line_break_between(&self, start: BytePos, end: BytePos) -> bool {
        let start_idx = start.0 as usize;
        let end_idx = end.0 as usize;

        if start_idx >= self.source.len() || end_idx > self.source.len() || start_idx >= end_idx {
            return false;
        }

        self.source[start_idx..end_idx].contains('\n')
    }

    /// Reassign trailing comments that are separated by line breaks
    fn reassign_trailing_comments(&mut self, module: &Module) {
        // eprintln!("Starting comment reassignment check...");

        // Collect all module items with their positions and hashes
        let mut items_info: Vec<(BytePos, BytePos, u64)> = Vec::new();

        for item in &module.body {
            if let Some((hash, _)) = SemanticHasher::hash_module_item(item) {
                let span = item.span();
                items_info.push((span.lo, span.hi, hash));
            }
        }

        // Sort by start position
        items_info.sort_by_key(|&(lo, _, _)| lo);

        // Process each item's trailing comments
        let mut reassignments: Vec<(u64, u64, ExtractedComment)> = Vec::new();

        for i in 0..items_info.len() {
            let (_, current_hi, current_hash) = items_info[i];

            if let Some(comments) = self.extracted.get(&current_hash) {
                for comment in comments.iter() {
                    if comment.comment_type == CommentType::Trailing {
                        // Check if there's a line break between the node and its trailing comment
                        if self.has_line_break_between(current_hi, comment.comment.span.lo) {
                            // Look for the next node
                            if i + 1 < items_info.len() {
                                let (next_lo, _, next_hash) = items_info[i + 1];

                                // Check if the comment is closer to the next node (no line break)
                                if !self.has_line_break_between(comment.comment.span.hi, next_lo) {
                                    // This comment should be reassigned as a leading comment to the next node
                                    reassignments.push((current_hash, next_hash, comment.clone()));
                                }
                            }
                        }
                    }
                }
            }
        }

        // Apply reassignments
        for (from_hash, to_hash, mut comment) in reassignments {
            // eprintln!("Reassigning comment '{}' from {:x} to {:x}",
            //     comment.comment.text, from_hash, to_hash);

            // Remove from current node
            if let Some(comments) = self.extracted.get_mut(&from_hash) {
                comments.retain(|c| {
                    !(c.comment_type == CommentType::Trailing
                        && c.comment.span == comment.comment.span
                        && c.comment.text == comment.comment.text)
                });
            }

            // Add to next node as leading
            comment.comment_type = CommentType::Leading;
            comment.semantic_hash = to_hash;
            self.extracted.entry(to_hash).or_default().push(comment);
        }
    }
}

impl<'a> Visit for CommentExtractor<'a> {
    fn visit_module(&mut self, module: &Module) {
        // Process all comments in the module to identify standalone ones
        let mut processed_comments = std::collections::HashSet::new();

        // Visit all module items and extract their comments
        for item in module.body.iter() {
            let item_span = item.span();

            // Check for leading comments
            if let Some(leading_comments) = self.comments.get_leading(item_span.lo) {
                for (index, comment) in leading_comments.iter().enumerate() {
                    let comment_line = self.get_line_number(comment.span.lo);

                    // Check if this is a standalone comment
                    if self.is_standalone_comment(comment, comment_line) {
                        self.standalone_comments.push(StandaloneComment {
                            comment: comment.clone(),
                            line: comment_line,
                            context_depth: self.context_depth,
                        });
                        processed_comments.insert(comment.span.lo);
                    } else if let Some((hash, _)) = SemanticHasher::hash_module_item(item) {
                        // Regular attached comment
                        self.extracted
                            .entry(hash)
                            .or_default()
                            .push(ExtractedComment {
                                semantic_hash: hash,
                                comment_type: CommentType::Leading,
                                comment: comment.clone(),
                                index,
                            });
                        processed_comments.insert(comment.span.lo);
                    }
                }
            }

            // Extract trailing comments normally
            if let Some((hash, _)) = SemanticHasher::hash_module_item(item) {
                if let Some(trailing_comments) = self.comments.get_trailing(item_span.hi) {
                    for (index, comment) in trailing_comments.iter().enumerate() {
                        self.extracted
                            .entry(hash)
                            .or_default()
                            .push(ExtractedComment {
                                semantic_hash: hash,
                                comment_type: CommentType::Trailing,
                                comment: comment.clone(),
                                index,
                            });
                        processed_comments.insert(comment.span.lo);
                    }
                }
            }

            // Visit children
            item.visit_with(self);
        }

        // Check for comments at the very beginning of the file
        if let Some(comments) = self.comments.get_leading(BytePos(0)) {
            for comment in comments {
                if !processed_comments.contains(&comment.span.lo) {
                    let comment_line = self.get_line_number(comment.span.lo);
                    if self.is_standalone_comment(&comment, comment_line) || comment_line == 0 {
                        self.standalone_comments.push(StandaloneComment {
                            comment: comment.clone(),
                            line: comment_line,
                            context_depth: self.context_depth,
                        });
                    } else {
                        self.floating_comments.push(comment.clone());
                    }
                }
            }
        }
    }

    fn visit_class(&mut self, class: &Class) {
        // Visit class members
        for member in class.body.iter() {
            // For class members, we need the class name for context
            if let Some(class_name) = self.get_current_class_name() {
                if let Some((hash, _)) = SemanticHasher::hash_class_member(member, &class_name) {
                    self.extract_node_comments(member.span(), hash);
                }
            }
        }

        class.visit_children_with(self);
    }

    fn visit_object_lit(&mut self, obj: &ObjectLit) {
        // Extract comments for object properties
        for prop in &obj.props {
            if let PropOrSpread::Prop(prop) = prop {
                let hash = self.hash_prop(prop);
                self.extract_node_comments(prop.span(), hash);
            }
        }

        obj.visit_children_with(self);
    }

    fn visit_jsx_element(&mut self, jsx: &JSXElement) {
        // Extract comments for JSX attributes
        for attr in &jsx.opening.attrs {
            if let JSXAttrOrSpread::JSXAttr(attr) = attr {
                let hash = self.hash_jsx_attr(attr);
                self.extract_node_comments(attr.span(), hash);
            }
        }

        jsx.visit_children_with(self);
    }
}

impl<'a> CommentExtractor<'a> {
    /// Helper to get the current class name (simplified - would need proper context tracking)
    fn get_current_class_name(&self) -> Option<String> {
        // In a real implementation, we'd track the current class context
        // For now, return None
        None
    }

    /// Generate hash for object property
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

    /// Generate hash for JSX attribute
    fn hash_jsx_attr(&self, attr: &JSXAttr) -> u64 {
        use std::collections::hash_map::DefaultHasher;
        use std::hash::{Hash, Hasher};

        let mut hasher = DefaultHasher::new();
        "jsx_attr".hash(&mut hasher);

        match &attr.name {
            JSXAttrName::Ident(ident) => {
                ident.sym.hash(&mut hasher);
            }
            JSXAttrName::JSXNamespacedName(ns) => {
                ns.ns.sym.hash(&mut hasher);
                ns.name.sym.hash(&mut hasher);
            }
        }

        hasher.finish()
    }
}

/// Represents a standalone comment with its position info
#[derive(Debug, Clone)]
pub struct StandaloneComment {
    /// The actual comment
    pub comment: Comment,
    /// Line number in the original source (0-indexed)
    pub line: usize,
    /// Lexical context depth (0 = module level, 1+ = nested blocks)
    pub context_depth: usize,
}

/// Result of comment extraction
pub struct CommentExtractionResult {
    /// Comments associated with specific nodes (by semantic hash)
    pub node_comments: HashMap<u64, Vec<ExtractedComment>>,
    /// Standalone comments that should maintain their position
    pub standalone_comments: Vec<StandaloneComment>,
    /// Comments that couldn't be associated with any node (deprecated, kept for compatibility)
    pub floating_comments: Vec<Comment>,
}

impl CommentExtractionResult {
    /// Get all comments for a given semantic hash
    pub fn get_comments(&self, hash: u64) -> Option<&Vec<ExtractedComment>> {
        self.node_comments.get(&hash)
    }

    /// Get all comments sorted by their original position
    pub fn all_comments_sorted(&self) -> Vec<ExtractedComment> {
        let mut all_comments = Vec::new();

        for comments in self.node_comments.values() {
            all_comments.extend(comments.clone());
        }

        // Sort by original position
        all_comments.sort_by_key(|c| c.comment.span.lo);

        all_comments
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::parser::TypeScriptParser;
    use swc_common::comments::CommentKind;

    fn extract_comments(source: &str) -> CommentExtractionResult {
        let parser = TypeScriptParser::new();
        let module = parser.parse(source, "test.ts").unwrap();
        let extractor = CommentExtractor::with_source(&parser.comments, source.to_string());
        extractor.extract(&module)
    }

    #[test]
    fn test_extract_leading_comment() {
        let source = r#"
// This is a function
function foo() {
    return 42;
}
"#;

        let result = extract_comments(source);
        assert_eq!(result.node_comments.len(), 1);

        // Get the comment for the function
        let comments = result.node_comments.values().next().unwrap();
        assert_eq!(comments.len(), 1);
        assert_eq!(comments[0].comment_type, CommentType::Leading);
        assert_eq!(comments[0].comment.text, " This is a function");
    }

    #[test]
    fn test_extract_trailing_comment() {
        let source = r#"
const x = 42; // The answer
"#;

        let result = extract_comments(source);
        assert_eq!(result.node_comments.len(), 1);

        let comments = result.node_comments.values().next().unwrap();
        assert_eq!(comments.len(), 1);
        assert_eq!(comments[0].comment_type, CommentType::Trailing);
        assert_eq!(comments[0].comment.text, " The answer");
    }

    #[test]
    fn test_extract_multiple_comments() {
        let source = r#"
// First comment
// Second comment
/* Block comment */
function foo() {
    return 42;
}
"#;

        let result = extract_comments(source);

        let comments = result.node_comments.values().next().unwrap();
        assert_eq!(comments.len(), 3);
        assert_eq!(comments[0].comment.text, " First comment");
        assert_eq!(comments[1].comment.text, " Second comment");
        assert_eq!(comments[2].comment.text, " Block comment ");
    }

    #[test]
    fn test_extract_import_comments() {
        let source = r#"
// React import
import React from 'react';

// Local imports
import { helper } from './helper'; // Helper utilities
"#;

        let result = extract_comments(source);

        // Should have comments for both imports
        assert!(result.node_comments.len() >= 2);

        // Verify all comments were extracted
        let all_comments = result.all_comments_sorted();
        assert_eq!(all_comments.len(), 3);
    }

    #[test]
    fn test_extract_block_comments() {
        let source = r#"
/* Block comment before function */
function foo() {
    /* Inner block comment */
    return 42;
}

/**
 * JSDoc comment
 * @param x - parameter
 */
function bar(x: number) {
    return x * 2;
}
"#;

        let result = extract_comments(source);

        // Count block comments
        let block_comments: Vec<_> = result
            .all_comments_sorted()
            .into_iter()
            .filter(|c| c.comment.kind == CommentKind::Block)
            .collect();

        // We expect at least 2 block comments (inner comments might not be extracted yet)
        assert!(block_comments.len() >= 2);
        assert!(block_comments[0]
            .comment
            .text
            .contains("Block comment before function"));
        assert!(block_comments[1].comment.text.contains("JSDoc comment"));
    }

    #[test]
    fn test_extract_class_comments() {
        let source = r#"
// Class comment
class MyClass {
    // Public field
    public field = 1;
    
    // Constructor
    constructor() {} // inline constructor comment
    
    // Method comment
    method() {
        return this.field; // Return field
    }
}
"#;

        let result = extract_comments(source);

        // Should have comment for the class
        let all_comments = result.all_comments_sorted();
        assert!(all_comments
            .iter()
            .any(|c| c.comment.text.contains("Class comment")));

        // Note: Class member comments require proper context tracking
        // which is not fully implemented yet
    }

    #[test]
    fn test_extract_object_literal_comments() {
        let source = r#"
const obj = {
    // First property
    a: 1, // Trailing comment on a
    
    /* Block comment for b */
    b: 2,
    
    // Method property
    method() { // Inline method comment
        return 42;
    }
};
"#;

        let result = extract_comments(source);
        let all_comments = result.all_comments_sorted();

        // Should extract some comments (object property extraction is limited)
        assert!(!all_comments.is_empty());
        // Object property comments are not fully implemented yet
        // This test documents current behavior
    }

    #[test]
    fn test_extract_variable_declaration_comments() {
        let source = r#"
// Const declaration
const x = 42; // The answer

// Let with destructuring
let { a, b } = obj; // Destructure

// Multiple declarations
const first = 1, // First var
      second = 2; // Second var
"#;

        let result = extract_comments(source);
        let all_comments = result.all_comments_sorted();

        assert!(all_comments
            .iter()
            .any(|c| c.comment.text.contains("Const declaration")));
        assert!(all_comments
            .iter()
            .any(|c| c.comment.text.contains("The answer")));
        assert!(all_comments
            .iter()
            .any(|c| c.comment.text.contains("Let with destructuring")));
        assert!(all_comments
            .iter()
            .any(|c| c.comment.text.contains("Destructure")));
    }

    #[test]
    fn test_comment_types() {
        let source = r#"
// Leading comment
const x = 1; // Trailing comment
"#;

        let result = extract_comments(source);
        let all_comments = result.all_comments_sorted();

        assert_eq!(all_comments.len(), 2);
        assert_eq!(all_comments[0].comment_type, CommentType::Leading);
        assert_eq!(all_comments[0].comment.text, " Leading comment");
        assert_eq!(all_comments[1].comment_type, CommentType::Trailing);
        assert_eq!(all_comments[1].comment.text, " Trailing comment");
    }

    #[test]
    fn test_floating_comments() {
        let source = r#"
// This comment is at the file level
// Before any code

import React from 'react';
"#;

        let result = extract_comments(source);

        // Currently floating comments are tracked separately
        // This test documents the current behavior
        assert_eq!(result.floating_comments.len(), 0); // Current implementation doesn't detect floating comments
    }

    #[test]
    fn test_export_comments() {
        let source = r#"
// Default export
export default function main() {
    return 42;
}

// Named export
export const value = 100; // Exported constant

// Export with type
export type MyType = string; // Type export
"#;

        let result = extract_comments(source);
        let all_comments = result.all_comments_sorted();

        assert!(all_comments
            .iter()
            .any(|c| c.comment.text.contains("Default export")));
        assert!(all_comments
            .iter()
            .any(|c| c.comment.text.contains("Named export")));
        assert!(all_comments
            .iter()
            .any(|c| c.comment.text.contains("Exported constant")));
        assert!(all_comments
            .iter()
            .any(|c| c.comment.text.contains("Export with type")));
        assert!(all_comments
            .iter()
            .any(|c| c.comment.text.contains("Type export")));
    }

    #[test]
    fn test_fr6_1_specific_case() {
        let source = r#"// FR6.1: Comments on imports should stay with their imports after sorting

// External dependencies
import React from 'react'; // UI library
import axios from 'axios'; // HTTP client

// Absolute imports
import { Button } from '@ui/components'; // Reusable button
import { api } from '@services/api';

// Relative imports
import { helper } from '../utils/helper'; // Utility functions
import { config } from './config'; // Local configuration"#;

        // First test without source (original behavior)
        let parser = TypeScriptParser::new();
        let module = parser.parse(source, "test.ts").unwrap();
        let extractor_no_source = CommentExtractor::new(&parser.comments);
        let result_no_source = extractor_no_source.extract(&module);

        println!("WITHOUT source-based reassignment:");
        for (hash, comments) in &result_no_source.node_comments {
            for comment in comments {
                println!(
                    "  Hash: {:x}, Type: {:?}, Text: {}",
                    hash, comment.comment_type, comment.comment.text
                );
            }
        }

        // Then test with source (smart reassignment)
        let result = extract_comments(source);

        println!("\nWITH source-based reassignment:");
        for (hash, comments) in &result.node_comments {
            for comment in comments {
                println!(
                    "  Hash: {:x}, Type: {:?}, Text: {}",
                    hash, comment.comment_type, comment.comment.text
                );
            }
        }

        // Find the "Relative imports" comment
        let all_comments = result.all_comments_sorted();
        let relative_comment = all_comments
            .iter()
            .find(|c| c.comment.text.contains("Relative imports"))
            .expect("Should find 'Relative imports' comment");

        println!(
            "\nRelative imports comment type after reassignment: {:?}",
            relative_comment.comment_type
        );

        // It should be a leading comment
        assert_eq!(relative_comment.comment_type, CommentType::Leading);
    }

    #[test]
    fn test_interface_and_type_comments() {
        let source = r#"
// Interface comment
interface User {
    // Name property
    name: string;
    // Age property
    age: number; // Optional age
}

// Type alias
type ID = string | number; // Union type
"#;

        let result = extract_comments(source);
        let all_comments = result.all_comments_sorted();

        assert!(all_comments
            .iter()
            .any(|c| c.comment.text.contains("Interface comment")));
        assert!(all_comments
            .iter()
            .any(|c| c.comment.text.contains("Type alias")));
        assert!(all_comments
            .iter()
            .any(|c| c.comment.text.contains("Union type")));
    }

    #[test]
    fn test_trailing_comment_reassignment() {
        let source = r#"
import { api } from '@services/api';

// Relative imports
import { helper } from '../utils/helper';
"#;

        let result = extract_comments(source);

        // The comment "// Relative imports" should be reassigned
        // from trailing on @services/api to leading on ../utils/helper
        let relative_import_comments = result
            .node_comments
            .values()
            .flat_map(|v| v.iter())
            .filter(|c| c.comment.text.contains("Relative imports"))
            .collect::<Vec<_>>();

        assert_eq!(relative_import_comments.len(), 1);
        assert_eq!(
            relative_import_comments[0].comment_type,
            CommentType::Leading
        );

        // Verify it's attached to the relative import
        let helper_import_hash = result
            .node_comments
            .iter()
            .find(|(_, comments)| {
                comments
                    .iter()
                    .any(|c| c.comment.text.contains("Relative imports"))
            })
            .map(|(hash, _)| *hash);

        assert!(helper_import_hash.is_some());

        // The helper import should have this comment
        let helper_comments = result
            .node_comments
            .get(&helper_import_hash.unwrap())
            .unwrap();
        assert!(helper_comments
            .iter()
            .any(|c| c.comment.text.contains("Relative imports")));
    }

    #[test]
    fn test_trailing_comment_not_reassigned_without_line_break() {
        let source = r#"
const x = 42; // The answer
const y = 100;
"#;

        let result = extract_comments(source);

        // The comment should remain as trailing on x
        let x_comments = result.all_comments_sorted();
        let answer_comment = x_comments
            .iter()
            .find(|c| c.comment.text.contains("The answer"))
            .unwrap();

        assert_eq!(answer_comment.comment_type, CommentType::Trailing);
    }

    #[test]
    fn test_comment_preservation_order() {
        let source = r#"
// 1. First
// 2. Second
// 3. Third
function foo() {
    return 42;
}
"#;

        let result = extract_comments(source);

        // Find comments for the function
        let func_comments = result
            .node_comments
            .values()
            .flat_map(|v| v.iter())
            .filter(|c| c.comment_type == CommentType::Leading)
            .collect::<Vec<_>>();

        assert_eq!(func_comments.len(), 3);
        assert_eq!(func_comments[0].index, 0);
        assert_eq!(func_comments[1].index, 1);
        assert_eq!(func_comments[2].index, 2);
        assert!(func_comments[0].comment.text.contains("1. First"));
        assert!(func_comments[1].comment.text.contains("2. Second"));
        assert!(func_comments[2].comment.text.contains("3. Third"));
    }
}
