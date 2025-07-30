# TODO.md - Task Queue for krokfmt

Tasks are ordered by priority. Always work on tasks from the top of this list first.

## In Progress
<!-- Move ONE task here when you start working on it -->

## Ready for Development

### High Priority

### Medium Priority

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