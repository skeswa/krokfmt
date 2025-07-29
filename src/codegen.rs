use anyhow::Result;
use swc_common::{sync::Lrc, BytePos, SourceMap, Span, DUMMY_SP};
use swc_ecma_ast::*;
use swc_ecma_codegen::{text_writer::JsWriter, Config, Emitter};
use swc_ecma_visit::{Visit, VisitWith};

use crate::transformer::ImportCategory;

pub struct CodeGenerator {
    source_map: Lrc<SourceMap>,
}

impl CodeGenerator {
    pub fn new(source_map: Lrc<SourceMap>) -> Self {
        Self { source_map }
    }

    pub fn generate(&self, module: &Module) -> Result<String> {
        let mut buf = Vec::new();
        
        {
            let writer = JsWriter::new(
                self.source_map.clone(),
                "\n",
                &mut buf,
                None,
            );
            
            let config = Config {
                minify: false,
                target: swc_ecma_ast::EsVersion::Es2022,
                ascii_only: false,
                omit_last_semi: false,
            };
            
            let mut emitter = Emitter {
                cfg: config,
                cm: self.source_map.clone(),
                comments: None,
                wr: Box::new(writer),
            };
            
            // Custom emit to handle import grouping
            self.emit_with_import_grouping(&mut emitter, module)?;
        }
        
        Ok(String::from_utf8(buf)?)
    }

    fn emit_with_import_grouping<W: swc_ecma_codegen::text_writer::WriteJs>(
        &self,
        emitter: &mut Emitter<'_, W>,
        module: &Module,
    ) -> Result<()> {
        let mut last_was_import = false;
        let mut last_import_category: Option<ImportCategory> = None;
        
        for (index, item) in module.body.iter().enumerate() {
            match item {
                ModuleItem::ModuleDecl(ModuleDecl::Import(import)) => {
                    let current_category = self.categorize_import(&import.src.value);
                    
                    // Add empty line between different import categories
                    if let Some(last_cat) = last_import_category {
                        if std::mem::discriminant(&last_cat) != std::mem::discriminant(&current_category) {
                            emitter.wr.write_str("\n")?;
                        }
                    }
                    
                    emitter.emit_import_decl(import)?;
                    last_was_import = true;
                    last_import_category = Some(current_category);
                }
                _ => {
                    // Add empty line after imports section
                    if last_was_import {
                        emitter.wr.write_str("\n")?;
                        last_was_import = false;
                    }
                    
                    match item {
                        ModuleItem::ModuleDecl(decl) => emitter.emit_module_decl(decl)?,
                        ModuleItem::Stmt(stmt) => emitter.emit_stmt(stmt)?,
                    }
                }
            }
            
            // Add newline between items (except after the last item)
            if index < module.body.len() - 1 {
                emitter.wr.write_str("\n")?;
            }
        }
        
        Ok(())
    }

    fn categorize_import(&self, path: &str) -> ImportCategory {
        if path.starts_with("./") || path.starts_with("../") {
            ImportCategory::Relative
        } else if path.starts_with('@') || path.starts_with('~') {
            ImportCategory::Absolute
        } else {
            ImportCategory::External
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::parser::TypeScriptParser;
    use crate::formatter::KrokFormatter;
    use std::sync::Arc;

    fn format_and_generate(source: &str) -> Result<String> {
        let parser = TypeScriptParser::new();
        let module = parser.parse(source, "test.ts")?;
        let formatted = KrokFormatter::new().format(module)?;
        
        let source_map = Arc::new(SourceMap::default());
        let generator = CodeGenerator::new(source_map);
        generator.generate(&formatted)
    }

    #[test]
    fn test_generate_imports_with_spacing() {
        let source = r#"
import { helper } from './helper';
import React from 'react';
import { Button } from '@ui/Button';
"#;
        
        let output = format_and_generate(source).unwrap();
        
        // Should have external imports first, then absolute, then relative
        // with empty lines between categories
        assert!(output.contains("import React from 'react';\n\nimport { Button } from '@ui/Button';\n\nimport { helper } from './helper';"));
    }

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
        
        assert!(output.contains("import React from 'react';\n\nconst x = 42;"));
        assert!(output.contains("export function hello()"));
    }
}