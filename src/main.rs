use anyhow::{Context, Result};
use clap::Parser;
use colored::Colorize;
use rayon::prelude::*;
use std::path::{Path, PathBuf};

use krokfmt::{
    codegen::CodeGenerator, file_handler::FileHandler, formatter::KrokFormatter,
    parser::TypeScriptParser,
};

/// Command-line interface for krokfmt.
///
/// The decision to be "highly opinionated" was intentional - we wanted to eliminate
/// configuration debates entirely. No options means no bikeshedding, allowing teams
/// to focus on writing code rather than arguing about formatting preferences.
#[derive(Parser)]
#[command(name = "krokfmt")]
#[command(author = "krokorok")]
#[command(version)]
#[command(about = "A highly opinionated TypeScript code formatter", long_about = None)]
struct Cli {
    #[arg(help = "Files or directories to format")]
    paths: Vec<PathBuf>,

    // The check mode exists because CI/CD pipelines need to verify formatting
    // without accidentally modifying committed code. This follows the pattern
    // established by other formatters like rustfmt and prettier.
    #[arg(
        short,
        long,
        help = "Check if files are formatted without modifying them"
    )]
    check: bool,

    // stdout mode was added for editor integrations and quick previews.
    // Many editors expect formatters to output to stdout for real-time formatting.
    #[arg(
        long,
        help = "Print formatted output to stdout instead of writing to file"
    )]
    stdout: bool,

    // Backups were made opt-out rather than opt-in because we've seen too many
    // formatters corrupt files due to parser bugs. Better safe than sorry.
    #[arg(long, help = "Skip creating backups of original files")]
    no_backup: bool,
}

fn main() -> Result<()> {
    let cli = Cli::parse();

    // Early exit with clear error - we chose to make this a hard error rather than
    // defaulting to current directory to prevent accidental mass reformatting.
    if cli.paths.is_empty() {
        eprintln!("{}", "Error: No files or directories specified".red());
        std::process::exit(1);
    }

    let file_handler = FileHandler::new(!cli.no_backup);
    let files = file_handler.find_typescript_files(&cli.paths)?;

    if files.is_empty() {
        println!("{}", "No TypeScript files found".yellow());
        return Ok(());
    }

    println!("{} {} files", "Formatting".green(), files.len());

    let mut had_changes = false;
    let mut had_errors = false;

    // Parallel processing was crucial for large codebases. We use rayon's work-stealing
    // to handle varying file sizes efficiently - small files don't block large ones.
    let results: Vec<_> = files
        .par_iter()
        .map(|file| process_file(&file_handler, file, &cli))
        .collect();

    // We collect results first, then report them sequentially to avoid jumbled output
    // from parallel processing. The colored output helps users quickly scan results.
    for (file, result) in files.iter().zip(results.iter()) {
        match result {
            Ok(changed) => {
                if *changed {
                    had_changes = true;
                    // In check mode, changes are failures - we show red X to indicate
                    // the file would be modified if we weren't in check mode.
                    if cli.check {
                        println!("{} {}", "✗".red(), file.display());
                    } else {
                        println!("{} {}", "✓".green(), file.display());
                    }
                } else {
                    println!("{} {} (no changes)", "✓".green(), file.display());
                }
            }
            Err(e) => {
                had_errors = true;
                eprintln!("{} {}: {}", "✗".red(), file.display(), e);
            }
        }
    }

    // Exit codes matter for CI/CD integration. We use standard Unix conventions:
    // 0 = success, 1 = expected failure (formatting needed), >1 = unexpected error
    if cli.check && had_changes {
        eprintln!("\n{}", "Some files are not formatted".red());
        std::process::exit(1);
    }

    if had_errors {
        eprintln!("\n{}", "Some files had errors".red());
        std::process::exit(1);
    }

    println!("\n{}", "All files formatted successfully".green());
    Ok(())
}

/// Process a single TypeScript file through the parse-format-generate pipeline.
///
/// Returns true if the file was changed, false if it was already formatted.
/// This boolean is crucial for check mode to determine exit codes.
fn process_file(file_handler: &FileHandler, path: &Path, cli: &Cli) -> Result<bool> {
    let content = file_handler.read_file(path)?;

    // We need to clone source_map and comments before parsing because the parser
    // consumes them. This allows the code generator to preserve comments and spans.
    let parser = TypeScriptParser::new();
    let source_map = parser.source_map.clone();
    let comments = parser.comments.clone();
    let module = parser
        .parse(&content, path.to_str().unwrap_or("unknown.ts"))
        .context("Failed to parse file")?;

    let formatter = KrokFormatter::new();
    let formatted_module = formatter.format(module).context("Failed to format file")?;

    // The generator needs both source_map and comments to maintain comment positioning
    // relative to the reformatted code structure.
    let generator = CodeGenerator::with_comments(source_map, comments);
    let formatted_content = generator
        .generate(&formatted_module)
        .context("Failed to generate code")?;

    // Simple string comparison is sufficient here - we're not doing a semantic diff
    // because any change, even whitespace, is a formatting change.
    if content == formatted_content {
        return Ok(false);
    }

    // Output handling is mutually exclusive: stdout for editor integration,
    // file writing for normal operation, or neither for check mode.
    if cli.stdout {
        println!("{formatted_content}");
    } else if !cli.check {
        file_handler.write_file(path, &formatted_content)?;
    }

    Ok(true)
}
