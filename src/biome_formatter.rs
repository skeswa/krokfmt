use anyhow::{Context, Result};
use biome_formatter::{AttributePosition, IndentStyle, LineWidth, QuoteStyle};
use biome_js_formatter::context::{ArrowParentheses, JsFormatOptions, Semicolons};
use biome_js_formatter::format_node;
use biome_js_parser::{parse, JsParserOptions};
use biome_js_syntax::JsFileSource;
use std::path::Path;

/// Configuration for the Biome formatter
#[derive(Debug, Clone)]
pub struct BiomeFormatterConfig {
    /// Whether to use spaces or tabs for indentation
    pub indent_style: IndentStyle,
    /// The width of a single indentation level
    pub indent_width: u8,
    /// The line width at which Biome will try to wrap code
    pub line_width: u16,
    /// The style for quotes in JavaScript
    pub quote_style: QuoteStyle,
    /// The style for quotes in JSX
    pub jsx_quote_style: QuoteStyle,
    /// When to use parentheses around arrow functions
    pub arrow_parentheses: ArrowParentheses,
    /// Whether to always use semicolons or rely on ASI
    pub semicolons: Semicolons,
    /// How to position attributes in JSX/HTML
    pub attribute_position: AttributePosition,
}

impl Default for BiomeFormatterConfig {
    fn default() -> Self {
        Self {
            indent_style: IndentStyle::Space,
            indent_width: 2,
            line_width: 80,
            quote_style: QuoteStyle::Double,
            jsx_quote_style: QuoteStyle::Double,
            arrow_parentheses: ArrowParentheses::AsNeeded,
            semicolons: Semicolons::Always,
            attribute_position: AttributePosition::Multiline,
        }
    }
}

/// Biome formatter wrapper for krokfmt
pub struct BiomeFormatter {
    config: BiomeFormatterConfig,
}

impl BiomeFormatter {
    /// Create a new BiomeFormatter with default configuration
    pub fn new() -> Self {
        Self {
            config: BiomeFormatterConfig::default(),
        }
    }

    /// Create a new BiomeFormatter with custom configuration
    pub fn with_config(config: BiomeFormatterConfig) -> Self {
        Self { config }
    }

    /// Format the given code using Biome
    ///
    /// This applies consistent formatting rules to already-organized code.
    /// The path is used to determine the source type (JS/TS/JSX/TSX).
    pub fn format(&self, code: &str, path: &Path) -> Result<String> {
        // Determine the source type from the file extension
        let source_type = JsFileSource::try_from(path)
            .with_context(|| format!("Failed to determine source type for {path:?}"))?;

        // Parse the code
        let parsed = parse(
            code,
            source_type,
            JsParserOptions::default().with_parse_class_parameter_decorators(),
        );

        // Check for parse errors
        if parsed.has_errors() {
            let errors: Vec<_> = parsed.diagnostics().iter().collect();
            anyhow::bail!(
                "Biome parser errors: {}",
                errors
                    .iter()
                    .map(|e| format!("{e:?}"))
                    .collect::<Vec<_>>()
                    .join(", ")
            );
        }

        // Create format options from our configuration
        let format_options = self.create_format_options(source_type);

        // Format the syntax tree
        let formatted =
            format_node(format_options, &parsed.syntax()).context("Failed to format with Biome")?;

        // Convert to final code
        let result = formatted
            .print()
            .context("Failed to print formatted code")?;

        Ok(result.into_code())
    }

    /// Create Biome format options from our configuration
    fn create_format_options(&self, source_type: JsFileSource) -> JsFormatOptions {
        JsFormatOptions::new(source_type)
            .with_indent_style(self.config.indent_style)
            .with_indent_width(self.config.indent_width.into())
            .with_line_width(
                LineWidth::try_from(self.config.line_width)
                    .unwrap_or_else(|_| LineWidth::try_from(80).unwrap()),
            )
            .with_quote_style(self.config.quote_style)
            .with_jsx_quote_style(self.config.jsx_quote_style)
            .with_arrow_parentheses(self.config.arrow_parentheses)
            .with_semicolons(self.config.semicolons)
            .with_attribute_position(self.config.attribute_position)
    }
}

impl Default for BiomeFormatter {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::path::PathBuf;

    #[test]
    fn test_basic_formatting() {
        let formatter = BiomeFormatter::new();
        let code = r#"const x={a:1,b:2,c:3};function   foo(  )  {return    x.a+x.b}"#;
        let path = PathBuf::from("test.js");

        let result = formatter.format(code, &path).unwrap();

        // Biome should format this nicely
        assert!(result.contains("const x = { a: 1, b: 2, c: 3 }"));
        assert!(result.contains("function foo()"));
        assert!(result.contains("return x.a + x.b"));
    }

    #[test]
    fn test_typescript_formatting() {
        let formatter = BiomeFormatter::new();
        let code = r#"interface User{name:string;age:number}const greet=(user:User):string=>{return `Hello ${user.name}`}"#;
        let path = PathBuf::from("test.ts");

        let result = formatter.format(code, &path).unwrap();

        // Should format TypeScript properly
        assert!(result.contains("interface User"));
        assert!(result.contains("name: string"));
        assert!(result.contains("age: number"));
        assert!(result.contains("const greet = (user: User): string =>"));
    }

    #[test]
    fn test_jsx_formatting() {
        let formatter = BiomeFormatter::new();
        let code = r#"const App=()=>{return<div className="app"><h1>Hello</h1><button onClick={()=>alert('clicked')}>Click</button></div>}"#;
        let path = PathBuf::from("test.jsx");

        let result = formatter.format(code, &path).unwrap();

        // Should format JSX properly
        assert!(result.contains("const App = () =>"));
        assert!(result.contains("<div className=\"app\">"));
        assert!(result.contains("<h1>Hello</h1>"));
        assert!(result.contains("onClick={() => alert(\"clicked\")}"));
    }

    #[test]
    fn test_custom_config() {
        let config = BiomeFormatterConfig {
            indent_style: IndentStyle::Tab,
            indent_width: 4,
            line_width: 120,
            quote_style: QuoteStyle::Single,
            jsx_quote_style: QuoteStyle::Single,
            arrow_parentheses: ArrowParentheses::Always,
            semicolons: Semicolons::AsNeeded,
            attribute_position: AttributePosition::Auto,
        };

        let formatter = BiomeFormatter::with_config(config);
        let code = r#"const message = "Hello"; const fn = x => x * 2"#;
        let path = PathBuf::from("test.js");

        let result = formatter.format(code, &path).unwrap();

        // Should use single quotes and no semicolons (based on config)
        assert!(result.contains("const message = 'Hello'"));
        assert!(result.contains("const fn = (x) => x * 2"));
    }
}
