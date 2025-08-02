# krokfmt Requirements Specification

## Table of Contents

1. [Functional Requirements](#functional-requirements)
2. [Non-Functional Requirements](#non-functional-requirements)
3. [Requirements Traceability Matrix](#requirements-traceability-matrix)
4. [Acceptance Criteria](#acceptance-criteria)

## Functional Requirements

### FR1: Import/Export Organization

#### FR1.1: Import Statement Parsing

**Description**: The system shall parse and identify all import and export statements in a TypeScript file.

**Acceptance Criteria**:

- Recognizes all ES6 import syntaxes:
  - Default imports: `import React from 'react'`
  - Named imports: `import { useState } from 'react'`
  - Namespace imports: `import * as utils from './utils'`
  - Side-effect imports: `import './styles.css'`
  - Type imports: `import type { User } from './types'`
- Handles mixed import styles: `import React, { useState } from 'react'`
- Preserves import aliases: `import { foo as bar } from './module'`

#### FR1.2: Import Categorization

**Description**: The system shall categorize imports into three distinct groups based on their path patterns.

**Categories**:

1. **External** - Packages from node_modules (no path prefix)
2. **Absolute** - Paths starting with `@` or `~`
3. **Relative** - Paths starting with `./` or `../`

**Examples**:

```typescript
// External
import React from "react";
import lodash from "lodash/debounce";

// Absolute
import { Button } from "@components/Button";
import { config } from "~/config";

// Relative
import { helper } from "./utils/helper";
import { User } from "../types";
```

#### FR1.3: Import Sorting

**Description**: The system shall sort imports alphabetically within each category by import path.

**Rules**:

- Case-sensitive alphabetical ordering
- Special characters follow ASCII ordering
- Path comparison ignores quotes

#### FR1.4: Import Positioning

**Description**: The system shall place all import and export statements at the top of the file.

**Constraints**:

- Preserves shebang lines (`#!/usr/bin/env node`)
- Preserves file-level comments before imports
- Moves orphaned imports to the top

#### FR1.5: Import Group Separation

**Description**: The system shall separate import groups with exactly one empty line.

**Layout**:

```typescript
// External imports
import a from "a";
import b from "b";

// Absolute imports
import c from "@/c";
import d from "@/d";

// Relative imports
import e from "./e";
import f from "../f";
import g from "../../g";
```

#### FR1.6: Import Syntax Preservation

**Description**: The system shall preserve the exact import syntax and semantics.

**Guarantees**:

- Maintains import type (default/named/namespace)
- Preserves type-only imports
- Keeps side-effect import behavior
- Retains import assertions

### FR2: Member Visibility Ordering

#### FR2.1: Export Detection

**Description**: The system shall identify exported versus non-exported members in a file.

**Member Types**:

- Functions (regular and arrow)
- Classes
- Interfaces
- Type aliases
- Constants
- Variables
- Enums

#### FR2.2: Visibility-Based Organization

**Description**: The system shall organize file contents by visibility level, with exported members at the top and non-exported members at the bottom.

**Organization Rules**:

- Exported members (public API) appear first
- Non-exported members (internal implementation) appear last
- Clear visual separation between visibility groups
- Maintains semantic correctness

#### FR2.3: Dependency Preservation

**Description**: The system shall never reorder members in a way that breaks code functionality, even when organizing by visibility.

**Dependency Rules**:

- If an exported member depends on a non-exported member, the dependency must appear first
- Variable usage before declaration is prevented
- Function hoisting behavior is respected
- Class inheritance chains are maintained
- Circular dependencies are handled gracefully

**Forward Reference Exceptions**:

TypeScript allows forward references for certain constructs. The formatter must recognize these cases and not require dependencies to be declared first:

1. **Function Declarations** - Can be called before declaration due to hoisting

   ```typescript
   foo(); // Valid - function declarations are hoisted
   function foo() {}
   ```

2. **Class Declarations** - Can be referenced in type positions before declaration

   ```typescript
   function createInstance(): MyClass {
     return new MyClass();
   } // Valid in type position
   class MyClass {}
   ```

3. **Interface Declarations** - Can be used in type positions or extended before declaration

   ```typescript
   interface A extends B {} // Valid - interfaces can reference later interfaces
   interface B {}
   ```

4. **Type Aliases** - Can be referenced in other type declarations before definition

   ```typescript
   type A = B | string; // Valid - types can reference later types
   type B = number;
   ```

5. **Enum Declarations** - Can be used in type positions before declaration
   ```typescript
   let status: Status; // Valid - enums can be referenced before declaration
   enum Status {
     Active,
     Inactive,
   }
   ```

**Runtime Dependency Rules**:

These constructs MUST have their dependencies declared first as they execute at runtime:

1. **Variable Declarations** - Cannot use undeclared variables

   ```typescript
   const a = b; // Error - b is not defined
   const b = 1;
   ```

2. **Arrow Functions and Function Expressions** - Not hoisted, must be declared before use

   ```typescript
   foo(); // Error - cannot access before initialization
   const foo = () => {};
   ```

3. **Class Expressions** - Not hoisted, must be declared before use

   ```typescript
   new MyClass(); // Error - cannot access before initialization
   const MyClass = class {};
   ```

4. **Object/Array Destructuring** - Requires values to exist

   ```typescript
   const { x } = obj; // Error - obj is not defined
   const obj = { x: 1 };
   ```

5. **Method Calls and Property Access** - Requires object to exist
   ```typescript
   console.log(config.url); // Error - config is not defined
   const config = { url: "api" };
   ```

#### FR2.4: Visibility Grouping and Alphabetization

**Description**: The system shall group declarations by visibility level and alphabetize within each group.

**Grouping Rules**:

1. **Visibility Groups** (in order):

   - Exported declarations (functions, classes, types, interfaces, enums, variables)
   - Non-exported declarations (same types as above)

2. **Within Each Group**:

   - Sort alphabetically by declaration name
   - Maintain stable sort for items with identical names

3. **Visual Separation**:

   - Add empty line between visibility groups (see FR7.1 for comprehensive visual separation rules)
   - No empty lines within a visibility group (unless preserving existing formatting)

4. **Dependency Override**:
   - Dependencies can be hoisted above their visibility group if required
   - Hoisted dependencies maintain their relative order

**Example**:

```typescript
// Before
function helperB() {
  return "b";
}
export function mainA() {
  return helperB();
}
const configC = { url: "api" };
export class ServiceD {
  config = configC;
}
function helperE() {
  return "e";
}

// After
// Hoisted dependencies (required by exports)
const configC = { url: "api" };
function helperB() {
  return "b";
}

// Exported members (alphabetized)
export function mainA() {
  return helperB();
}
export class ServiceD {
  config = configC;
}

// Non-exported members (alphabetized)
function helperE() {
  return "e";
}
```

### FR3: Alphabetical Sorting

#### FR3.1: Function Argument Sorting

**Description**: The system shall sort function arguments alphabetically when order doesn't affect behavior.

**Applicability**:

- Object destructuring parameters
- NOT positional parameters

**Example**:

```typescript
// Before
function process({ zebra, apple, banana }: Options) {}

// After
function process({ apple, banana, zebra }: Options) {}
```

#### FR3.2: Object Property Sorting

**Description**: The system shall sort object literal properties alphabetically.

**Rules**:

- Computed properties sort by their string representation
- Spread operators sort to the end
- Getters/setters stay together

#### FR3.3: Class Member Sorting

**Description**: The system shall sort class fields and methods alphabetically within visibility groups.

**Order**:

1. Public static fields (alphabetically)
2. Private static fields (alphabetically)
3. Public static methods (alphabetically)
4. Private static methods (alphabetically)
5. Public instance fields (alphabetically)
6. Private instance fields (alphabetically)
7. Constructor
8. Public instance methods (alphabetically)
9. Private instance methods (alphabetically)

**Visual Separation**: See FR7.3 for rules on adding empty lines between these visibility groups.

#### FR3.4: Type Member Sorting

**Description**: The system shall sort members of union and intersection types alphabetically.

**Example**:

```typescript
// Before
type Status = "error" | "success" | "pending";

// After
type Status = "error" | "pending" | "success";
```

#### FR3.5: Enum Member Sorting

**Description**: The system shall sort enum members alphabetically.

**Constraints**:

- Only for string enums
- Numeric enums preserve values

#### FR3.6: JSX Property Sorting

**Description**: The system shall sort JSX/TSX element properties alphabetically.

**Special Rules**:

- `key` and `ref` always first
- Event handlers group together
- Spread operators at the end

### FR4: CLI Interface

#### FR4.1: Single File Processing

**Description**: The system shall process individual TypeScript files.

**Command**: `krokfmt path/to/file.ts`

**Behavior**:

- Formats file in place
- Creates backup unless --no-backup
- Reports success/failure

#### FR4.2: Directory Processing

**Description**: The system shall recursively process all TypeScript files in directories.

**Command**: `krokfmt src/`

**Features**:

- Finds all .ts/.tsx/.mts/.cts files
- Skips node_modules
- Skips hidden directories
- Follows symbolic links

#### FR4.3: Glob Pattern Support

**Description**: The system shall support glob patterns for file selection.

**Command**: `krokfmt "src/**/*.ts"`

**Patterns**:

- Standard glob syntax
- Multiple patterns allowed
- Negative patterns (exclusions)

#### FR4.4: Check Mode

**Description**: The system shall verify formatting without modifying files.

**Command**: `krokfmt --check src/`

**Behavior**:

- Exit code 0 if all formatted
- Exit code 1 if changes needed
- Lists files needing formatting

#### FR4.5: Stdout Mode

**Description**: The system shall output formatted code to stdout.

**Command**: `krokfmt --stdout file.ts`

**Use Cases**:

- Piping to other tools
- Preview formatting
- Editor integration

#### FR4.6: Version Display

**Description**: The system shall display version information.

**Command**: `krokfmt --version`

**Output**: `krokfmt 0.1.0`

#### FR4.7: Help Display

**Description**: The system shall provide comprehensive help information.

**Command**: `krokfmt --help`

**Contents**:

- Usage examples
- Option descriptions
- File pattern examples

### FR5: File Handling

#### FR5.1: Encoding Preservation

**Description**: The system shall preserve file encoding (UTF-8).

**Features**:

- Detects BOM
- Maintains encoding
- Handles non-ASCII characters

#### FR5.2: Line Ending Preservation

**Description**: The system shall preserve existing line endings.

**Detection**:

- LF (Unix/macOS)
- CRLF (Windows)
- Consistent throughout file

#### FR5.3: Backup Creation

**Description**: The system shall create backups before modifying files.

**Default Behavior**:

- Creates .bak files
- Single backup per file
- Opt-out with --no-backup

#### FR5.4: File Type Support

**Description**: The system shall handle all TypeScript file extensions.

**Extensions**:

- `.ts` - Standard TypeScript
- `.tsx` - TypeScript with JSX
- `.mts` - ES modules
- `.cts` - CommonJS modules

### FR6: Comment Handling

#### FR6.1: Line Comment Preservation

**Description**: The system shall preserve all single-line comments (`//`) in their correct positions.

**Rules**:

- Comments on their own line stay on their own line
- Trailing comments remain at end of line
- Leading comments stay attached to following statement
- Indentation is preserved relative to code

**Example**:

```typescript
// This comment stays with the import
import React from "react"; // This stays at end of line

// This comment stays with the function
function foo() {
  // Inner comment preserved
  return 42; // Trailing comment
}
```

#### FR6.2: Block Comment Preservation

**Description**: The system shall preserve all multi-line comments (`/* */`) with formatting intact.

**Rules**:

- Block comment formatting is preserved exactly
- Position relative to code is maintained
- Inline block comments stay inline
- Multi-line blocks keep their line structure

**Example**:

```typescript
/* This block comment
   spans multiple lines
   and keeps its format */
const x = /* inline */ 42;

/*
 * Star-aligned comment
 * stays formatted
 */
function bar() {}
```

#### FR6.3: JSDoc Comment Preservation

**Description**: The system shall preserve JSDoc comments (`/** */`) with their associated declarations.

**Rules**:

- JSDoc comments move with their documented element
- Internal JSDoc formatting is preserved
- Tags and indentation maintained
- Blank lines between JSDoc and declaration preserved

**Example**:

```typescript
/**
 * This is a JSDoc comment
 * @param {string} name - The name
 * @returns {void}
 */
export function greet(name: string): void {}
```

#### FR6.4: Comment Positioning

**Description**: The system shall maintain correct comment positioning relative to code.

**Categories**:

1. **Leading** - Comments before a statement on separate lines
2. **Trailing** - Comments after code on the same line
3. **Floating** - Comments between statements not attached to code
4. **Header** - File-level comments at the top

**Rules**:

- Leading comments move with their associated statement
- Trailing comments stay on the same line as their code
- Floating comments maintain relative position
- Header comments remain at file top (after shebang if present)

#### FR6.5: Comment Association

**Description**: The system shall correctly associate comments with code elements.

**Association Rules**:

- Comments immediately before a statement belong to that statement
- Comments at the end of a line belong to that line's code
- Blank lines can break comment association
- Comments move with their associated code during reordering

**Example**:

```typescript
// This belongs to export const a
export const a = 1;

const b = 2; // This belongs to const b

// This is a floating comment

// This belongs to function c
function c() {}
```

#### FR6.6: Special Comment Handling

**Description**: The system shall recognize and preserve special TypeScript comments.

**Special Comments**:

- `// @ts-ignore` - TypeScript ignore directive
- `// @ts-expect-error` - TypeScript error expectation
- `// @ts-nocheck` - File-level TS checking disable
- `// eslint-disable` - ESLint directives
- `// prettier-ignore` - Prettier directives
- `// #region` / `// #endregion` - Code folding markers

**Rules**:

- Pragma comments must stay with their target line
- File-level directives stay at appropriate file position
- Region markers maintain their pairing

#### FR6.7: Standalone Comment Preservation

**Description**: The system shall preserve the position of standalone comments within their local context, particularly when those comments provide section headers or contextual information.

**Scope**:

1. **Within Classes** - Standalone comments at the beginning of a class body remain at the top after member reordering
2. **Within Functions** - Section comments maintain their position relative to the code blocks they describe
3. **Module Level** - Due to visibility-based organization (FR2.4), top-level declarations may be reordered, which can affect section organization

**Rules**:

- Comments separated from code by blank lines are considered standalone
- Within classes/functions, standalone comments maintain their relative position
- Comments directly preceding code (no blank line) are attached to that code and move with it
- Class-level descriptive comments stay at the top of the class body

**Note**: At the module level, the formatter prioritizes visibility-based organization (exported members first, non-exported second) which may reorder declarations and their associated comments. This is by design to maintain a consistent public API surface at the top of modules.

### FR7: Visual Separation

#### FR7.1: Module-Level Declaration Separation

**Description**: The system shall add visual separation between different types of top-level declarations to improve code readability.

**Rules**:

- One empty line between different declaration types (function, class, interface, type, const/let/var, enum)
- One empty line between exported and non-exported visibility groups (see FR2.4)
- No empty lines between consecutive declarations of the same type and visibility
- Related items may stay together (e.g., a type and its type guard function)

**Example**:

```typescript
// Imports (see FR1.5 for import group separation)
import React from 'react';
import { helper } from './utils';

// Constants grouped together (no separation)
export const API_URL = '/api';
export const TIMEOUT = 5000;

// Empty line between const and function declarations
export function fetchData() {
    return fetch(API_URL);
}
export function processData(data: unknown) {
    return transform(data);
}

// Empty line between exported and non-exported groups
const INTERNAL_CACHE = new Map();

// Empty line between const and function
function validateCache() {
    return INTERNAL_CACHE.size > 0;
}

// Empty line between function and class
class DataProcessor {
    process() { /* ... */ }
}

// Empty line between class and interface
interface Config {
    url: string;
    timeout: number;
}

// Empty line between interface and type
type Status = 'idle' | 'loading' | 'error';

// Related items can stay together
type User = { id: string; name: string };
function isUser(value: unknown): value is User {
    return typeof value === 'object' && value !== null && 'id' in value;
}
```

#### FR7.2: Import Group Separation

**Description**: Import groups shall be visually separated (this updates and clarifies FR1.5).

**Rules**: See FR1.5 - no changes, just referenced here for completeness.

#### FR7.3: Class Member Group Separation

**Description**: The system shall add visual separation between class member visibility groups to improve class readability.

**Groups** (in order per FR3.3):

1. Public static fields
2. Private static fields
3. Public static methods
4. Private static methods
5. Public instance fields
6. Private instance fields
7. Constructor
8. Public instance methods
9. Private instance methods

**Rules**:

- One empty line between each visibility group
- No empty lines within a visibility group
- Comments remain with their associated members

**Example**:

```typescript
class ApiClient {
    // Public static fields
    static defaultTimeout = 5000;
    static version = '1.0.0';
    
    // Private static fields
    static #instance: ApiClient;
    static #secretKey = process.env.SECRET_KEY;
    
    // Public static methods
    static getInstance() {
        if (!ApiClient.#instance) {
            ApiClient.#instance = new ApiClient();
        }
        return ApiClient.#instance;
    }
    
    // Private static methods
    static #validateKey(key: string) {
        return key.length > 10;
    }
    
    // Public instance fields
    baseUrl: string;
    timeout: number;
    
    // Private instance fields
    #authToken?: string;
    #retryCount = 0;
    
    // Constructor
    constructor(baseUrl = '/api') {
        this.baseUrl = baseUrl;
        this.timeout = ApiClient.defaultTimeout;
    }
    
    // Public instance methods
    async get(endpoint: string) {
        return this.#request('GET', endpoint);
    }
    
    setAuth(token: string) {
        this.#authToken = token;
    }
    
    // Private instance methods
    async #request(method: string, endpoint: string) {
        // Implementation
    }
    
    #handleError(error: Error) {
        this.#retryCount++;
        throw error;
    }
}
```

#### FR7.4: Declaration Type Preservation

**Description**: The system shall preserve logical groupings indicated by the programmer.

**Rules**:

- Multiple consecutive empty lines in the source are normalized to one empty line
- Section header comments create logical boundaries that are preserved
- Related declarations (e.g., a constant and its type) maintain their grouping
- The formatter should not break apart obviously related code

**Example**:

```typescript
// ========== Configuration Types ==========
// This section header creates a logical boundary

export interface AppConfig {
    name: string;
    version: string;
}

export const DEFAULT_CONFIG: AppConfig = {
    name: 'MyApp',
    version: '1.0.0'
};

// ========== Utility Functions ==========
// Another logical section

export function parseConfig(json: string): AppConfig {
    return JSON.parse(json);
}
```

## Non-Functional Requirements

### NFR1: Performance

#### NFR1.1: Processing Speed

**Description**: The system shall format files quickly and efficiently.

**Metrics**:

- 1000 lines in < 100ms
- 10,000 lines in < 1s
- Linear time complexity

#### NFR1.2: Parallel Processing

**Description**: The system shall process multiple files concurrently.

**Implementation**:

- Work-stealing parallelism
- CPU core utilization
- Shared nothing architecture

#### NFR1.3: Memory Efficiency

**Description**: The system shall use memory proportional to file size.

**Constraints**:

- < 10MB overhead per file
- No memory leaks
- Efficient string handling

#### NFR1.4: Large File Support

**Description**: The system shall handle large TypeScript files gracefully.

**Limits**:

- Files up to 10MB
- 100,000+ lines of code
- Graceful degradation

### NFR2: Correctness

#### NFR2.1: Semantic Preservation

**Description**: The system shall never change program behavior.

**Guarantees**:

- No logic changes
- No type changes
- No runtime differences

#### NFR2.2: Comment Preservation

**Description**: The system shall preserve all comments and their associations.

**Types**:

- Line comments
- Block comments
- JSDoc comments
- TS pragma comments

#### NFR2.3: Syntax Support

**Description**: The system shall support all valid TypeScript syntax.

**Coverage**:

- ES2022+ features
- Latest TypeScript
- Stage 3 proposals
- JSX/TSX

#### NFR2.4: Output Validity

**Description**: The system shall always produce valid TypeScript.

**Validation**:

- Parseable output
- No syntax errors
- Maintains types

### NFR3: Robustness

#### NFR3.1: Error Recovery

**Description**: The system shall handle errors gracefully without crashing.

**Scenarios**:

- Syntax errors
- File permissions
- Disk space
- Malformed input

#### NFR3.2: Error Messaging

**Description**: The system shall provide clear, actionable error messages.

**Format**:

```
Error: Failed to parse file
  File: src/components/Button.tsx
  Line: 42, Column: 15
  Issue: Unexpected token ']', expected '}'
```

#### NFR3.3: Partial Formatting

**Description**: The system shall format valid portions of files with errors.

**Strategy**:

- Skip unparseable sections
- Format valid parts
- Report limitations

#### NFR3.4: Circular Dependency Handling

**Description**: The system shall detect and handle circular dependencies.

**Approach**:

- Dependency graph analysis
- Cycle detection
- Safe fallback behavior

### NFR4: Developer Experience

#### NFR4.1: Distribution

**Description**: The system shall be distributed as a single binary.

**Platforms**:

- macOS (x64, ARM64)
- Linux (x64, ARM64)
- Windows (x64)

#### NFR4.2: Cross-Platform Support

**Description**: The system shall work identically across platforms.

**Compatibility**:

- Path handling
- Line endings
- File systems

#### NFR4.3: Zero Dependencies

**Description**: The system shall have no runtime dependencies.

**Benefits**:

- Easy installation
- No version conflicts
- Portable execution

#### NFR4.4: CI/CD Integration

**Description**: The system shall integrate well with CI/CD pipelines.

**Features**:

- Exit codes
- Machine-readable output
- Quiet mode
- JSON reports (future)

### NFR5: Maintainability

#### NFR5.1: Test Coverage

**Description**: The system shall maintain high test coverage.

**Targets**:

- 90% line coverage
- 85% branch coverage
- 100% critical paths

#### NFR5.2: TDD Methodology

**Description**: The system shall be developed using Test-Driven Development.

**Process**:

1. Write failing test
2. Implement minimum code
3. Refactor
4. Document

#### NFR5.3: Modular Architecture

**Description**: The system shall use a modular, extensible architecture.

**Principles**:

- Single responsibility
- Loose coupling
- High cohesion
- Clear interfaces

#### NFR5.4: Error Handling

**Description**: The system shall have comprehensive error handling.

**Requirements**:

- Result types
- Error propagation
- Recovery strategies
- Logging

## Acceptance Criteria

### Release Criteria

1. **Functionality**

   - All FR1.\* requirements implemented
   - FR3.2 (object sorting) implemented
   - FR4.\* (CLI) fully functional

2. **Quality**

   - Zero panics on valid input
   - 90% test coverage achieved
   - All integration tests passing

3. **Performance**

   - Meets NFR1.1 speed requirements
   - Memory usage within limits
   - Parallel processing working

4. **Documentation**
   - README complete
   - All public APIs documented
   - Usage examples provided

### Definition of Done

A requirement is considered complete when:

1. Implementation passes all tests
2. Code review completed
3. Documentation updated
4. Integration tests added
5. No regression in existing features
