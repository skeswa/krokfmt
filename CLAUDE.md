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

# Run a specific test
cargo test test_import_organization_complete

# Run tests with output
cargo test -- --nocapture

# Build debug version
cargo build

# Build release version
cargo build --release

# Run linter
cargo clippy

# Format Rust code
cargo fmt
```

## IMPORTANT: Post-Change Verification

**After EVERY code change, you MUST run the following commands in order:**

1. `cargo fmt` - Format the code according to Rust standards
2. `cargo clippy` - Check for common mistakes and improve code quality
3. `cargo test` - Ensure all tests pass

This is non-negotiable. These commands must be run after:
- Adding new code
- Modifying existing code
- Refactoring
- Adding or modifying tests

If any of these commands fail:
- `cargo fmt` - The changes will be applied automatically, commit them
- `cargo clippy` - Fix all warnings before proceeding
- `cargo test` - Fix failing tests before moving on

## Architecture Overview

The formatter follows a pipeline architecture:

1. **File Discovery** (`file_handler.rs`): Finds TypeScript files based on CLI args
2. **Parsing** (`parser.rs`): Uses SWC to parse TypeScript into AST
3. **Analysis** (`transformer.rs`): Analyzes imports and categorizes them
4. **Transformation** (`formatter.rs`): Applies formatting rules to AST
5. **Code Generation** (`codegen.rs`): Converts AST back to formatted code

### Key Design Decisions

- **Parallel Processing**: Uses Rayon for concurrent file processing
- **Zero Configuration**: No config files or options - formatting rules are hardcoded
- **AST-based**: Manipulates the AST directly rather than string manipulation
- **Import Categories**: External (node_modules), Absolute (@/~), Relative (./)

### Module Interactions

```
main.rs → file_handler.rs → (parallel) → parser.rs → transformer.rs → formatter.rs → codegen.rs
```

### Critical Implementation Details

1. **Import Categorization** (transformer.rs:37): 
   - External: No prefix (e.g., 'react')
   - Absolute: Starts with @ or ~ (e.g., '@utils/helper')
   - Relative: Starts with ./ or ../ (e.g., './components')

2. **AST Visitor Pattern** (formatter.rs): Uses SWC's VisitMut trait for in-place AST modifications

3. **Import Group Spacing** (codegen.rs): Custom emitter adds empty lines between import categories

4. **Source Map Sharing** (parser.rs): Arc-wrapped SourceMap is shared between parser and codegen

## Testing Strategy

- Use a TDD workflow (write tests first)
- Every requirement (see docs/requirements.md) should have a corresponding test
- **Unit Tests**: Each module has tests covering its specific functionality
- **Integration Tests** (`tests/integration_tests.rs`): End-to-end formatting tests
- **Test Fixtures** (`test_files/`): Sample TypeScript files for manual testing

## Task Management

All tasks are tracked in TODO.md. The file contains:
- **In Progress**: Current task being worked on (max 1)
- **Ready for Development**: Prioritized task queue
- **Completed**: Finished tasks with completion dates

When implementing a task:
1. Write tests first (TDD approach)
2. Implement the minimum code to pass tests
3. Refactor if needed
4. Update integration tests
5. Run `cargo test` to ensure nothing is broken
6. Move task to Completed

## Adding New Tasks

When you discover new work, add it to TODO.md with:
- Clear description of what needs to be done
- Reference to functional requirement (e.g., FR3.1)
- List of affected files
- Any dependencies on other tasks
