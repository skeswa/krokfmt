# krokfmt

A highly opinionated, zero-configuration TypeScript code formatter written in Rust.

## Features

- **Import Organization**: Automatically groups and sorts imports by scope (external, absolute, relative)
- **Alphabetical Sorting**: Sorts all unordered lists (object properties, function arguments, etc.)
- **Visibility-based Ordering**: Places exported members before private members
- **Zero Configuration**: No options, no debates - just consistent formatting
- **Fast**: Written in Rust using SWC for blazing fast performance
- **Parallel Processing**: Formats multiple files concurrently

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

### Import Organization

Imports are grouped into three categories with empty lines between:

1. **External** - Packages from node_modules
2. **Absolute** - Paths starting with `@` or `~`
3. **Relative** - Paths starting with `./` or `../`

Within each group, imports are sorted alphabetically by path.

```typescript
// Before
import { helper } from './helper';
import React from 'react';
import { Button } from '@ui/Button';
import axios from 'axios';

// After
import axios from 'axios';
import React from 'react';

import { Button } from '@ui/Button';

import { helper } from './helper';
```

### Alphabetical Sorting

All lists where order doesn't affect behavior are sorted alphabetically:

```typescript
// Before
const config = {
    timeout: 5000,
    baseURL: 'http://api.example.com',
    headers: { 'Content-Type': 'application/json' },
};

// After
const config = {
    baseURL: 'http://api.example.com',
    headers: { 'Content-Type': 'application/json' },
    timeout: 5000,
};
```

### Member Visibility Ordering

Exported members are moved toward the top of the file (when possible without breaking dependencies).

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