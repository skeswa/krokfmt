use anyhow::{Context, Result};
use swc_common::{comments::SingleThreadedComments, sync::Lrc, FileName, SourceMap};
use swc_ecma_ast::Module;
use swc_ecma_parser::{lexer::Lexer, Parser, StringInput, Syntax, TsConfig};

/// Wrapper around SWC's TypeScript parser with our specific configuration.
///
/// We store source_map and comments as public fields because the formatter pipeline
/// needs to pass them through to the code generator. This preserves comment positioning
/// and source locations across the entire transformation.
pub struct TypeScriptParser {
    pub source_map: Lrc<SourceMap>,
    pub comments: SingleThreadedComments,
}

impl Default for TypeScriptParser {
    fn default() -> Self {
        Self {
            source_map: Lrc::new(SourceMap::default()),
            comments: SingleThreadedComments::default(),
        }
    }
}

impl TypeScriptParser {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn parse(&self, source: &str, filename: &str) -> Result<Module> {
        let fm = self
            .source_map
            .new_source_file(FileName::Custom(filename.to_string()), source.to_string());

        // TSX detection is file extension based - we chose this over content sniffing
        // to avoid ambiguity and match common tooling behavior (webpack, tsc, etc).
        let syntax = Syntax::Typescript(TsConfig {
            tsx: filename.ends_with(".tsx"),
            decorators: true,      // Always enabled since Angular/NestJS are popular
            no_early_errors: true, // We want to format even partially invalid code
            ..Default::default()
        });

        // The lexer needs comment tracking enabled to preserve them through formatting.
        // Without this, all comments would be stripped from the output.
        let lexer = Lexer::new(
            syntax,
            Default::default(),
            StringInput::from(&*fm),
            Some(&self.comments),
        );

        let mut parser = Parser::new_from(lexer);

        parser
            .parse_module()
            .map_err(|err| anyhow::anyhow!("Failed to parse {}: {:?}", filename, err))
            .context("Failed to parse TypeScript module")
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use swc_ecma_ast::*;

    #[test]
    fn test_parse_empty_file() {
        let parser = TypeScriptParser::new();
        let result = parser.parse("", "test.ts");
        assert!(result.is_ok());
        let module = result.unwrap();
        assert_eq!(module.body.len(), 0);
    }

    #[test]
    fn test_parse_simple_import() {
        let parser = TypeScriptParser::new();
        let source = r#"import { foo } from './bar';"#;
        let result = parser.parse(source, "test.ts");
        assert!(result.is_ok());

        let module = result.unwrap();
        assert_eq!(module.body.len(), 1);

        match &module.body[0] {
            ModuleItem::ModuleDecl(ModuleDecl::Import(_)) => {}
            _ => panic!("Expected import declaration"),
        }
    }

    #[test]
    fn test_parse_multiple_imports() {
        let parser = TypeScriptParser::new();
        let source = r#"
import React from 'react';
import { useState } from 'react';
import './styles.css';
import type { Props } from './types';
"#;
        let result = parser.parse(source, "test.ts");
        assert!(result.is_ok());

        let module = result.unwrap();
        assert_eq!(module.body.len(), 4);

        // All items should be imports
        for item in &module.body {
            match item {
                ModuleItem::ModuleDecl(ModuleDecl::Import(_)) => {}
                _ => panic!("Expected only import declarations"),
            }
        }
    }

    #[test]
    fn test_parse_exports() {
        let parser = TypeScriptParser::new();
        let source = r#"
export const foo = 42;
export function bar() {}
export { baz } from './baz';
export default class MyClass {}
"#;
        let result = parser.parse(source, "test.ts");
        assert!(result.is_ok());

        let module = result.unwrap();
        assert_eq!(module.body.len(), 4);
    }

    #[test]
    fn test_parse_typescript_specific_syntax() {
        let parser = TypeScriptParser::new();
        let source = r#"
interface User {
    name: string;
    age: number;
}

type ID = string | number;

enum Status {
    Active,
    Inactive
}

const user: User = { name: "John", age: 30 };
"#;
        let result = parser.parse(source, "test.ts");
        assert!(result.is_ok());
    }

    #[test]
    fn test_parse_tsx_file() {
        let parser = TypeScriptParser::new();
        let source = r#"
import React from 'react';

interface Props {
    title: string;
}

export const Component: React.FC<Props> = ({ title }) => {
    return <div>{title}</div>;
};
"#;
        let result = parser.parse(source, "test.tsx");
        assert!(result.is_ok());
    }

    #[test]
    fn test_parse_syntax_error() {
        let parser = TypeScriptParser::new();
        let source = r#"import { foo from './bar';"#; // Missing closing brace
        let result = parser.parse(source, "test.ts");
        assert!(result.is_err());
    }
}
