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

### Overview

- Use a TDD workflow (write tests first)
- Every requirement (see docs/requirements.md) should have dedicated tests
- Tests are organized by functional requirement groups for clarity

### Test Structure

```
tests/
├── fr1_tests.rs        # FR1: Import/Export Organization (26 tests)
├── fr2_tests.rs        # FR2: Member Visibility Ordering (16 tests)
├── fr3_tests.rs        # FR3: Alphabetical Sorting (24 tests)
└── integration_tests.rs # End-to-end integration tests
```

### Requirement Test Files

Each `fr*_tests.rs` file contains dedicated tests for specific requirements:

- **fr1_tests.rs**: Import/Export Organization
  - FR1.1: Import Statement Parsing (7 tests)
  - FR1.2: Import Categorization (4 tests)
  - FR1.3: Import Sorting (4 tests)
  - FR1.4: Import Positioning (3 tests)
  - FR1.5: Import Group Separation (4 tests)
  - FR1.6: Import Syntax Preservation (4 tests)

- **fr2_tests.rs**: Member Visibility Ordering
  - FR2.1: Export Detection (6 tests)
  - FR2.2: Export Prioritization (2 tests)
  - FR2.3: Dependency Preservation (5 tests)
  - FR2.4: Intelligent Grouping (3 tests)

- **fr3_tests.rs**: Alphabetical Sorting
  - FR3.1: Function Argument Sorting (5 tests)
  - FR3.2: Object Property Sorting (4 tests)
  - FR3.3: Class Member Sorting (4 tests)
  - FR3.4: Type Member Sorting (4 tests)
  - FR3.5: Enum Member Sorting (3 tests)
  - FR3.6: JSX Property Sorting (4 tests)

### Test Guidelines

1. **One Test Per Requirement**: Each test should target exactly one requirement
2. **Clear Test Names**: Test names must clearly indicate which requirement they verify (e.g., `test_fr1_1_parse_default_imports`)
3. **Ordered Tests**: Tests are listed in order of requirement declaration
4. **Isolated Testing**: Each test should be independent and not rely on other tests
5. **Comprehensive Coverage**: Every sub-requirement must have at least one dedicated test

### Running Tests

```bash
# Run all tests
cargo test

# Run tests for specific requirement group
cargo test --test fr1_tests  # Import/Export tests
cargo test --test fr2_tests  # Member Visibility tests
cargo test --test fr3_tests  # Alphabetical Sorting tests

# Run a specific test
cargo test test_fr1_1_parse_default_imports

# Run tests with output
cargo test -- --nocapture
```

### Other Test Types

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
