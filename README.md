# krokfmt

[![Tests](https://github.com/skeswa/krokfmt/actions/workflows/test.yml/badge.svg)](https://github.com/skeswa/krokfmt/actions/workflows/test.yml)
[![License: MIT](https://img.shields.io/badge/License-MIT-yellow.svg)](https://opensource.org/licenses/MIT)

A highly opinionated, zero-configuration code organizer written in Rust.

## Goal

In the age of AI, I need to get better at quickly reading and understanding
generated code. This tool makes code "flow" in a more consistent way. Like using
a consistent format, using a consistent set of organizing principles can reduce
unnecessary "brain strain".

## Features

- **Import Organization**: Automatically groups and sorts imports by scope (external, absolute, relative)
- **Alphabetical Sorting**: Sorts all unordered lists (object properties, function arguments, etc.)
- **Visibility-based Ordering**: Places exported members before private members
- **Zero Configuration**: No options, no debates - just consistent formatting

## Installation

```bash
cargo install krokfmt
```

## Usage

Format files in place:

```bash
krokfmt src/
krokfmt "src/**/*.ts"
krokfmt file1.ts file2.tsx
```

Check if files are formatted (CI mode):

```bash
krokfmt --check src/
```

Print formatted output without modifying files:

```bash
krokfmt --stdout file.ts
```

Skip backup creation:

```bash
krokfmt --no-backup src/
```

## Formatting Rules

krokfmt enforces a strict set of formatting rules with no configuration options. Here's what it does:

### 1. Import Organization

Imports are automatically organized into three categories with empty lines between groups:

1. **External** - Packages from node_modules (no path prefix)
2. **Absolute** - Paths starting with `@` or `~`
3. **Relative** - Paths starting with `./` or `../`

Within each group, imports are sorted alphabetically by path.

```typescript
// Before
import { helper } from "./helper";
import React from "react";
import { Button } from "@ui/Button";
import axios from "axios";
import type { User } from "../types";
import "./styles.css";

// After
import axios from "axios";
import React from "react";
import "./styles.css";

import { Button } from "@ui/Button";

import type { User } from "../types";
import { helper } from "./helper";
```

**Special handling:**

- Preserves all import syntaxes (default, named, namespace, side-effect, type)
- Maintains import aliases (`import { foo as bar }`)
- Moves orphaned imports to the top of the file
- Preserves shebang lines and file-level comments

### 2. Member Visibility Ordering

Declarations are organized by visibility to clearly separate public API from internal implementation:

```typescript
// Before
function internalHelper() {
  return "helper";
}
export function publicAPI() {
  return internalHelper();
}
const privateConfig = { key: "value" };
export const publicConfig = { ...privateConfig, public: true };

// After
const privateConfig = { key: "value" };

export const publicConfig = { ...privateConfig, public: true };
export function publicAPI() {
  return internalHelper();
}

function internalHelper() {
  return "helper";
}
```

**Smart dependency preservation:**

- Runtime dependencies are hoisted when needed (variables, arrow functions)
- Function declarations can be called before declaration (hoisting)
- Type-only constructs (interfaces, type aliases) can forward reference each other
- Classes in type positions don't require ordering

### 3. Alphabetical Sorting

All unordered lists are sorted alphabetically for consistency and easier scanning:

#### Object Properties

```typescript
// Before
const user = {
  age: 30,
  name: "John",
  email: "john@example.com",
};

// After
const user = {
  age: 30,
  email: "john@example.com",
  name: "John",
};
```

#### Function Parameters (Object Destructuring)

```typescript
// Before
function createUser({ email, name, age }: UserData) {}
const process = ({ output, input, config }) => {};

// After
function createUser({ age, email, name }: UserData) {}
const process = ({ config, input, output }) => {};
```

#### Class Members

Classes are organized by visibility and type in a specific order:

1. Public static fields (alphabetically)
2. Private static fields (alphabetically)
3. Public static methods (alphabetically)
4. Private static methods (alphabetically)
5. Public instance fields (alphabetically)
6. Private instance fields (alphabetically)
7. Constructor
8. Public instance methods (alphabetically)
9. Private instance methods (alphabetically)

**Note:** Private members use the `#` syntax for true privacy. TypeScript's `private` keyword is treated as public for sorting purposes since it's only a compile-time check.

```typescript
// Before
class User {
  name: string;
  #privateId: number;
  static VERSION = "1.0";
  static #SECRET = "hidden";

  greet() {}
  #validate() {}
  constructor() {}
  static create() {}
  static #generate() {}
  age: number;
}

// After
class User {
  static VERSION = "1.0";
  static #SECRET = "hidden";

  static create() {}
  static #generate() {}

  age: number;
  name: string;
  #privateId: number;

  constructor() {}

  greet() {}
  #validate() {}
}
```

#### Type Members

Union and intersection types are sorted alphabetically:

```typescript
// Before
type Status = "error" | "success" | "pending";
type Combined = Writable & Timestamped & Identifiable;

// After
type Status = "error" | "pending" | "success";
type Combined = Identifiable & Timestamped & Writable;
```

#### Enum Members

Only string enums are sorted (numeric enums preserve their values):

```typescript
// Before
enum Status {
  Pending = "pending",
  Active = "active",
  Disabled = "disabled",
}

// After
enum Status {
  Active = "active",
  Disabled = "disabled",
  Pending = "pending",
}
```

#### JSX/TSX Properties

JSX props follow a specific order for consistency:

1. `key` and `ref` (always first)
2. Regular props (alphabetically)
3. Event handlers (alphabetically, grouped)
4. Spread operators (always last)

```typescript
// Before
<Button
    onClick={handleClick}
    disabled={false}
    key="btn-1"
    className="primary"
    ref={buttonRef}
    {...props}
/>

// After
<Button
    key="btn-1"
    ref={buttonRef}
    className="primary"
    disabled={false}
    onClick={handleClick}
    {...props}
/>
```

### 4. Comment Preservation

All comments are preserved in their correct positions:

- **Line comments** (`//`) maintain their position relative to code
- **Block comments** (`/* */`) preserve internal formatting
- **JSDoc comments** (`/** */`) move with their associated declarations
- **Special comments** (TypeScript directives, ESLint rules) stay in place

```typescript
// This comment stays with the import
import React from "react";

/**
 * This JSDoc moves with the function
 * @param name - The name to greet
 */
export function greet(name: string) {
  // This comment is preserved inside
  return `Hello, ${name}!`; // This stays at line end
}
```

### 5. Whitespace and Formatting

- Empty lines between import groups
- Empty lines between visibility groups (exported vs non-exported)
- No empty lines within alphabetized groups
- Preserves existing line endings (LF or CRLF)
- Maintains file encoding (UTF-8 with optional BOM)

### 6. What krokfmt Does NOT Change

- Does not modify code logic or behavior
- Does not change import/export semantics
- Does not alter numeric enum values
- Does not reorder positional function parameters
- Does not format code style (indentation, brackets, etc.)
- Only reorders and organizes existing code structures

## Development

```bash
# Run tests
cargo test

# Build release version
cargo build --release

# Run formatter on sample files
cargo run -- test_files/
```

## License

MIT
