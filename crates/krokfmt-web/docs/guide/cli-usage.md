# CLI Usage

The krokfmt command-line interface is designed to be simple and intuitive.

## Basic Usage

```bash
krokfmt [OPTIONS] [FILES/DIRECTORIES...]
```

## Common Commands

### Format Files

Format TypeScript files in place:

```bash
# Format all files in src directory
krokfmt src/

# Format specific files
krokfmt index.ts app.tsx

# Format all TypeScript files recursively
krokfmt .
```

### Check Mode

Verify files are formatted without modifying them:

```bash
# Check all files
krokfmt --check .

# Returns exit code 0 if formatted, 1 if not
```

### Print to stdout

Preview formatting without modifying files:

```bash
# Print formatted version to console
krokfmt --stdout file.ts
```

## Options

| Option | Description |
|--------|-------------|
| `--check` | Check if files are formatted (exit 1 if not) |
| `--stdout` | Print formatted output to stdout |
| `--version` | Display version information |
| `--help` | Show help message |

## File Selection

### Supported Extensions

krokfmt automatically detects and formats files with these extensions:
- `.ts` - TypeScript files
- `.tsx` - TypeScript with JSX
- `.mts` - ES module TypeScript
- `.cts` - CommonJS module TypeScript

### Glob Patterns

Use shell glob patterns to select files:

```bash
# All TypeScript files in components
krokfmt src/components/*.ts

# All TSX files recursively
krokfmt '**/*.tsx'

# Multiple patterns
krokfmt 'src/**/*.ts' 'tests/**/*.ts'
```

### Ignoring Files

krokfmt respects `.gitignore` patterns by default. To format ignored files:

```bash
# Format all files including gitignored
krokfmt --no-ignore .
```

## Exit Codes

| Code | Description |
|------|-------------|
| 0 | Success |
| 1 | Files need formatting (check mode) or formatting error |
| 2 | Invalid arguments or file not found |

## Environment Variables

### Logging

Control log output with `KROKFMT_LOG`:

```bash
# Show debug logs
KROKFMT_LOG=debug krokfmt .

# Only show errors
KROKFMT_LOG=error krokfmt .
```

Levels: `error`, `warn`, `info`, `debug`, `trace`

### Color Output

Disable colored output:

```bash
NO_COLOR=1 krokfmt .
```

## Integration Examples

### Pre-commit Hook

`.git/hooks/pre-commit`:

```bash
#!/bin/sh
# Format staged TypeScript files
git diff --cached --name-only --diff-filter=ACM | \
  grep -E '\.(ts|tsx)$' | \
  xargs krokfmt

# Re-stage formatted files
git diff --cached --name-only --diff-filter=ACM | \
  grep -E '\.(ts|tsx)$' | \
  xargs git add
```

### NPM Script

`package.json`:

```json
{
  "scripts": {
    "format": "krokfmt src/",
    "format:check": "krokfmt --check src/"
  }
}
```

### Watch Mode

Use with file watchers:

```bash
# With watchman
watchman-make -p 'src/**/*.ts' 'src/**/*.tsx' --run 'krokfmt'

# With nodemon
nodemon --watch src --ext ts,tsx --exec krokfmt
```

## Performance Tips

1. **Format directories** instead of individual files for better performance
2. **Use specific paths** instead of formatting the entire project
3. **Run in parallel** CI jobs for large codebases
4. **Cache results** in CI when using check mode

## Troubleshooting

### Command Not Found

Ensure krokfmt is in your PATH:

```bash
# Check installation
which krokfmt

# Add to PATH (if installed locally)
export PATH="$PATH:$HOME/.cargo/bin"
```

### Permission Denied

Make the binary executable:

```bash
chmod +x /path/to/krokfmt
```

### Out of Memory

For very large files, increase memory limits:

```bash
ulimit -s unlimited
krokfmt large-file.ts
```