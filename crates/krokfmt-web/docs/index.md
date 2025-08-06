---
layout: home

hero:
  name: "krokfmt"
  text: "Zero-Configuration TypeScript Formatter"
  tagline: "A highly opinionated code formatter that just works. No config files, no options, no debates."
  actions:
    - theme: brand
      text: Try in Playground
      link: /playground
    - theme: alt
      text: Get Started
      link: /guide/getting-started

features:
  - icon: 🎯
    title: Zero Configuration
    details: No config files, no options, no debates. Just consistent formatting that works out of the box. Focus on writing code, not arguing about style.
    
  - icon: ⚡
    title: Lightning Fast
    details: Written in Rust for maximum performance. Format large codebases in seconds with parallel processing and efficient AST manipulation.
    
  - icon: 📦
    title: Smart Organization
    details: Automatically organizes imports, sorts object properties, and orders class members by visibility. Your code stays clean and maintainable.
    
  - icon: 💬
    title: Comment Preservation
    details: Intelligently preserves all types of comments, including inline, JSDoc, and trailing comments. Your documentation stays intact.
    
  - icon: 🔧
    title: TypeScript Native
    details: Built specifically for TypeScript and TSX. Handles all modern TypeScript features including decorators, generics, and type assertions.
    
  - icon: 🤖
    title: AI-Era Design
    details: Designed to make AI-generated code more readable and consistent. Perfect for maintaining quality in the age of code generation.
---

## Quick Start

### Installation

::: code-group

```bash [cargo]
cargo install krokfmt
```

```bash [binary]
# Download from GitHub releases
curl -L https://github.com/skeswa/krokfmt/releases/latest/download/krokfmt-$(uname -s)-$(uname -m) -o krokfmt
chmod +x krokfmt
sudo mv krokfmt /usr/local/bin/
```

:::

### Usage

::: code-group

```bash [Format files]
# Format files in place
krokfmt src/

# Format specific file
krokfmt src/index.ts
```

```bash [Check mode]
# Check if files are formatted (CI mode)
krokfmt --check src/
```

```bash [Print output]
# Print formatted output without modifying files
krokfmt --stdout file.ts
```

:::

## Example

krokfmt automatically organizes and formats your TypeScript code:

::: code-group

```typescript [Before]
import {z} from 'zod';
import React from 'react';
import {Button} from './Button';
import axios from 'axios';

export function UserCard({name,email,id}:{
name:string,email:string,id:number
}) {
const handleClick=()=>{
console.log('clicked');
}
return <div>{name}</div>
}
```

```typescript [After]
import axios from 'axios';
import React from 'react';
import { z } from 'zod';

import { Button } from './Button';

export function UserCard({
    email,
    id,
    name,
}: {
    email: string;
    id: number;
    name: string;
}) {
    const handleClick = () => {
        console.log('clicked');
    };
    return <div>{name}</div>;
}
```

:::

## Why krokfmt?

In a world of endless formatter configuration options and style debates, krokfmt takes a different approach: **no configuration at all**. 

- ✅ **Consistent** - Every codebase formatted with krokfmt looks the same
- ✅ **Fast** - Written in Rust for maximum performance
- ✅ **Smart** - Understands TypeScript semantics, not just syntax
- ✅ **Complete** - Handles imports, exports, classes, interfaces, and more
- ✅ **Respectful** - Preserves your comments and documentation

Stop debating tabs vs spaces, semicolons, and bracket placement. Let krokfmt handle it all.