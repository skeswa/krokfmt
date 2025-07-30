# TODO.md - Task Queue for krokfmt

Tasks are ordered by priority. Always work on tasks from the top of this list first.

## Requirements Status Summary

### Functional Requirements
- ✅ FR1: Import/Export Organization (All sub-requirements tested)
- ✅ FR2: Member Visibility Ordering (All sub-requirements tested)
- ✅ FR3: Alphabetical Sorting (All sub-requirements tested)
- ⚠️  FR4: CLI Interface (Implemented but needs comprehensive tests)
- ❌ FR5: File Handling (Not fully implemented/tested)
- ⚠️  FR6: Comment Handling (Implemented but has issues)

### Non-Functional Requirements
- ⚠️  NFR1: Performance (Benchmarks exist but need specific metrics)
- ✅ NFR2: Correctness (Mostly implemented)
- ❌ NFR3: Robustness (Error handling needs work)
- ❌ NFR4: Developer Experience (Distribution/platform support pending)
- ❌ NFR5: Maintainability (Coverage reporting not set up)

## In Progress
<!-- Move ONE task here when you start working on it -->

## Ready for Development

### High Priority

1. **Fix comment preservation issues (FR6: NFR2.2)**
   - Comments are now captured and emitted, but have issues:
   - Fix comment indentation (some comments lose proper indentation)
   - Ensure comments move with their associated code during reordering
   - Fix floating comment positioning
   - Handle comment associations correctly in formatter
   - Files: `src/formatter.rs`, `src/codegen.rs`

### Medium Priority

2. **Implement and test FR4 requirements (CLI Interface)**
   - FR4.1: Single File Processing - Already implemented
   - FR4.2: Directory Processing - Already implemented
   - FR4.3: Glob Pattern Support - Already implemented
   - FR4.4: Check Mode - Already implemented
   - FR4.5: Stdout Mode - Already implemented
   - FR4.6: Version Display - Already implemented
   - FR4.7: Help Display - Already implemented
   - Add comprehensive tests for all CLI features
   - Files: `tests/integration_tests.rs`, `src/main.rs`

3. **Implement and test FR5 requirements (File Handling)**
   - FR5.1: Encoding Preservation (UTF-8, BOM detection)
   - FR5.2: Line Ending Preservation (LF/CRLF)
   - FR5.3: Backup Creation (.bak files)
   - FR5.4: File Type Support (.ts, .tsx, .mts, .cts)
   - Create test fixtures for each file handling scenario
   - Files: `src/file_handler.rs`, `tests/`

4. **Implement and test NFR1 requirements (Performance)**
   - NFR1.1: Processing Speed benchmarks (1000 lines < 100ms)
   - NFR1.2: Parallel Processing verification
   - NFR1.3: Memory Efficiency tests
   - NFR1.4: Large File Support (up to 10MB)
   - Enhance existing benchmarks with specific metrics
   - Files: `benches/`, performance test suite

5. **Implement and test NFR3 requirements (Robustness)**
   - NFR3.1: Error Recovery (handle syntax errors gracefully)
   - NFR3.2: Error Messaging (clear, actionable messages)
   - NFR3.3: Partial Formatting (format valid portions)
   - NFR3.4: Circular Dependency Handling (already partially done)
   - Create error handling test suite
   - Files: `src/error.rs` (new), `tests/error_tests.rs` (new)

6. **Implement and test NFR4 requirements (Developer Experience)**
   - NFR4.1: Distribution as single binary
   - NFR4.2: Cross-Platform Support verification
   - NFR4.3: Zero Dependencies verification
   - NFR4.4: CI/CD Integration features
   - Set up release pipeline and platform tests
   - Files: `.github/workflows/`, build scripts

7. **Implement and test NFR5 requirements (Maintainability)**
   - NFR5.1: Test Coverage measurement (90% target)
   - NFR5.2: TDD Methodology documentation
   - NFR5.3: Modular Architecture verification
   - NFR5.4: Error Handling patterns
   - Set up coverage reporting and architecture docs
   - Files: `Cargo.toml`, documentation

### Low Priority

8. **Implement source map support**
   - Generate source maps during code generation
   - Useful for debugging transformed code
   - File: `src/codegen.rs`

9. **Add --fix mode for common issues**
   - Auto-fix missing semicolons
   - Convert var to let/const
   - Remove unused imports

## Completed
<!-- Move completed tasks here with completion date -->

- ✅ Implement comment preservation (FR6: NFR2.2) - Initial implementation (2025-07-30)
  - Added detailed requirements for comment handling (FR6.1-6.6)
  - Created comprehensive test cases for all comment types
  - Updated parser to capture comments using SingleThreadedComments
  - Updated codegen to emit comments in correct positions
  - Comments are now preserved but need fixes for positioning and associations

- ✅ Set up project structure and dependencies (2024-01-29)
- ✅ Implement import organization and sorting (2024-01-29)
- ✅ Implement basic object property sorting (2024-01-29)
- ✅ Create comprehensive test infrastructure (2024-01-29)
- ✅ Implement function argument sorting (FR3.1) (2025-07-29)
- ✅ Implement class member sorting (FR3.3) (2025-07-29)
- ✅ Implement type member sorting (FR3.4) (2025-07-29)
- ✅ Implement enum member sorting (FR3.5) (2025-07-29)
- ✅ Add JSX property sorting (FR3.6) (2025-07-29)
- ✅ Implement member visibility ordering (FR2.1 & FR2.2) (2025-07-29)
  - Export detection to identify exported vs non-exported members
  - Export prioritization to move exported members toward top
  - Dependency graph to preserve correctness when reordering
- ✅ Implement all FR1.x requirements (2025-07-29)
  - FR1.1: Import Statement Parsing - Parse all import syntaxes
  - FR1.2: Import Categorization - External/Absolute/Relative
  - FR1.3: Import Sorting - Alphabetically within categories
  - FR1.4: Import Positioning - All imports at top of file
  - FR1.5: Import Group Separation - Empty lines between groups
  - FR1.6: Import Syntax Preservation - Maintain exact syntax
- ✅ Add performance benchmarks (2025-07-29)
  - Created benchmark suite for formatting speed
  - Added benchmarks for different file sizes and complexities
  - Integrated benchmarks into CI pipeline
  - Created both synthetic and real-world benchmarks
- ✅ Implement FR2.3: Dependency Preservation (2025-07-29)
  - Enhanced dependency analyzer to handle all TypeScript patterns
  - Added support for destructuring, namespaces, and computed properties
  - Fixed circular dependency handling to prevent stack overflow
  - Created comprehensive test suite with 8 additional test cases
- ✅ Implement FR2.4: Intelligent Grouping (2025-07-29)
  - Keeps type definitions with their type guards
  - Groups interfaces with their implementations
  - Maintains related functions and classes together
  - Added advanced grouping patterns for state management and API clients
- ✅ Complete all FR2 requirements (2025-07-29)
  - FR2.1: Export Detection - Identifies all export patterns
  - FR2.2: Export Prioritization - Moves exports to top while respecting dependencies
  - FR2.3: Dependency Preservation - Never breaks code functionality
  - FR2.4: Intelligent Grouping - Keeps related items together
  - Created 16 comprehensive tests covering all edge cases

## Task Guidelines

1. **Adding Tasks**: Add new tasks in the appropriate priority section
2. **Starting Work**: Move ONE task to "In Progress" when you begin
3. **Completing Tasks**: Move to "Completed" with date when done
4. **Task Format**: Include description, subtasks, and relevant files
5. **Dependencies**: Note if a task depends on another task