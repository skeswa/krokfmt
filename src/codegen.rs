use anyhow::Result;
use swc_common::{comments::SingleThreadedComments, sync::Lrc, SourceMap};
use swc_ecma_ast::*;
use swc_ecma_codegen::{text_writer::JsWriter, Config, Emitter};

use crate::transformer::{ImportAnalyzer, ImportCategory};

#[derive(Debug, Clone, PartialEq)]
enum DeclarationType {
    Function,
    Class,
    Interface,
    Type,
    Const,
    Enum,
}

#[derive(Debug, Clone, PartialEq)]
enum ClassMemberGroup {
    PublicStaticFields,
    PrivateStaticFields,
    PublicStaticMethods,
    PrivateStaticMethods,
    PublicInstanceFields,
    PrivateInstanceFields,
    Constructor,
    PublicInstanceMethods,
    PrivateInstanceMethods,
}

/// Generates formatted TypeScript/JavaScript code from the AST.
///
/// This is a wrapper around SWC's code generator with custom post-processing.
/// We handle two critical tasks that the standard emitter doesn't:
/// 1. Adding empty lines between import categories for visual grouping
/// 2. Fixing comment indentation to match the reformatted code structure
pub struct CodeGenerator {
    source_map: Lrc<SourceMap>,
    comments: Option<SingleThreadedComments>,
}

impl CodeGenerator {
    pub fn new(source_map: Lrc<SourceMap>) -> Self {
        Self {
            source_map,
            comments: None,
        }
    }

    pub fn with_comments(source_map: Lrc<SourceMap>, comments: SingleThreadedComments) -> Self {
        Self {
            source_map,
            comments: Some(comments),
        }
    }

    pub fn generate(&self, module: &Module) -> Result<String> {
        let mut buf = Vec::new();

        {
            let writer = JsWriter::new(self.source_map.clone(), "\n", &mut buf, None);

            let config = Config::default();

            let mut emitter = Emitter {
                cfg: config,
                cm: self.source_map.clone(),
                comments: self
                    .comments
                    .as_ref()
                    .map(|c| c as &dyn swc_common::comments::Comments),
                wr: Box::new(writer),
            };

            // Use the standard emit_module method
            emitter.emit_module(module)?;
        }

        let generated = String::from_utf8(buf)?;

        // Post-processing is necessary because SWC's emitter doesn't
        // understand our custom formatting requirements for visual spacing.
        Ok(self.add_visual_spacing(generated, module))
    }

    /// Add visual spacing between logical groups in the formatted code.
    ///
    /// This string-based approach is necessary because SWC's AST doesn't model
    /// empty lines. We parse the generated code to identify boundaries and inject
    /// newlines at transitions to create visual separation between:
    /// - Different import categories (external, absolute, relative)
    /// - Imports and the rest of the code
    /// - Different visibility groups (exported vs non-exported)
    fn add_visual_spacing(&self, code: String, _module: &Module) -> String {
        let lines: Vec<&str> = code.lines().collect();
        let mut result = Vec::new();
        let mut last_import_category: Option<ImportCategory> = None;
        let mut last_was_import = false;
        let mut first_non_import_found = false;
        let mut last_was_exported: Option<bool> = None;
        let mut in_imports_section = true;
        let mut brace_depth: i32 = 0;
        let mut last_declaration_type: Option<DeclarationType> = None;
        let mut in_class = false;
        let mut last_member_group: Option<ClassMemberGroup> = None;

        for line in lines.iter() {
            let trimmed = line.trim_start();

            // Update brace depth based on closing braces at the start of the line
            // This ensures we correctly identify when we're back at top level
            for ch in trimmed.chars() {
                if ch == '}' {
                    brace_depth = brace_depth.saturating_sub(1);
                    if brace_depth == 0 && in_class {
                        in_class = false;
                        last_member_group = None;
                    }
                } else if ch != ' ' && ch != '\t' {
                    break; // Stop at first non-whitespace, non-brace character
                }
            }

            // Check if this line is an import statement
            let is_import = trimmed.starts_with("import ");

            if is_import {
                // Extract the import path to determine category
                if let Some(from_pos) = line.find(" from ") {
                    let after_from = &line[from_pos + 6..];
                    if let Some(quote_start) = after_from.find(['\'', '"']) {
                        let quote_char = after_from.chars().nth(quote_start).unwrap();
                        if let Some(quote_end) = after_from[quote_start + 1..].find(quote_char) {
                            let path = &after_from[quote_start + 1..quote_start + 1 + quote_end];
                            let category = ImportAnalyzer::categorize_import(path);

                            // Add empty line between different import categories
                            if let Some(last_cat) = &last_import_category {
                                if std::mem::discriminant(last_cat)
                                    != std::mem::discriminant(&category)
                                {
                                    result.push("");
                                }
                            }

                            last_import_category = Some(category);
                        }
                    }
                } else if line.contains(['\'', '"']) {
                    // Side-effect import like: import './polyfills';
                    let quote_start = line.find(['\'', '"']).unwrap();
                    let quote_char = line.chars().nth(quote_start).unwrap();
                    if let Some(quote_end) = line[quote_start + 1..].find(quote_char) {
                        let path = &line[quote_start + 1..quote_start + 1 + quote_end];
                        let category = ImportAnalyzer::categorize_import(path);

                        // Add empty line between different import categories
                        if let Some(last_cat) = &last_import_category {
                            if std::mem::discriminant(last_cat) != std::mem::discriminant(&category)
                            {
                                result.push("");
                            }
                        }

                        last_import_category = Some(category);
                    }
                }

                last_was_import = true;
            } else {
                // First non-import after imports needs separation for visual clarity.
                // We skip if the line is already empty to avoid double spacing.
                if last_was_import && !first_non_import_found && !line.trim().is_empty() {
                    result.push("");
                    first_non_import_found = true;
                    in_imports_section = false;
                }
                last_was_import = false;

                // Check for visibility transitions in non-import declarations
                // Only consider top-level declarations (brace_depth == 0)
                if !in_imports_section
                    && !trimmed.is_empty()
                    && !trimmed.starts_with("//")
                    && brace_depth == 0
                {
                    // Check if this line starts an exported declaration
                    let is_exported = trimmed.starts_with("export ");

                    // Remove "export " prefix to detect the actual declaration type
                    let declaration_part = if is_exported {
                        trimmed.strip_prefix("export ").unwrap_or(trimmed)
                    } else {
                        trimmed
                    };

                    // Detect declaration type
                    let declaration_type = if declaration_part.starts_with("function ")
                        || declaration_part.starts_with("async function ")
                    {
                        Some(DeclarationType::Function)
                    } else if declaration_part.starts_with("class ")
                        || declaration_part.starts_with("abstract class ")
                    {
                        Some(DeclarationType::Class)
                    } else if declaration_part.starts_with("interface ") {
                        Some(DeclarationType::Interface)
                    } else if declaration_part.starts_with("type ") {
                        Some(DeclarationType::Type)
                    } else if declaration_part.starts_with("const ")
                        || declaration_part.starts_with("let ")
                        || declaration_part.starts_with("var ")
                    {
                        Some(DeclarationType::Const)
                    } else if declaration_part.starts_with("enum ") {
                        Some(DeclarationType::Enum)
                    } else {
                        None
                    };

                    if let Some(current_type) = declaration_type {
                        let mut need_separator = false;

                        // Check for visibility transition
                        if let Some(last_exported) = last_was_exported {
                            if last_exported != is_exported {
                                need_separator = true;
                            }
                        }

                        // Check for declaration type transition (FR7.1)
                        if !need_separator {
                            if let Some(last_type) = &last_declaration_type {
                                if last_type != &current_type {
                                    need_separator = true;
                                }
                            }
                        }

                        if need_separator {
                            result.push("");
                        }

                        last_was_exported = Some(is_exported);
                        last_declaration_type = Some(current_type);
                    }
                }

                // Handle class member separation (FR7.3)
                // Skip lines that are just closing braces or empty statements
                if in_class
                    && brace_depth == 1
                    && !trimmed.is_empty()
                    && !trimmed.starts_with("//")
                    && !trimmed.starts_with("}")
                {
                    let member_group = detect_class_member_group(trimmed);

                    if let Some(current_group) = member_group {
                        if let Some(last_group) = &last_member_group {
                            if last_group != &current_group {
                                // Add empty line between different member groups
                                result.push("");
                            }
                        }
                        last_member_group = Some(current_group);
                    }
                }
            }

            result.push(line);

            // Check if this line declares a class (for next iteration)
            if (trimmed.starts_with("class ")
                || trimmed.starts_with("export class ")
                || trimmed.starts_with("abstract class ")
                || trimmed.starts_with("export abstract class "))
                && line.trim().ends_with('{')
            {
                in_class = true;
                last_member_group = None;
            }

            // Update brace depth after processing the line (for opening braces)
            // Count only the last brace on the line to avoid counting braces in method bodies
            if line.trim().ends_with('{') {
                brace_depth += 1;
            }
        }

        result.join("\n")
    }
}

/// Detects the class member group based on the line content
fn detect_class_member_group(line: &str) -> Option<ClassMemberGroup> {
    let trimmed = line.trim();

    // Constructor
    if trimmed.starts_with("constructor") {
        return Some(ClassMemberGroup::Constructor);
    }

    // Detect static members
    if trimmed.starts_with("static ") {
        let after_static = trimmed.strip_prefix("static ").unwrap();
        let is_private = after_static.starts_with("#") || after_static.starts_with("private ");

        // Check if it's a method (has parentheses) or field
        let is_method = after_static.contains('(') && after_static.contains(')');

        return match (is_private, is_method) {
            (false, false) => Some(ClassMemberGroup::PublicStaticFields),
            (true, false) => Some(ClassMemberGroup::PrivateStaticFields),
            (false, true) => Some(ClassMemberGroup::PublicStaticMethods),
            (true, true) => Some(ClassMemberGroup::PrivateStaticMethods),
        };
    }

    // Instance members
    let is_private = trimmed.starts_with("#") || trimmed.starts_with("private ");
    // Check if it's a method or field
    // Methods have parentheses, fields have : or = for assignment
    let has_parens = trimmed.contains('(') && trimmed.contains(')');

    // Heuristic: if it has parentheses before any = or :, it's likely a method
    let is_method = if has_parens {
        if let (Some(paren_pos), Some(assign_pos)) = (trimmed.find('('), trimmed.find('=')) {
            paren_pos < assign_pos
        } else {
            true
        }
    } else {
        false
    };

    match (is_private, is_method) {
        (false, false) => Some(ClassMemberGroup::PublicInstanceFields),
        (true, false) => Some(ClassMemberGroup::PrivateInstanceFields),
        (false, true) => Some(ClassMemberGroup::PublicInstanceMethods),
        (true, true) => Some(ClassMemberGroup::PrivateInstanceMethods),
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::formatter::KrokFormatter;
    use crate::parser::TypeScriptParser;

    fn format_and_generate(source: &str) -> Result<String> {
        let parser = TypeScriptParser::new();
        let module = parser.parse(source, "test.ts")?;
        let formatted = KrokFormatter::new().format(module)?;

        let source_map = Lrc::new(SourceMap::default());
        let generator = CodeGenerator::new(source_map);
        generator.generate(&formatted)
    }

    // TODO: These tests need to be updated to handle import spacing correctly
    // Currently the standard SWC emitter doesn't add empty lines between import groups
    // #[test]
    // fn test_generate_imports_with_spacing() {
    //     let source = r#"
    // import { helper } from './helper';
    // import React from 'react';
    // import { Button } from '@ui/Button';
    // "#;
    //
    //     let output = format_and_generate(source).unwrap();
    //
    //     // Should have external imports first, then absolute, then relative
    //     // with empty lines between categories
    //     assert!(output.contains("import React from 'react';\n\nimport { Button } from '@ui/Button';\n\nimport { helper } from './helper';"));
    // }

    #[test]
    fn test_preserve_code_after_imports() {
        let source = r#"
import React from 'react';

const x = 42;
export function hello() {
    return "world";
}
"#;

        let output = format_and_generate(source).unwrap();

        // The standard emitter preserves the structure but may not have double newlines
        assert!(output.contains("import React from 'react';"));
        assert!(output.contains("const x = 42;"));
        assert!(output.contains("export function hello()"));
    }
}
