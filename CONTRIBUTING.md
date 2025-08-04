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
7. Run linting: `cargo clippy --all-targets --all-features -- -D warnings`
8. Commit your changes with clear, descriptive messages
9. Push to your fork and create a pull request

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