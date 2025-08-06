use anyhow::{Context, Result};
use clap::{Parser, Subcommand};
use std::env;
use std::path::Path;
use std::time::SystemTime;
use xshell::{cmd, Shell};

/// Build automation for krokfmt
#[derive(Parser)]
#[command(name = "xtask")]
#[command(about = "Build automation for krokfmt", long_about = None)]
struct Cli {
    #[command(subcommand)]
    command: Command,
}

#[derive(Subcommand)]
enum Command {
    /// Build everything (WASM and web server)
    Build {
        /// Build in release mode
        #[arg(long)]
        release: bool,
    },
    /// Build WASM module only
    BuildWasm {
        /// Build in release mode
        #[arg(long)]
        release: bool,
        /// Force rebuild even if up to date
        #[arg(long)]
        force: bool,
    },
    /// Build web server only
    BuildWeb {
        /// Build in release mode
        #[arg(long)]
        release: bool,
    },
    /// Run all tests and checks
    Test,
    /// Run the web server locally
    RunWeb {
        /// Build in release mode
        #[arg(long)]
        release: bool,
    },
    /// Clean build artifacts
    Clean,
    /// Build Docker image
    DockerBuild,
    /// Run Docker container
    DockerRun,
    /// Install required dependencies
    InstallDeps,
    /// Format all code
    Fmt {
        /// Check formatting without applying changes
        #[arg(long)]
        check: bool,
    },
    /// Run clippy lints
    Clippy,
    /// Run CI checks (fmt, clippy, test)
    Ci,
}

fn main() -> Result<()> {
    let cli = Cli::parse();
    let sh = Shell::new()?;

    // Change to workspace root
    let workspace_root = project_root()?;
    sh.change_dir(workspace_root);

    match cli.command {
        Command::Build { release } => {
            build_wasm(&sh, release, false)?;
            build_web(&sh, release)?;
        }
        Command::BuildWasm { release, force } => {
            build_wasm(&sh, release, force)?;
        }
        Command::BuildWeb { release } => {
            build_web(&sh, release)?;
        }
        Command::Test => {
            test(&sh)?;
        }
        Command::RunWeb { release } => {
            build_wasm(&sh, release, false)?;
            run_web(&sh, release)?;
        }
        Command::Clean => {
            clean(&sh)?;
        }
        Command::DockerBuild => {
            docker_build(&sh)?;
        }
        Command::DockerRun => {
            docker_run(&sh)?;
        }
        Command::InstallDeps => {
            install_deps(&sh)?;
        }
        Command::Fmt { check } => {
            fmt(&sh, check)?;
        }
        Command::Clippy => {
            clippy(&sh)?;
        }
        Command::Ci => {
            ci(&sh)?;
        }
    }

    Ok(())
}

fn project_root() -> Result<std::path::PathBuf> {
    let dir = env::current_dir()?;
    let mut current = dir.as_path();

    loop {
        if current.join("Cargo.toml").exists() && current.join("crates").exists() {
            return Ok(current.to_path_buf());
        }

        match current.parent() {
            Some(parent) => current = parent,
            None => anyhow::bail!("Could not find workspace root"),
        }
    }
}

fn build_wasm(sh: &Shell, release: bool, force: bool) -> Result<()> {
    use std::fs;

    // Check if rebuild is needed
    let pkg_dir = Path::new("crates/krokfmt-playground/pkg");
    let wasm_file = pkg_dir.join("krokfmt_playground_bg.wasm");

    let needs_rebuild = if force {
        println!("Force rebuild requested");
        true
    } else if !pkg_dir.exists() || !wasm_file.exists() {
        println!("WASM package not found, building...");
        true
    } else {
        // Check if any source files are newer than the built WASM
        let wasm_modified = fs::metadata(&wasm_file)?.modified()?;

        let src_files = [
            "crates/krokfmt-playground/src",
            "crates/krokfmt-playground/Cargo.toml",
            "crates/krokfmt/src", // Also check krokfmt since playground depends on it
            "crates/krokfmt/Cargo.toml",
        ];

        let mut needs_rebuild = false;
        for src_path in &src_files {
            if is_dir_newer_than(Path::new(src_path), &wasm_modified)? {
                println!("Source files changed, rebuilding WASM...");
                needs_rebuild = true;
                break;
            }
        }

        needs_rebuild
    };

    if !needs_rebuild {
        println!("✅ WASM module is up to date");

        return Ok(());
    }

    // Check if wasm-pack is installed
    if cmd!(sh, "which wasm-pack").run().is_err() {
        println!("wasm-pack not found. Installing...");
        install_wasm_pack(sh)?;
    }

    // Check if wasm32 target is installed
    if cmd!(sh, "rustup target list --installed")
        .read()?
        .lines()
        .find(|line| line.trim() == "wasm32-unknown-unknown")
        .is_none()
    {
        println!("Installing wasm32-unknown-unknown target...");
        cmd!(sh, "rustup target add wasm32-unknown-unknown").run()?;
    }

    println!("Building WASM module...");
    sh.change_dir("crates/krokfmt-playground");

    let profile = if release { "--release" } else { "--dev" };
    cmd!(sh, "wasm-pack build --target web --out-dir pkg {profile}")
        .run()
        .context("Failed to build WASM module")?;

    sh.change_dir("../..");
    println!("✅ WASM module built successfully");

    Ok(())
}

fn is_dir_newer_than(dir: &Path, reference_time: &SystemTime) -> Result<bool> {
    use std::fs;

    if dir.is_file() {
        let metadata = fs::metadata(dir)?;
        return Ok(metadata.modified()? > *reference_time);
    }

    for entry in fs::read_dir(dir)? {
        let entry = entry?;
        let path = entry.path();

        if path.is_dir() {
            // Skip target and node_modules directories
            if let Some(name) = path.file_name() {
                if name == "target" || name == "node_modules" || name == ".git" {
                    continue;
                }
            }
            if is_dir_newer_than(&path, reference_time)? {
                return Ok(true);
            }
        } else {
            // Check if this file is newer
            let metadata = entry.metadata()?;
            if metadata.modified()? > *reference_time {
                return Ok(true);
            }
        }
    }

    Ok(false)
}

fn build_web(sh: &Shell, release: bool) -> Result<()> {
    println!("Building web documentation...");

    // Check if npm is installed
    if cmd!(sh, "which npm").run().is_err() {
        anyhow::bail!("npm is required to build the web documentation. Please install Node.js.");
    }

    // Change to web directory
    sh.change_dir("crates/krokfmt-web");

    // Check if node_modules exists, if not run npm install
    if !std::path::Path::new("node_modules").exists() {
        println!("Installing npm dependencies...");
        cmd!(sh, "npm install")
            .run()
            .context("Failed to install npm dependencies")?;
    }

    // Build VitePress site
    println!("Building VitePress documentation...");
    cmd!(sh, "npm run build")
        .run()
        .context("Failed to build VitePress documentation")?;

    // Change back to root
    sh.change_dir("../..");

    println!("✅ Web documentation built successfully");
    Ok(())
}

fn test(sh: &Shell) -> Result<()> {
    println!("Running tests...");

    cmd!(sh, "cargo test --workspace")
        .run()
        .context("Failed to run tests")?;

    println!("✅ All tests passed");
    Ok(())
}

fn run_web(sh: &Shell, release: bool) -> Result<()> {
    println!("Starting web server on http://localhost:3000");

    // Check if npm is installed
    if cmd!(sh, "which npm").run().is_err() {
        anyhow::bail!("npm is required to run the web server. Please install Node.js.");
    }

    // Change to web directory
    sh.change_dir("crates/krokfmt-web");

    // Check if node_modules exists, if not run npm install
    if !std::path::Path::new("node_modules").exists() {
        println!("Installing npm dependencies...");
        cmd!(sh, "npm install")
            .run()
            .context("Failed to install npm dependencies")?;
    }

    // Run the development server
    cmd!(sh, "npm run dev")
        .run()
        .context("Failed to run web server")?;

    // Change back to root (though this won't be reached due to server running)
    sh.change_dir("../..");

    Ok(())
}

fn clean(sh: &Shell) -> Result<()> {
    println!("Cleaning build artifacts...");

    cmd!(sh, "cargo clean").run()?;

    let wasm_pkg = "crates/krokfmt-playground/pkg";
    if std::path::Path::new(wasm_pkg).exists() {
        cmd!(sh, "rm -rf {wasm_pkg}").run()?;
    }

    // Clean VitePress build artifacts
    let vitepress_dist = "crates/krokfmt-web/docs/.vitepress/dist";
    if std::path::Path::new(vitepress_dist).exists() {
        cmd!(sh, "rm -rf {vitepress_dist}").run()?;
    }

    // Clean node_modules if requested
    let node_modules = "crates/krokfmt-web/node_modules";
    if std::path::Path::new(node_modules).exists() {
        println!("Note: Run 'rm -rf crates/krokfmt-web/node_modules' to also clean npm dependencies");
    }

    println!("✅ Clean complete");
    Ok(())
}

fn docker_build(sh: &Shell) -> Result<()> {
    println!("Building Docker image...");

    cmd!(
        sh,
        "docker build -f deployment/docker/Dockerfile.web -t krokfmt-web:latest ."
    )
    .run()
    .context("Failed to build Docker image")?;

    println!("✅ Docker image built successfully");
    Ok(())
}

fn docker_run(sh: &Shell) -> Result<()> {
    println!("Running Docker container...");

    cmd!(sh, "docker run -p 3000:3000 krokfmt-web:latest")
        .run()
        .context("Failed to run Docker container")?;

    Ok(())
}

fn install_deps(sh: &Shell) -> Result<()> {
    println!("Installing dependencies...");

    // Install wasm32 target
    println!("Installing wasm32-unknown-unknown target...");
    cmd!(sh, "rustup target add wasm32-unknown-unknown").run()?;

    // Install wasm-pack
    install_wasm_pack(sh)?;

    println!("✅ Dependencies installed");
    Ok(())
}

fn install_wasm_pack(sh: &Shell) -> Result<()> {
    println!("Installing wasm-pack...");

    // Try to install via cargo first (faster)
    if cmd!(sh, "cargo install wasm-pack").run().is_ok() {
        return Ok(());
    }

    // Fallback to curl installer
    cmd!(
        sh,
        "curl https://rustwasm.github.io/wasm-pack/installer/init.sh -sSf | sh"
    )
    .run()
    .context("Failed to install wasm-pack")?;

    Ok(())
}

fn fmt(sh: &Shell, check: bool) -> Result<()> {
    println!("Formatting code...");

    let args = if check {
        vec!["fmt", "--all", "--", "--check"]
    } else {
        vec!["fmt", "--all"]
    };

    cmd!(sh, "cargo {args...}")
        .run()
        .context("Failed to format code")?;

    if check {
        println!("✅ Code formatting check passed");
    } else {
        println!("✅ Code formatted");
    }

    Ok(())
}

fn clippy(sh: &Shell) -> Result<()> {
    println!("Running clippy...");

    cmd!(
        sh,
        "cargo clippy --workspace --all-targets --all-features -- -D warnings"
    )
    .run()
    .context("Clippy found issues")?;

    println!("✅ Clippy passed");
    Ok(())
}

fn ci(sh: &Shell) -> Result<()> {
    println!("Running CI checks...");

    // Format check
    fmt(sh, true)?;

    // Clippy
    clippy(sh)?;

    // Tests
    test(sh)?;

    println!("✅ All CI checks passed");
    Ok(())
}
