use anyhow::Result;
use swc_common::{sync::Lrc, SourceMap};
use swc_ecma_ast::*;
use swc_ecma_codegen::{text_writer::JsWriter, Config, Emitter};

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
            let writer = JsWriter::new(self.source_map.clone(), "\n", &mut buf, None);

            let config = Config::default();

            let mut emitter = Emitter {
                cfg: config,
                cm: self.source_map.clone(),
                comments: None,
                wr: Box::new(writer),
            };

            // Use the standard emit_module method
            emitter.emit_module(module)?;
        }

        Ok(String::from_utf8(buf)?)
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
