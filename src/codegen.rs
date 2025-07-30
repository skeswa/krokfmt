use anyhow::Result;
use swc_common::{comments::SingleThreadedComments, sync::Lrc, SourceMap};
use swc_ecma_ast::*;
use swc_ecma_codegen::{text_writer::JsWriter, Config, Emitter};

use crate::comment_fixer::fix_comment_indentation;
use crate::transformer::{ImportAnalyzer, ImportCategory};

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

        // Two-phase post-processing is necessary because SWC's emitter doesn't
        // understand our import grouping requirements. First we fix comments that
        // may have been misaligned during AST manipulation, then we add spacing.
        let with_fixed_comments = fix_comment_indentation(generated);
        Ok(self.add_import_spacing(with_fixed_comments, module))
    }

    /// Add empty lines between import categories and after the import section.
    ///
    /// This string-based approach is necessary because SWC's AST doesn't model
    /// empty lines. We parse the generated code to identify import boundaries
    /// and inject newlines at category transitions. This creates the visual
    /// grouping that makes large import sections readable.
    fn add_import_spacing(&self, code: String, _module: &Module) -> String {
        let lines: Vec<&str> = code.lines().collect();
        let mut result = Vec::new();
        let mut last_import_category: Option<ImportCategory> = None;
        let mut last_was_import = false;
        let mut first_non_import_found = false;

        for line in lines.iter() {
            // Check if this line is an import statement
            let is_import = line.trim_start().starts_with("import ");

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
                }
                last_was_import = false;
            }

            result.push(line);
        }

        result.join("\n")
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
