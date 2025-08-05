pub mod biome_formatter;
pub mod codegen;
pub mod comment_classifier;
pub mod comment_extractor;
pub mod comment_formatter;
pub mod comment_reinserter;
pub mod file_handler;
pub mod organizer;
pub mod parser;
pub mod selective_comment_handler;
pub mod semantic_hash;
pub mod transformer;

use anyhow::{Context, Result};
use std::path::Path;

/// Simple heuristic to detect JSX content in source code.
/// Looks for common JSX patterns like <Component> or JSX expressions.
fn contains_jsx(source: &str) -> bool {
    // Look for JSX element patterns: < followed by uppercase letter or lowercase HTML tag
    // This is a simple heuristic that covers most cases
    source.contains("</") || source.contains("/>") || 
    source.contains("React.") || source.contains("jsx") ||
    // Check for common JSX patterns
    source.chars().zip(source.chars().skip(1)).any(|(c1, c2)| {
        c1 == '<' && (c2.is_ascii_uppercase() || c2.is_ascii_lowercase())
    })
}

/// Format TypeScript/TSX code with krokfmt's opinionated rules.
///
/// This is the main entry point for programmatic use of krokfmt.
/// It applies the full formatting pipeline: parsing, organizing, and final formatting.
pub fn format_typescript(source: &str, filename: &str) -> Result<String> {
    // Auto-detect JSX content and use appropriate extension
    let has_jsx = contains_jsx(source);
    let effective_filename =
        if !filename.ends_with(".tsx") && !filename.ends_with(".jsx") && has_jsx {
            // If the filename doesn't already indicate JSX/TSX and we detected JSX, use .tsx
            "input.tsx".to_string()
        } else if filename.ends_with(".ts") && has_jsx {
            // If it's explicitly .ts but contains JSX, convert to .tsx
            filename.replace(".ts", ".tsx")
        } else {
            filename.to_string()
        };

    // Parse the TypeScript code
    let parser = parser::TypeScriptParser::new();
    let source_map = parser.source_map.clone();
    let comments = parser.comments.clone();
    let module = parser
        .parse(source, &effective_filename)
        .context("Failed to parse TypeScript code")?;

    // Organize the code structure with selective comment preservation
    let formatter = comment_formatter::CommentFormatter::new(source_map, comments);
    let organized_content = formatter
        .format(module, source)
        .context("Failed to organize code")?;

    // Apply final formatting with Biome
    let biome_formatter = biome_formatter::BiomeFormatter::new();
    let formatted_content = biome_formatter
        .format(&organized_content, Path::new(&effective_filename))
        .context("Failed to format with Biome")?;

    Ok(formatted_content)
}
