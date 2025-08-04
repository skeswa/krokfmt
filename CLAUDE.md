# CLAUDE.md

This file provides guidance to Claude Code (claude.ai/code) when working with code in this repository.

## Overview

krokfmt is a highly opinionated, zero-configuration TypeScript code formatter written in Rust. It uses the SWC parser ecosystem and enforces strict formatting rules with no configuration options.

## IMPORTANT: Task-Based Workflow

**ALWAYS check TODO.md first before starting any work.** This project uses a task-based workflow where all work items are tracked in TODO.md. 

### Workflow Rules:
1. **Start here**: Read TODO.md and pick the TOP task from "Ready for Development"
2. **Move task**: Move your selected task to "In Progress" section
3. **Complete task**: Move to "Completed" with today's date when done
4. **Add tasks**: If you discover new work, add it to TODO.md in the appropriate priority section
5. **One at a time**: Only work on ONE task at a time

## Development Commands

```bash
# Run all tests
cargo test

# Run snapshot tests only
cargo test --test snapshot_tests

# Run a specific snapshot test
cargo test --test snapshot_tests test_fr1_1_default_imports

# Run tests with output
cargo test -- --nocapture

# Review snapshot changes interactively
cargo insta review

# Accept all snapshot changes
cargo insta accept

# Build debug version
cargo build

# Build release version
cargo build --release

# Run linter
cargo clippy --all-targets --all-features -- -D warnings

# Format Rust code
cargo fmt

# Run benchmarks
cargo bench

# Run specific benchmark
cargo bench --bench real_world_bench

# Run benchmarks and save baseline
cargo bench -- --save-baseline main

# Compare benchmarks against baseline
cargo bench -- --baseline main
```

## IMPORTANT: Post-Change Verification

**After EVERY code change, you MUST run the following commands in order:**

1. `cargo fmt` - Format the code according to Rust standards
2. `cargo clippy --all-targets --all-features -- -D warnings` - Check for common mistakes and improve code quality
3. `cargo test` - Ensure all tests pass

This is non-negotiable. These commands must be run after:
- Adding new code
- Modifying existing code
- Refactoring
- Adding or modifying tests

If any of these commands fail:
- `cargo fmt` - The changes will be applied automatically, commit them
- `cargo clippy --all-targets --all-features -- -D warnings` - Fix all warnings before proceeding
- `cargo test` - Fix failing tests before moving on

## Architecture Overview

The formatter follows a pipeline architecture:

1. **File Discovery** (`file_handler.rs`): Finds TypeScript files based on CLI args
2. **Parsing** (`parser.rs`): Uses SWC to parse TypeScript into AST with comments
3. **Comment Classification** (`comment_classifier.rs`): Identifies inline vs non-inline comments
4. **Selective Comment Handling** (`selective_comment_handler.rs`): Separates inline from non-inline comments
5. **Analysis** (`transformer.rs`): Analyzes imports and categorizes them
6. **Transformation** (`formatter.rs`): Applies formatting rules to AST (with inline comments preserved)
7. **Code Generation** (`codegen.rs`): Converts AST back to formatted code
8. **Comment Reinsertion** (`comment_reinserter.rs`): Reinserts non-inline comments at correct positions

### Key Design Decisions

- **Parallel Processing**: Uses Rayon for concurrent file processing
- **Zero Configuration**: No config files or options - formatting rules are hardcoded
- **AST-based**: Manipulates the AST directly rather than string manipulation
- **Import Categories**: External (node_modules), Absolute (@/~), Relative (./)
- **Selective Comment Preservation**: Inline comments stay in AST, others are extracted/reinserted
- **Two-Phase Formatting**: When source is available, uses selective preservation for better results

## Code Comment Style Guidelines

This codebase emphasizes high-quality comments that focus on **intent and context** rather than mere description. When adding comments:

### What Makes a Good Comment

1. **Explain the "why", not the "what"**: Code shows what is happening; comments should explain why decisions were made
2. **Provide historical context**: Mention design decisions, alternatives considered, or lessons learned
3. **Clarify non-obvious implications**: Explain side effects, performance considerations, or subtle interactions
4. **Document assumptions and constraints**: Make implicit knowledge explicit

### Examples of Good Comments

```rust
// Two-pass analysis is necessary because forward references are allowed
// in JavaScript. First we catalog all declarations, then we can accurately
// identify which identifier references are dependencies.

// Backup first, write second. This ordering ensures we never lose the original
// file if the write fails. The slight performance cost is worth the safety.

// We only track intra-module dependencies, not external imports or builtins.
// Self-references are excluded to avoid circular dependency false positives.
```

### What to Avoid

- Obvious comments that restate the code
- Comments that describe what a well-named function/variable already explains
- Outdated comments that no longer match the implementation
- TODO comments without context or ownership

### Where to Add Comments

- **Module/struct level**: Document the overall purpose and design philosophy
- **Complex algorithms**: Explain the approach and why it was chosen
- **Public APIs**: Document contracts, edge cases, and usage patterns
- **Non-obvious code**: Any code where the intent isn't immediately clear
- **Configuration/constants**: Explain why specific values were chosen

### Module Interactions

```
main.rs → file_handler.rs → (parallel) → parser.rs → two_phase_formatter.rs
                                              ↓
                                    selective_comment_handler.rs
                                              ↓
                                    comment_classifier.rs
                                              ↓
                                    transformer.rs → formatter.rs → codegen.rs
                                              ↓
                                    comment_reinserter.rs
```

### Critical Implementation Details

1. **Import Categorization** (transformer.rs:37): 
   - External: No prefix (e.g., 'react')
   - Absolute: Starts with @ or ~ (e.g., '@utils/helper')
   - Relative: Starts with ./ or ../ (e.g., './components')

2. **AST Visitor Pattern** (formatter.rs): Uses SWC's VisitMut trait for in-place AST modifications

3. **Import Group Spacing** (codegen.rs): Custom emitter adds empty lines between import categories

4. **Source Map Sharing** (parser.rs): Arc-wrapped SourceMap is shared between parser and codegen

5. **Comment Classification** (comment_classifier.rs): 
   - Inline: Within expressions (e.g., `/* comment */ value`)
   - Leading: Before declarations
   - Trailing: End of line
   - Standalone: Separated by blank lines

6. **Selective Two-Phase Formatting** (selective_two_phase_formatter.rs): Preserves inline comments naturally while handling others separately

## Testing Strategy

### Overview

- Use a TDD workflow (write tests first)
- Every requirement (see docs/requirements.md) must have snapshot tests
- All tests use the snapshot testing approach with [insta](https://insta.rs/)
- Test inputs are TypeScript files in `tests/fixtures/`
- Expected outputs are automatically managed in `tests/snapshots/`

### Test Structure

```
tests/
├── snapshot_tests.rs   # All requirement tests using snapshot approach
├── fixtures/          # Input TypeScript files
│   ├── fr1/          # FR1: FR1.X fixtures
│   ├── fr2/          # FR2: FR2.X fixtures
│   ├── fr3/          # FR3: FR3.X fixtures
│   └── frX/          # FRX: fixtures to do with functional requirement X (FRX.Y)
└── snapshots/        # Generated snapshots (auto-managed by insta)
```

### Fixture Organization

#### Naming Convention

Fixtures must follow this naming pattern:
```
{requirement}_{subrequirement}_{description}.input.ts
```

Examples:
- `fr1/1_1_default_imports.input.ts` - FR1.1 test for default imports
- `fr2/2_3_dependency_preservation.input.ts` - FR2.3 test for dependencies
- `fr3/3_6_jsx_properties.input.ts` - FR3.6 test for JSX props

#### Test Organization in snapshot_tests.rs

Tests must be organized by requirement groups with clear comments:

```rust
// FR1: Import/Export Organization Tests

#[test]
fn test_fr1_1_default_imports() {
    test_fixture("fr1/1_1_default_imports");
}

#[test]
fn test_fr1_2_categorization() {
    test_fixture("fr1/1_2_categorization");
}

// FR2: Member Visibility Ordering Tests

#[test]
fn test_fr2_1_export_detection() {
    test_fixture("fr2/2_1_export_detection");
}
```

### Creating New Tests

1. **Create the fixture file** following the naming convention:
   ```typescript
   // tests/fixtures/fr1/1_x_new_feature.input.ts
   // FR1.x: Description of what this tests
   import { z } from './z';
   import { a } from './a';
   ```

2. **Add the test** in the correct section of `snapshot_tests.rs`:
   ```rust
   #[test]
   fn test_fr1_x_new_feature() {
       test_fixture("fr1/1_x_new_feature");
   }
   ```

3. **Generate the snapshot**:
   ```bash
   cargo test --test snapshot_tests test_fr1_x_new_feature
   ```

4. **Review and accept**:
   ```bash
   cargo insta review
   ```

### Test Guidelines

1. **One fixture per sub-requirement**: Each sub-requirement should have at least one dedicated fixture
2. **Clear documentation**: Each fixture should start with a comment explaining what it tests
3. **Comprehensive examples**: Include edge cases and complex scenarios
4. **Isolated tests**: Each fixture should test one specific behavior
5. **Real-world code**: Use realistic TypeScript patterns in fixtures

### Benefits of Snapshot Testing

- **Visual diffs**: See exactly what changed in the formatter output
- **Easy updates**: One command to update all snapshots after intentional changes
- **Readable tests**: Input and output are plain TypeScript files
- **Fast feedback**: Quickly spot unintended formatting changes
- **Version controlled**: Snapshot changes are tracked in git

### Manual Testing

The `test_files/` directory contains sample TypeScript files for manual testing:
```bash
# Test a single file manually
cargo run -- test_files/sample.ts --stdout

# Test all sample files
cargo run -- test_files/
```

## Performance Benchmarking

The project includes performance benchmarks to measure formatting speed across different file sizes and complexities.

### Benchmark Structure

Benchmarks are located in `benches/`:
- `formatting_bench.rs` - Synthetic benchmarks with various file sizes
- `real_world_bench.rs` - Benchmarks using actual test fixtures

### Running Benchmarks

```bash
# Run all benchmarks
cargo bench

# Run specific benchmark suite
cargo bench --bench real_world_bench

# Save baseline for comparison
cargo bench -- --save-baseline my-baseline

# Compare against baseline
cargo bench -- --baseline my-baseline
```

### Benchmark Results

Results are saved in `target/criterion/` with HTML reports showing:
- Execution time distributions
- Performance comparisons between runs
- Throughput measurements (bytes/second)

### CI Integration

Benchmarks run automatically on:
- Every push to main branch
- Pull requests (with comparison against base branch)
- Manual workflow dispatch

Results are uploaded as GitHub Actions artifacts for review.

## Task Management

All tasks are tracked in TODO.md. The file contains:
- **In Progress**: Current task being worked on (max 1)
- **Ready for Development**: Prioritized task queue
- **Completed**: Finished tasks with completion dates

When implementing a task:
1. Write snapshot tests first (TDD approach)
2. Implement the minimum code to pass tests
3. Refactor if needed
4. Run `cargo test --test snapshot_tests` to verify
5. Review snapshots with `cargo insta review`
6. Move task to Completed

## Adding New Tasks

When you discover new work, add it to TODO.md with:
- Clear description of what needs to be done
- Reference to functional requirement (e.g., FR3.1)
- List of affected files
- Any dependencies on other tasks
