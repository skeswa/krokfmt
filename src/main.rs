use anyhow::{Context, Result};
use clap::Parser;
use colored::Colorize;
use rayon::prelude::*;
use std::path::PathBuf;
use std::sync::Arc;

use krokfmt::{
    codegen::CodeGenerator,
    file_handler::FileHandler,
    formatter::KrokFormatter,
    parser::TypeScriptParser,
};

#[derive(Parser)]
#[command(name = "krokfmt")]
#[command(author = "krokorok")]
#[command(version)]
#[command(about = "A highly opinionated TypeScript code formatter", long_about = None)]
struct Cli {
    #[arg(help = "Files or directories to format")]
    paths: Vec<PathBuf>,

    #[arg(short, long, help = "Check if files are formatted without modifying them")]
    check: bool,

    #[arg(long, help = "Print formatted output to stdout instead of writing to file")]
    stdout: bool,

    #[arg(long, help = "Skip creating backups of original files")]
    no_backup: bool,
}

fn main() -> Result<()> {
    let cli = Cli::parse();

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

    // Process files in parallel for better performance
    let results: Vec<_> = files
        .par_iter()
        .map(|file| process_file(&file_handler, file, &cli))
        .collect();

    for (file, result) in files.iter().zip(results.iter()) {
        match result {
            Ok(changed) => {
                if *changed {
                    had_changes = true;
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

fn process_file(file_handler: &FileHandler, path: &PathBuf, cli: &Cli) -> Result<bool> {
    let content = file_handler.read_file(path)?;
    
    // Parse
    let parser = TypeScriptParser::new();
    let source_map = Arc::clone(&parser.source_map);
    let module = parser
        .parse(&content, path.to_str().unwrap_or("unknown.ts"))
        .context("Failed to parse file")?;
    
    // Format
    let formatter = KrokFormatter::new();
    let formatted_module = formatter.format(module).context("Failed to format file")?;
    
    // Generate
    let generator = CodeGenerator::new(source_map);
    let formatted_content = generator
        .generate(&formatted_module)
        .context("Failed to generate code")?;
    
    // Check if content changed
    if content == formatted_content {
        return Ok(false);
    }
    
    // Handle output
    if cli.stdout {
        println!("{}", formatted_content);
    } else if !cli.check {
        file_handler.write_file(path, &formatted_content)?;
    }
    
    Ok(true)
}