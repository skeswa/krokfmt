# Getting Started

## Installation

krokfmt can be installed in several ways:

### Using Cargo

If you have Rust installed, you can install krokfmt using cargo:

```bash
cargo install krokfmt
```

### Pre-built Binaries

Download pre-built binaries from the [GitHub releases page](https://github.com/skeswa/krokfmt/releases).

#### macOS/Linux

```bash
# Download the binary (replace VERSION and PLATFORM)
curl -L https://github.com/skeswa/krokfmt/releases/download/VERSION/krokfmt-PLATFORM -o krokfmt

# Make it executable
chmod +x krokfmt

# Move to a directory in your PATH
sudo mv krokfmt /usr/local/bin/
```

#### Windows

Download the Windows executable from the releases page and add it to your PATH.

### From Source

```bash
# Clone the repository
git clone https://github.com/skeswa/krokfmt.git
cd krokfmt

# Build and install
cargo install --path crates/krokfmt
```

## Basic Usage

### Format Files

Format TypeScript files in place:

```bash
# Format all TypeScript files in a directory
krokfmt src/

# Format specific files
krokfmt src/index.ts src/components/*.tsx

# Format all TypeScript files recursively
krokfmt .
```

### Check Mode

Check if files are properly formatted without modifying them (useful for CI):

```bash
krokfmt --check src/

# Exit code will be non-zero if files need formatting
```

### Print to stdout

Print formatted output without modifying files:

```bash
krokfmt --stdout src/index.ts
```

### Watch Mode

Automatically format files when they change:

```bash
krokfmt --watch src/
```

## Editor Integration

### VS Code

Install the krokfmt VS Code extension (coming soon).

### Vim/Neovim

Add to your config:

```vim
" Format on save
autocmd BufWritePre *.ts,*.tsx !krokfmt %
```

### Other Editors

krokfmt can be integrated with any editor that supports external formatters. Configure your editor to run `krokfmt <file>` on save.

## CI/CD Integration

### GitHub Actions

```yaml
name: Format Check

on: [push, pull_request]

jobs:
  format:
    runs-on: ubuntu-latest
    steps:
      - uses: actions/checkout@v3
      
      - name: Install krokfmt
        run: |
          curl -L https://github.com/skeswa/krokfmt/releases/latest/download/krokfmt-linux-x86_64 -o krokfmt
          chmod +x krokfmt
          sudo mv krokfmt /usr/local/bin/
      
      - name: Check formatting
        run: krokfmt --check .
```

### Pre-commit Hook

Add to `.git/hooks/pre-commit`:

```bash
#!/bin/sh
# Format staged TypeScript files
git diff --cached --name-only --diff-filter=ACMR | grep -E '\.(ts|tsx)$' | xargs krokfmt

# Re-stage formatted files
git diff --cached --name-only --diff-filter=ACMR | grep -E '\.(ts|tsx)$' | xargs git add
```

## Next Steps

- Learn about [Import Organization](/guide/import-organization)
- Understand [Member Ordering](/guide/member-ordering)
- Explore [Comment Preservation](/guide/comment-preservation)
- Read about [TypeScript Support](/guide/typescript-support)