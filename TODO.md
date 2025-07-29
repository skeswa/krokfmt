# TODO.md - Task Queue for krokfmt

Tasks are ordered by priority. Always work on tasks from the top of this list first.

## In Progress
<!-- Move ONE task here when you start working on it -->

## Ready for Development

### High Priority

### Medium Priority

5. **Implement member visibility ordering (FR2.2)**
   - Move exported members toward the top of the file
   - Build dependency graph to preserve correctness
   - File: `src/formatter.rs`

### Low Priority

7. **Add performance benchmarks**
   - Create benchmark suite for formatting speed
   - Test against large TypeScript files
   - Add to CI pipeline
   - File: `benches/`

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

## Task Guidelines

1. **Adding Tasks**: Add new tasks in the appropriate priority section
2. **Starting Work**: Move ONE task to "In Progress" when you begin
3. **Completing Tasks**: Move to "Completed" with date when done
4. **Task Format**: Include description, subtasks, and relevant files
5. **Dependencies**: Note if a task depends on another task