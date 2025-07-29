# Contributing to krokfmt

Thank you for your interest in contributing to krokfmt! This document provides guidelines and instructions for contributing.

## Code of Conduct

By participating in this project, you agree to abide by our code of conduct: be respectful, inclusive, and constructive.

## How to Contribute

### Reporting Bugs

1. Check if the bug has already been reported in the [issues](https://github.com/skeswa/krokfmt/issues)
2. If not, create a new issue using the bug report template
3. Include:
   - A clear description of the bug
   - Steps to reproduce
   - Expected vs actual behavior
   - Your environment details

### Suggesting Features

1. Check if the feature has already been suggested
2. Create a new issue using the feature request template
3. Explain why this feature would be useful

### Submitting Code

1. Fork the repository
2. Create a new branch from `main`: `git checkout -b feature/your-feature-name`
3. Make your changes following our coding standards
4. Add tests for your changes
5. Ensure all tests pass: `cargo test`
6. Run formatting: `cargo fmt`
7. Run linting: `cargo clippy`
8. Commit your changes with clear, descriptive messages
9. Push to your fork and create a pull request

## Development Setup

1. Install Rust (1.70.0 or later)
2. Clone the repository
3. Run `cargo build` to build the project
4. Run `cargo test` to run tests

## Testing

We use Test-Driven Development (TDD). Please:
1. Write tests first
2. Make them fail
3. Write the minimum code to make them pass
4. Refactor if needed

### Running Tests

```bash
# Run all tests
cargo test

# Run specific test
cargo test test_name

# Run tests with output
cargo test -- --nocapture

# Run tests in release mode
cargo test --release
```

## Code Style

- Follow Rust naming conventions
- Use `cargo fmt` before committing
- Fix all `cargo clippy` warnings
- Add documentation comments for public APIs
- Keep functions small and focused

## Project Structure

```
src/
├── main.rs           # CLI entry point
├── lib.rs            # Library entry point
├── parser.rs         # TypeScript parsing
├── formatter.rs      # Core formatting logic
├── transformer.rs    # Import analysis
├── codegen.rs        # Code generation
└── file_handler.rs   # File operations

tests/
└── integration_tests.rs  # End-to-end tests

docs/
└── requirements.md   # Detailed requirements
```

## Adding New Features

1. Check TODO.md for planned features
2. Discuss major changes in an issue first
3. Follow the existing patterns in the codebase
4. Update documentation as needed
5. Add comprehensive tests

## Git Workflow

1. Keep commits atomic and focused
2. Write clear commit messages
3. Rebase on main before creating PR
4. Keep PR scope manageable

## Questions?

Feel free to ask questions in:
- GitHub issues
- Pull request discussions

Thank you for contributing!