# krokfmt Design Document

## Table of Contents
1. [Executive Summary](#executive-summary)
2. [System Architecture](#system-architecture)
3. [Core Components](#core-components)
4. [Data Flow](#data-flow)
5. [Detailed Component Design](#detailed-component-design)
6. [Formatting Rules Engine](#formatting-rules-engine)
7. [Performance Considerations](#performance-considerations)
8. [Error Handling Strategy](#error-handling-strategy)
9. [Testing Architecture](#testing-architecture)
10. [Future Extensibility](#future-extensibility)

## Executive Summary

krokfmt is a highly opinionated, zero-configuration TypeScript code formatter designed to enforce consistent code organization principles. Built in Rust for maximum performance, it leverages the SWC (Speedy Web Compiler) ecosystem for TypeScript parsing and code generation.

### Key Design Principles
- **Zero Configuration**: No options or configuration files - one true way to format
- **Performance First**: Rust implementation with parallel processing
- **Semantic Preservation**: Never changes program behavior
- **Predictable Output**: Deterministic formatting results
- **Progressive Enhancement**: Format what's possible, report what's not

## System Architecture

```
┌─────────────────────────────────────────────────────────────────────┐
│                           CLI Interface                              │
│                        (clap-based parser)                          │
└─────────────────────┬───────────────────────────┬──────────────────┘
                      │                           │
                      ▼                           ▼
┌─────────────────────────────────┐ ┌────────────────────────────────┐
│        File Handler             │ │      Progress Reporter         │
│   - File discovery              │ │   - Colored output             │
│   - Glob expansion              │ │   - Error aggregation          │
│   - Backup management           │ │   - Exit code handling         │
└─────────────────┬───────────────┘ └────────────────────────────────┘
                  │
                  ▼
┌─────────────────────────────────────────────────────────────────────┐
│                    Parallel Processing Engine                        │
│                         (Rayon-based)                               │
└─────────────────────┬───────────────────────────────────────────────┘
                      │
                      ▼
┌─────────────────────────────────────────────────────────────────────┐
│                      Format Pipeline                                 │
│  ┌─────────┐  ┌──────────┐  ┌───────────┐  ┌──────────────────┐   │
│  │ Parser  │→ │ Analyzer │→ │Transformer│→ │ Code Generator   │   │
│  │  (SWC)  │  │          │  │           │  │     (SWC)        │   │
│  └─────────┘  └──────────┘  └───────────┘  └──────────────────┘   │
└─────────────────────────────────────────────────────────────────────┘
```

### Component Interaction Diagram

```mermaid
graph TB
    CLI[CLI Interface<br/>main.rs] --> FH[File Handler<br/>file_handler.rs]
    FH --> |discovers files| PP[Parallel Processor<br/>Rayon]
    PP --> |per file| FP[Format Pipeline]
    
    subgraph FP[Format Pipeline]
        P[Parser<br/>parser.rs] --> A[Analyzer<br/>transformer.rs]
        A --> F[Formatter<br/>formatter.rs]
        F --> CG[Code Generator<br/>codegen.rs]
    end
    
    CG --> |formatted code| FH2[File Handler<br/>Write]
    FH2 --> R[Reporter<br/>Colored Output]
    
    style CLI fill:#f9f,stroke:#333,stroke-width:4px
    style FP fill:#bbf,stroke:#333,stroke-width:2px
```

## Core Components

### 1. CLI Interface (`main.rs`)
Responsible for user interaction and orchestration:
- Argument parsing using `clap`
- File discovery coordination
- Progress reporting
- Exit code management

### 2. File Handler (`file_handler.rs`)
Manages all file system operations:
- TypeScript file discovery (`.ts`, `.tsx`, `.mts`, `.cts`)
- Directory traversal with exclusions (node_modules, hidden dirs)
- Glob pattern expansion
- Backup file creation
- Atomic file writing

### 3. Parser Module (`parser.rs`)
Wraps SWC's TypeScript parser:
- Configures parser for TypeScript/TSX syntax
- Manages source maps for accurate error reporting
- Handles syntax error recovery
- Supports all modern TypeScript features

### 4. Analyzer Module (`transformer.rs`)
Analyzes AST to extract formatting information:
- Import/export detection and categorization
- Dependency graph construction
- Member visibility analysis
- Sortable element identification

### 5. Transformer Module (`formatter.rs`)
Applies formatting rules to the AST:
- Import reorganization
- Property sorting
- Member reordering
- AST mutation with semantic preservation

### 6. Code Generator (`codegen.rs`)
Converts formatted AST back to source code:
- Custom emitter for import grouping
- Comment preservation
- Whitespace management
- Source map generation (future)

### Module Dependency Graph

```mermaid
graph BT
    main[main.rs] --> lib[lib.rs]
    lib --> parser[parser.rs]
    lib --> transformer[transformer.rs]
    lib --> formatter[formatter.rs]
    lib --> codegen[codegen.rs]
    lib --> file_handler[file_handler.rs]
    
    formatter --> transformer
    formatter --> parser
    codegen --> transformer
    main --> file_handler
    main --> parser
    main --> formatter
    main --> codegen
    
    parser --> swc[swc_ecma_parser]
    codegen --> swc2[swc_ecma_codegen]
    transformer --> swc3[swc_ecma_ast]
```

## Data Flow

```
┌──────────────┐     ┌──────────────┐     ┌──────────────┐
│  Input File  │ --> │   Parser     │ --> │     AST      │
│   (*.ts)     │     │              │     │              │
└──────────────┘     └──────────────┘     └──────┬───────┘
                                                  │
                                                  ▼
┌──────────────┐     ┌──────────────┐     ┌──────────────┐
│ Output File  │ <-- │   Codegen    │ <-- │ Transformed  │
│   (*.ts)     │     │              │     │     AST      │
└──────────────┘     └──────────────┘     └──────┬───────┘
                                                  ▲
                                                  │
                     ┌──────────────┐     ┌──────────────┐
                     │ Transformer  │ <-- │   Analyzer   │
                     │              │     │              │
                     └──────────────┘     └──────────────┘
```

### AST Transformation Pipeline

```mermaid
sequenceDiagram
    participant S as Source Code
    participant P as Parser
    participant AST as AST
    participant A as Analyzer
    participant T as Transformer
    participant CG as CodeGen
    participant O as Output
    
    S->>P: Parse TypeScript
    P->>AST: Create AST
    AST->>A: Visit Nodes
    A->>A: Categorize Imports
    A->>A: Analyze Dependencies
    A->>T: Provide Metadata
    T->>AST: Mutate AST
    Note over T,AST: - Reorder imports<br/>- Sort properties<br/>- Arrange members
    AST->>CG: Generate Code
    CG->>O: Formatted Source
```

## Detailed Component Design

### Parser Configuration

```rust
pub struct TypeScriptParser {
    pub source_map: Arc<SourceMap>,
}

impl TypeScriptParser {
    pub fn parse(&self, source: &str, filename: &str) -> Result<Module> {
        let syntax = Syntax::Typescript(TsConfig {
            tsx: filename.ends_with(".tsx"),
            decorators: true,
            no_early_errors: true,
            ..Default::default()
        });
        // ... parsing logic
    }
}
```

Key features:
- Automatic TSX detection based on file extension
- Decorator support for modern TypeScript
- Error recovery for partial formatting
- Source map integration for accurate positioning

### Import Categorization Logic

```
                    Import Path Analysis
                           │
            ┌──────────────┼──────────────┐
            │              │              │
            ▼              ▼              ▼
    Starts with ./    Starts with     Everything
    or ../            @ or ~          else
            │              │              │
            ▼              ▼              ▼
        Relative      Absolute       External
        Import        Import         Import
```

Categories are determined by path prefix:
- **External**: No special prefix (e.g., `react`, `lodash/debounce`)
- **Absolute**: Starts with `@` or `~` (e.g., `@utils/helper`)
- **Relative**: Starts with `./` or `../` (e.g., `./components/Button`)

### Import Processing Flow

```mermaid
flowchart LR
    S[Source File] --> P[Parse AST]
    P --> E[Extract Imports]
    E --> C{Categorize}
    
    C -->|starts with ./ or ../| REL[Relative]
    C -->|starts with @ or ~| ABS[Absolute]
    C -->|no prefix| EXT[External]
    
    REL --> SR[Sort Alphabetically]
    ABS --> SA[Sort Alphabetically]
    EXT --> SE[Sort Alphabetically]
    
    SR --> G[Group with<br/>Empty Lines]
    SA --> G
    SE --> G
    
    G --> O[Output]
```

### AST Transformation Pipeline

```rust
pub struct FormatterVisitor;

impl VisitMut for FormatterVisitor {
    fn visit_mut_object_lit(&mut self, obj: &mut ObjectLit) {
        self.sort_object_props(&mut obj.props);
        obj.visit_mut_children_with(self);
    }
    
    fn visit_mut_call_expr(&mut self, call: &mut CallExpr) {
        self.sort_arguments(&mut call.args);
        call.visit_mut_children_with(self);
    }
    
    // Additional visitors for other sortable constructs
}
```

The visitor pattern allows for:
- Deep AST traversal
- In-place mutations
- Recursive formatting
- Extensible rule application

### Data Structure Relationships

```mermaid
classDiagram
    class TypeScriptParser {
        +source_map: Arc~SourceMap~
        +parse(source, filename): Result~Module~
    }
    
    class ImportInfo {
        +category: ImportCategory
        +path: String
        +import_decl: ImportDecl
    }
    
    class ImportCategory {
        <<enumeration>>
        External
        Absolute
        Relative
    }
    
    class KrokFormatter {
        +format(module): Result~Module~
    }
    
    class CodeGenerator {
        +source_map: Lrc~SourceMap~
        +generate(module): Result~String~
    }
    
    ImportInfo --> ImportCategory
    TypeScriptParser --> Module
    KrokFormatter --> Module
    CodeGenerator --> Module
```

## Formatting Rules Engine

### Rule Categories

1. **Structural Rules** (affect file structure)
   - Import hoisting
   - Export grouping
   - Member visibility ordering

2. **Ordering Rules** (affect element order)
   - Import sorting by category and path
   - Object property alphabetization
   - Class member sorting

3. **Spacing Rules** (affect whitespace)
   - Empty lines between import groups
   - Consistent spacing around elements

### Rule Application Order

```
1. Extract imports/exports
        │
        ▼
2. Categorize and sort imports
        │
        ▼
3. Apply structural transformations
        │
        ▼
4. Apply ordering rules recursively
        │
        ▼
5. Generate code with spacing rules
```

### Conflict Resolution

When rules conflict, precedence is:
1. Semantic preservation (never break code)
2. Structural rules
3. Ordering rules
4. Spacing rules

### Import Grouping Visualization

```mermaid
graph TD
    subgraph "Original File"
        O1[import './local']
        O2[import 'react']
        O3[import '@/utils']
        O4[const code = 1]
        O5[import 'axios']
    end
    
    subgraph "After Formatting"
        F1[import 'axios']
        F2[import 'react']
        F3[empty line]
        F4[import '@/utils']
        F5[empty line]
        F6[import './local']
        F7[empty line]
        F8[const code = 1]
    end
    
    O1 -.-> F6
    O2 -.-> F2
    O3 -.-> F4
    O4 -.-> F8
    O5 -.-> F1
```

## Performance Considerations

### Parallel Processing Architecture

```
┌─────────────────────────────────────────────────┐
│              Main Thread                         │
│  - File discovery                               │
│  - Work distribution                            │
│  - Result aggregation                           │
└────────────────┬────────────────────────────────┘
                 │
    ┌────────────┼────────────┬─────────────┐
    ▼            ▼            ▼             ▼
┌─────────┐ ┌─────────┐ ┌─────────┐  ┌─────────┐
│Worker 1 │ │Worker 2 │ │Worker 3 │  │Worker N │
│         │ │         │ │         │  │         │
│ Parse   │ │ Parse   │ │ Parse   │  │ Parse   │
│ Format  │ │ Format  │ │ Format  │  │ Format  │
│ Write   │ │ Write   │ │ Write   │  │ Write   │
└─────────┘ └─────────┘ └─────────┘  └─────────┘
```

Optimizations:
- Rayon for automatic work-stealing parallelism
- Arc-wrapped source maps for shared access
- Minimal allocations during formatting
- Streaming file I/O

### Parallel Processing Workflow

```mermaid
graph TD
    M[Main Thread] --> Q[Task Queue]
    Q --> W1[Worker 1]
    Q --> W2[Worker 2]
    Q --> W3[Worker 3]
    Q --> WN[Worker N]
    
    W1 --> R[Results Channel]
    W2 --> R
    W3 --> R
    WN --> R
    
    R --> M
    
    subgraph "Per Worker"
        F1[Read File] --> F2[Parse]
        F2 --> F3[Format]
        F3 --> F4[Write]
    end
```

### Memory Management

- **Parser**: Reuses source map across files
- **AST**: In-place mutations where possible
- **Strings**: Cow<str> for efficient string handling
- **Buffers**: Pre-allocated for code generation

## Error Handling Strategy

### Error Categories

1. **Fatal Errors** (stop execution)
   - File system errors (permissions, disk full)
   - Invalid CLI arguments
   - Catastrophic parser failures

2. **File Errors** (skip file, continue)
   - Syntax errors
   - Unsupported TypeScript features
   - Encoding issues

3. **Formatting Warnings** (apply partial formatting)
   - Circular dependencies
   - Ambiguous sorting scenarios
   - Comment association issues

### Error Reporting

```
✗ src/components/Button.tsx: Failed to parse file
  → Unexpected token at line 42, column 15
  → Expected '}' but found ']'

✗ src/utils/helper.ts: Circular dependency detected
  → Cannot reorder members without breaking semantics
  → Partial formatting applied

Summary: 2 errors, 148 files formatted successfully
```

### Error Handling Flow

```mermaid
stateDiagram-v2
    [*] --> Processing
    Processing --> ParseError: Syntax Error
    Processing --> FileError: I/O Error
    Processing --> Success: No Errors
    
    ParseError --> PartialFormat: Can Recover
    ParseError --> SkipFile: Cannot Recover
    
    FileError --> Report: Log Error
    PartialFormat --> Report: Warn User
    SkipFile --> Report: Error Message
    Success --> Report: Success Message
    
    Report --> [*]
```

### File Processing State Machine

```mermaid
stateDiagram-v2
    [*] --> Discovered: File Found
    Discovered --> Reading: Start Processing
    Reading --> Parsing: Content Loaded
    Parsing --> Analyzing: AST Created
    Analyzing --> Transforming: Metadata Extracted
    Transforming --> Generating: AST Modified
    Generating --> Writing: Code Generated
    Writing --> Complete: File Updated
    
    Reading --> Failed: I/O Error
    Parsing --> Failed: Syntax Error
    Analyzing --> Failed: Analysis Error
    Transforming --> Failed: Transform Error
    Generating --> Failed: Generation Error
    Writing --> Failed: Write Error
    
    Complete --> [*]: Success
    Failed --> [*]: Error Reported
```

## Testing Architecture

### Test Categories

1. **Unit Tests** (per module)
   ```rust
   #[test]
   fn test_categorize_imports() {
       assert_eq!(
           ImportAnalyzer::categorize_import("react"),
           ImportCategory::External
       );
   }
   ```

2. **Integration Tests** (full pipeline)
   ```rust
   #[test]
   fn test_complete_formatting() {
       let input = load_fixture("messy.ts");
       let expected = load_fixture("clean.ts");
       assert_eq!(format_code(input), expected);
   }
   ```

3. **Property-Based Tests** (invariants)
   ```rust
   #[proptest]
   fn formatting_preserves_semantics(input: TypeScriptAst) {
       let formatted = format(input.clone());
       assert_eq!(evaluate(input), evaluate(formatted));
   }
   ```

4. **Snapshot Tests** (regression prevention)
   ```rust
   #[test]
   fn test_real_world_file() {
       let result = format_file("samples/react-component.tsx");
       insta::assert_snapshot!(result);
   }
   ```

### Test Data Organization

```
tests/
├── fixtures/
│   ├── imports/
│   ├── objects/
│   ├── classes/
│   └── edge-cases/
├── snapshots/
└── golden/
    └── real-world-examples/
```

## Future Extensibility

### Plugin Architecture (Future)

```rust
trait FormattingRule {
    fn name(&self) -> &str;
    fn applicable(&self, node: &Node) -> bool;
    fn apply(&self, node: &mut Node) -> Result<()>;
}

struct RuleEngine {
    rules: Vec<Box<dyn FormattingRule>>,
}
```

### Incremental Formatting (Future)

```
┌────────────────┐     ┌────────────────┐
│ Previous AST   │     │  Current AST   │
│   (cached)     │     │                │
└────────┬───────┘     └────────┬───────┘
         │                      │
         └──────────┬───────────┘
                    ▼
            ┌───────────────┐
            │  Diff Engine  │
            │               │
            └───────┬───────┘
                    ▼
            ┌───────────────┐
            │Format Changed │
            │  Nodes Only   │
            └───────────────┘
```

### Language Support Expansion

The architecture supports adding new languages by:
1. Implementing a new parser module
2. Adding language-specific rules
3. Extending the categorization logic
4. Updating file detection

### Editor Integration Points

```
┌─────────────┐     JSON-RPC      ┌─────────────┐
│   VSCode    │ ←───────────────→ │  krokfmt    │
│  Extension  │                   │   Server    │
└─────────────┘                   └─────────────┘
                                          │
                                          ▼
                                  ┌───────────────┐
                                  │  Format API   │
                                  │  (library)    │
                                  └───────────────┘
```

## Conclusion

krokfmt's design prioritizes simplicity, performance, and correctness. By leveraging Rust's performance characteristics and SWC's robust parsing capabilities, it provides a fast, reliable formatting solution that enforces consistent code organization principles without configuration overhead.