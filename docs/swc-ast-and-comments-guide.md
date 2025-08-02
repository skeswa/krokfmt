# Understanding SWC AST and Comment Handling

## Table of Contents
1. [Introduction](#introduction)
2. [SWC AST Basics](#swc-ast-basics)
3. [How Comments are Stored](#how-comments-are-stored)
4. [Comment Types and Positions](#comment-types-and-positions)
5. [Working with Comments in Practice](#working-with-comments-in-practice)
6. [Common Pitfalls](#common-pitfalls)
7. [Advanced Techniques](#advanced-techniques)

## Introduction

SWC (Speedy Web Compiler) is a TypeScript/JavaScript compiler written in Rust. Unlike traditional AST representations where comments might be attached directly to nodes, SWC stores comments separately from the AST using a position-based system. This document explains how this system works and how to effectively work with it.

## SWC AST Basics

### What is an AST?

An Abstract Syntax Tree (AST) is a tree representation of source code structure. Each node in the tree represents a construct in the source code.

```typescript
// Source code
const x = 1 + 2;

// Simplified AST representation
VariableDeclaration {
  declarations: [
    VariableDeclarator {
      id: Identifier { name: "x" },
      init: BinaryExpression {
        left: Literal { value: 1 },
        operator: "+",
        right: Literal { value: 2 }
      }
    }
  ]
}
```

### Key SWC AST Concepts

#### 1. Spans
Every AST node in SWC has a `span` field that indicates where in the source code the node appears:

```rust
pub struct Span {
    pub lo: BytePos,  // Start position (byte offset)
    pub hi: BytePos,  // End position (byte offset)
    pub ctxt: SyntaxContext,  // Hygiene information
}
```

Example:
```typescript
// Source code with byte positions
// 0123456789012345
   const x = 1 + 2;
// ^^^^^^^^^^^^^^^
// Span { lo: 0, hi: 16 }
```

#### 2. BytePos
`BytePos` is a newtype wrapper around `u32` representing a byte offset in the source file:

```rust
#[derive(Copy, Clone, Eq, PartialEq, Ord, PartialOrd, Hash)]
pub struct BytePos(pub u32);
```

#### 3. Module Structure
The top-level AST node is typically a `Module`:

```rust
pub struct Module {
    pub span: Span,
    pub body: Vec<ModuleItem>,
    pub shebang: Option<JsWord>,
}
```

## How Comments are Stored

### The Key Insight: Separate Storage

**Comments are NOT part of the AST nodes**. Instead, they are stored in a separate data structure called `SingleThreadedComments`:

```rust
pub struct SingleThreadedComments {
    leading: RefCell<FxHashMap<BytePos, Vec<Comment>>>,
    trailing: RefCell<FxHashMap<BytePos, Vec<Comment>>>,
}
```

### Comment Structure

Each comment is represented as:

```rust
pub struct Comment {
    pub kind: CommentKind,
    pub span: Span,
    pub text: JsWord,
}

pub enum CommentKind {
    Line,   // Single-line comment: // comment
    Block,  // Multi-line comment: /* comment */
}
```

### Position-Based Storage

Comments are stored at the BytePos where their associated code STARTS, not where the comment itself appears:

```typescript
// Source with byte positions marked
// 00000000001111111111222222222233333333334
// 01234567890123456789012345678901234567890
   // This is a comment      <- Comment at bytes 0-20
   const x = 1;              <- Code starts at byte 23

// The comment is stored at BytePos(23), NOT BytePos(0)!
```

## Comment Types and Positions

### Leading Comments

Leading comments appear before a node and are stored at the node's start position:

```typescript
// Example 1: Simple leading comment
// 00000000001111111111222222222233333
// 01234567890123456789012345678901234
   // Leading comment for x
   const x = 1;

// Stored as: leading[BytePos(26)] = [Comment { text: " Leading comment for x" }]
```

```typescript
// Example 2: Multiple leading comments
// 00000000001111111111222222222233333333334444444
// 01234567890123456789012345678901234567890123456
   // First comment
   // Second comment
   const y = 2;

// Stored as: leading[BytePos(43)] = [
//   Comment { text: " First comment" },
//   Comment { text: " Second comment" }
// ]
```

### Trailing Comments

Trailing comments appear on the same line after code:

```typescript
// Example: Trailing comment
// 00000000001111111111222222222233333
// 01234567890123456789012345678901234
   const z = 3; // Trailing comment

// Stored as: trailing[BytePos(14)] = [Comment { text: " Trailing comment" }]
// Note: BytePos(14) is the END position of the statement
```

### Complex Example

```typescript
// Complete example with byte positions
// 000000000011111111112222222222333333333344444444445
// 012345678901234567890123456789012345678901234567890
   // Leading for import
   import { x } from './x'; // Trailing on import
   
   /* Block comment */
   // Leading for const
   const a = 1; // Trailing on const

// Storage:
// leading[BytePos(23)] = [Comment { text: " Leading for import" }]
// trailing[BytePos(47)] = [Comment { text: " Trailing on import" }]
// leading[BytePos(91)] = [Comment { text: " Block comment ", kind: Block }]
// leading[BytePos(91)] += [Comment { text: " Leading for const" }]
// trailing[BytePos(103)] = [Comment { text: " Trailing on const" }]
```

## Working with Comments in Practice

### Reading Comments

To read comments using the `Comments` trait:

```rust
use swc_common::comments::Comments;

// Get leading comments for a position
if let Some(comments) = comment_map.get_leading(node.span.lo) {
    for comment in comments {
        println!("Leading comment: {}", comment.text);
    }
}

// Get trailing comments for a position
if let Some(comments) = comment_map.get_trailing(node.span.hi) {
    for comment in comments {
        println!("Trailing comment: {}", comment.text);
    }
}
```

### Adding Comments

```rust
use swc_common::comments::{Comment, CommentKind};

// Add a leading comment
comment_map.add_leading(
    node.span.lo,
    Comment {
        kind: CommentKind::Line,
        span: DUMMY_SP,
        text: " This is a new comment".into(),
    }
);
```

### Moving Comments (The Challenge)

When reorganizing AST nodes, comments don't automatically move because they're tied to BytePos:

```typescript
// Original code
// Comment for b
export const b = 2;
// Comment for a  
export const a = 1;

// After sorting exports alphabetically, AST nodes are reordered but comments stay at original positions:
// Comment for b    <- Still at original position!
export const a = 1;
// Comment for a    <- Still at original position!
export const b = 2;
```

### Diagram: Comment Storage Architecture

```
Source Code                AST                     Comments Storage
-----------                ---                     ----------------
                                                   
"// Comment"               Module                  SingleThreadedComments {
"const x = 1;"    ------>    |                      leading: {
                             +-- VarDecl               BytePos(11) => [Comment]
                                  |                  },
                                  +-- span            trailing: {}
                                      lo: 11        }
                                      hi: 23
```

### Diagram: The Reordering Problem

```
BEFORE REORDERING:
Source Positions:    0-10        11-20        21-30
                    Comment1     Code1       Comment2     Code2
                       |           |            |           |
Comments Storage:      |           |            |           |
  leading[11] --------+            |            |           |
  leading[31] ---------------------+------------+           |
                                                            |
AST Nodes:            VarDecl1 (span: 11-20)               |
                      VarDecl2 (span: 31-40) <-------------+

AFTER REORDERING (moving VarDecl2 before VarDecl1):
Source Positions:    0-10        11-20        21-30
                    Code2       Code1       [empty]
                      |           |
AST Nodes:           VarDecl2    VarDecl1
                    (span: 31-40) (span: 11-20)  <- Spans unchanged!
                         |              |
Comments Storage:        |              |
  leading[11] ----------+               |  <- Comment1 still here!
  leading[31] --------------------------  <- Comment2 still here!

Result: Comments appear in wrong positions!
```

## Common Pitfalls

### 1. Assuming Comments are Part of Node Spans

```rust
// WRONG: This won't include comments
let node_text = &source_code[node.span.lo.0..node.span.hi.0];

// Comments are stored separately and need separate handling
```

### 2. Forgetting Comments are at Code Start Positions

```typescript
// Source:
// Comment here
const x = 1;

// The comment is NOT at BytePos(0) where it appears
// It's at BytePos(14) where 'const' starts
```

### 3. Not Handling Multiple Comments

```rust
// Multiple comments can exist at the same position
// Always iterate through all comments:
if let Some(comments) = comment_map.get_leading(pos) {
    for comment in comments {  // Don't assume just one!
        // Process each comment
    }
}
```

### 4. Modifying Spans Doesn't Move Comments

```rust
// Changing a node's span doesn't move its comments
node.span = Span::new(BytePos(100), BytePos(200), DUMMY_SP);
// Comments are still at the original positions!
```

## Advanced Techniques

### 1. Comment Association Tracking

To maintain comment associations during transformations:

```rust
struct CommentAssociation {
    node_id: NodeId,
    leading: Vec<Comment>,
    trailing: Vec<Comment>,
}

// Before transformation: collect associations
let associations = collect_comment_associations(&ast, &comments);

// After transformation: reapply comments at new positions
apply_comment_associations(&new_ast, &associations, &mut new_comments);
```

### 2. Comment-Aware Visitors

When using the Visitor pattern, track comment positions:

```rust
impl VisitMut for CommentAwareTransformer {
    fn visit_mut_var_decl(&mut self, node: &mut VarDecl) {
        // Capture comments before transformation
        let leading = self.comments.get_leading(node.span.lo);
        let trailing = self.comments.get_trailing(node.span.hi);
        
        // Transform the node
        node.visit_mut_children_with(self);
        
        // Handle comment migration if needed
        if node.span != old_span {
            // Comments need special handling
        }
    }
}
```

### 3. Creating New Nodes with Comments

When creating synthetic nodes:

```rust
// Create node with DUMMY_SP
let new_node = VarDecl {
    span: DUMMY_SP,  // Placeholder span
    // ... other fields
};

// Calculate actual position in output
let position = calculate_position(&new_node);

// Add comments at the calculated position
comment_map.add_leading(position, Comment {
    kind: CommentKind::Line,
    span: DUMMY_SP,
    text: " Generated code".into(),
});
```

### 4. Comment Preservation Strategies

For tools that need to preserve comments during transformation:

1. **Minimal Transformation**: Only modify what's necessary, preserve spans
2. **Comment Extraction**: Extract all comments before transformation, reapply after
3. **Position Mapping**: Create a map of old positions to new positions
4. **Post-Processing**: Fix comment positions after code generation

## Key Takeaways

1. **Comments are position-based, not node-based**: They're stored at BytePos locations, not attached to AST nodes
2. **Spans are immutable**: AST nodes keep their original spans even when moved
3. **Leading comments are stored at code start**: Not where the comment appears
4. **Multiple comments can exist at one position**: Always handle collections
5. **Transformations require explicit comment handling**: Moving nodes doesn't move comments
6. **SWC's design favors in-place transformations**: Major reordering is challenging

## Why This Design?

SWC's comment system is optimized for:
- **Performance**: Direct position lookup is fast
- **Streaming**: Can emit comments during single-pass code generation
- **Preservation**: Comments maintain exact positions for transformations that don't reorder

The trade-off is that structural transformations (like sorting imports or reordering declarations) require complex comment migration logic that SWC doesn't provide out of the box.

## Further Reading

- [SWC AST Documentation](https://rustdoc.swc.rs/swc_ecma_ast/)
- [SWC Comments Module](https://rustdoc.swc.rs/swc_common/comments/index.html)
- [SWC Parser Implementation](https://github.com/swc-project/swc/tree/main/crates/swc_ecma_parser)