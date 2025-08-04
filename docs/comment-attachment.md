# The Comment Attachment Problem in krokfmt: A Comprehensive Guide

## Table of Contents

1. [Overview](#overview)
2. [The Problem](#the-problem)
3. [Understanding SWC's Architecture](#understanding-swcs-architecture)
4. [Why Traditional Solutions Don't Work](#why-traditional-solutions-dont-work)
5. [Attempted Solutions](#attempted-solutions)
6. [The Two-Phase Approach](#the-two-phase-approach)
7. [Technical Deep Dive](#technical-deep-dive)
8. [Code Examples](#code-examples)
9. [Lessons Learned](#lessons-learned)
10. [Future Directions](#future-directions)

## Overview

One of the most challenging technical issues in krokfmt is the inability to properly preserve comment associations when reorganizing code through AST transformations. This document provides a comprehensive guide to understanding the problem, why it exists, the various solutions that have been attempted, and the innovative two-phase approach we designed to work around SWC's limitations.

## The Problem

When krokfmt reorganizes code (e.g., sorting imports, reordering exports, alphabetizing class members), comments that should logically move with their associated code remain at their original positions. This results in comments appearing in the wrong places or being associated with the wrong code elements.

### Example

Input:
```typescript
// Comment for export b
export const b = 2;

// Comment for export a  
export const a = 1;
```

Expected output (after alphabetical sorting):
```typescript
// Comment for export a
export const a = 1;

// Comment for export b
export const b = 2;
```

Actual output:
```typescript
// Comment for export b
export const a = 1;

// Comment for export a
export const b = 2;
```

## Understanding SWC's Architecture

To understand why this problem exists, we need to dive deep into how SWC (Speedy Web Compiler) handles comments and AST structures.

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

### Key SWC Concepts

#### 1. Spans
Every AST node in SWC has a `span` field that indicates where in the source code the node appears:

```rust
pub struct Span {
    pub lo: BytePos,  // Start position (byte offset)
    pub hi: BytePos,  // End position (byte offset)
    pub ctxt: SyntaxContext,  // Hygiene information
}
```

**Important**: Spans are immutable and do NOT include comments - only the actual code.

#### 2. Comment Storage

**Comments are NOT part of the AST nodes**. Instead, they are stored in a separate data structure called `SingleThreadedComments`:

```rust
pub struct SingleThreadedComments {
    leading: RefCell<FxHashMap<BytePos, Vec<Comment>>>,
    trailing: RefCell<FxHashMap<BytePos, Vec<Comment>>>,
}
```

### The Key Insight: Position-Based Storage

Comments are stored at the BytePos where their associated code STARTS, not where the comment itself appears:

```typescript
// Source with byte positions marked
// 00000000001111111111222222222233333333334
// 01234567890123456789012345678901234567890
   // This is a comment      <- Comment at bytes 0-20
   const x = 1;              <- Code starts at byte 23

// The comment is stored at BytePos(23), NOT BytePos(0)!
```

```mermaid
graph LR
    subgraph "Source File"
        direction TB
        Bytes["Byte Positions"]
        Comment["0-20: // This is a comment"]
        Code["23-35: const x = 1;"]
        
        Bytes --> Comment
        Bytes --> Code
    end
    
    subgraph "Comment Storage"
        Map["SingleThreadedComments<br/>HashMap"]
        Entry["BytePos(23) => [Comment]"]
        
        Map --> Entry
    end
    
    Comment -.->|"Stored at code start position"| Entry
    Code -.->|"Start position: 23"| Entry
    
    style Comment fill:#fcc,stroke:#333,stroke-width:2px
    style Code fill:#cfc,stroke:#333,stroke-width:2px
    style Entry fill:#ccf,stroke:#333,stroke-width:2px
```

### How SWC's Comment System Works

1. **Parsing Phase**:
   - Comments are extracted and stored in a `SingleThreadedComments` structure
   - Each comment is associated with a BytePos (byte position in source)
   - Comments are categorized as "leading" or "trailing" based on position

2. **AST Structure**:
   - Each AST node has a `span` field containing start and end positions
   - These positions refer to the original source code locations
   - Spans are immutable and part of the node's identity

3. **Code Generation**:
   - The emitter walks the AST and generates code
   - At each node, it checks for comments at that node's span positions
   - Comments are emitted based on their stored BytePos

## Why Traditional Solutions Don't Work

### The Intuitive Approach That Fails

Many developers (including us) have tried this seemingly reasonable approach:

1. **Read all comments** - Walk the AST and collect all leading/trailing comments for each node
2. **Build association map** - Create a map of node → comments
3. **Remove comments** - Clear the comment storage
4. **Rearrange AST** - Reorder nodes as needed
5. **Reassociate comments** - Add comments back based on the association map

### Why It Fails: Immutable Spans

Here's a concrete example to illustrate the issue:

```typescript
// Original source file:
// Comment for A
const a = 1;
// Comment for B
const b = 2;
```

#### Initial State
```rust
// AST nodes with their spans (immutable!)
const_a: VarDecl { span: Span { lo: BytePos(16), hi: BytePos(28) }, ... }
const_b: VarDecl { span: Span { lo: BytePos(44), hi: BytePos(56) }, ... }

// Comment storage
leading[BytePos(16)] = ["Comment for A"]
leading[BytePos(44)] = ["Comment for B"]
```

#### After Reordering (Swapping Order)
```rust
// After reordering in the AST vector:
module.body = vec![const_b, const_a];

// BUT the spans are still:
const_b: VarDecl { span: Span { lo: BytePos(44), hi: BytePos(56) }, ... }
const_a: VarDecl { span: Span { lo: BytePos(16), hi: BytePos(28) }, ... }
```

#### The Problem
When trying to reassociate comments:
```rust
// We want to add "Comment for B" to const_b, which is now first
// But const_b.span.lo is still BytePos(44)!

comments.add_leading(const_b.span.lo, "Comment for B");
// This adds the comment at BytePos(44), not at the beginning of the file!
```

### Visual Representation of the Problem

```mermaid
graph TB
    subgraph "What We Want"
        W1["Output Position 0: // Comment for B"]
        W2["Output Position 16: const b = 2;"]
        W3["Output Position 28: // Comment for A"]
        W4["Output Position 44: const a = 1;"]
    end
    
    subgraph "What Actually Happens"
        A1["Output Position 0: const b = 2;"]
        A2["Output Position 12: const a = 1;"]
        A3["Output Position 24: (empty)"]
        A4["Output Position 44: // Comment for B"]
    end
    
    subgraph "Why"
        R1["const_b.span still says BytePos(44)"]
        R2["Comments added at span positions"]
        R3["Emitter respects original spans"]
    end
    
    W1 -.->|"❌ Can't achieve"| A1
    R1 --> A4
    R2 --> A4
    R3 --> A4
    
    style W1 fill:#cfc,stroke:#333,stroke-width:2px
    style W2 fill:#cfc,stroke:#333,stroke-width:2px
    style A1 fill:#fcc,stroke:#333,stroke-width:2px
    style A4 fill:#fcc,stroke:#333,stroke-width:2px
```

## Attempted Solutions

Over the course of development, we attempted numerous solutions to this problem. Each taught us valuable lessons about the constraints we were working with.

### 1. Comment Map Tracking (July 2025)

**Approach**: Build a map of AST nodes to their associated comments before reorganization, then restore the associations afterward.

**Implementation**:
- Created mappings between node identities and their comments
- After AST transformation, tried to update comment positions

**Why it failed**: SWC's emitter still uses the original BytePos values from the AST nodes. Even though we knew which comments belonged to which nodes, we couldn't change where they were emitted.

### 2. Clear and Re-add Comments

**Approach**: Remove all comments from the comment store and re-add them at new positions after transformation.

**Implementation**:
```rust
// Clear all comments
comments.clear();
// After transformation, add comments at new positions
comments.add_leading(new_pos, comment);
```

**Why it failed**: The AST nodes still contained their original spans. Comments added at "new" positions were still emitted at the original locations because the emitter uses the span information from the AST nodes.

### 3. Synthetic Spans

**Approach**: Create new spans for reorganized AST nodes to reflect their new positions.

**Why it failed**: SWC's AST nodes are designed to be immutable. The span information is deeply embedded in the node structure and cannot be modified without reconstructing the entire AST.

### 4. Manual Comment Emission

**Approach**: Disable SWC's built-in comment emission and handle it manually during code generation.

**Why it failed**: This would require reimplementing SWC's entire comment emission system, including:
- Proper indentation handling
- Line break management
- Comment style preservation (line vs block comments)
- Integration with source maps

The complexity was prohibitive and would likely introduce more bugs than it solved.

### 5. Post-processing String Manipulation

**Approach**: Let SWC generate code with misplaced comments, then fix them with string manipulation.

**Why it failed**: 
- Too fragile - would need complex pattern matching for every possible scenario
- Would break source maps
- Could introduce syntax errors if not done perfectly
- Performance impact of parsing generated code

### 6. Comment Attacher Module (August 2025)

**Approach**: Create a sophisticated system to track comment associations through AST transformations.

**Implementation**:
```rust
pub struct CommentAttacher {
    node_comments: HashMap<NodeIdentity, Vec<AttachedComment>>,
    floating_comments: Vec<Comment>,
}

pub struct NodeIdentity {
    node_type: String,      // e.g., "FunctionDecl"
    name: Option<String>,   // e.g., function name
    context: String,        // Additional disambiguation
}
```

**Why it failed**: While the infrastructure was sound, the fundamental BytePos issue remained. Comments were duplicated, placed inline instead of on separate lines, or lost entirely.

### 7. Deep Cloning with Span Updates (August 2025)

**Approach**: Deep clone the AST with updated spans to reflect new positions.

**Implementation**:
1. Created `span_calculator.rs` to calculate new BytePos positions
2. Created `ast_rebuilder.rs` to deep clone the AST with updated spans
3. Modified the formatter to use this approach when comments are present

**Why it failed**: 
- While we successfully updated all spans in the AST, the comments are still stored in the original `SingleThreadedComments` structure with their original BytePos associations
- Simply updating node spans doesn't move the comments
- The comment store would need to be rebuilt with new BytePos mappings

### 8. Comment Migration (August 2025)

**Approach**: Building on span updating, attempt to migrate comments to their new positions.

**Implementation**:
1. Created `comment_migrator.rs` to move comments from old BytePos to new BytePos
2. Attempted to extract comments and re-add them at new positions

**Why it failed**:
- `SingleThreadedComments` doesn't provide a public API to iterate through all comments
- No way to get a list of all positions that have comments
- The internal storage is private and can't be accessed directly
- Would require O(n) scanning where n is file size in bytes

## The Two-Phase Approach

After exhausting traditional approaches, we designed an innovative two-phase solution that works around SWC's limitations.

### High-Level Overview

```mermaid
graph TD
    A[Original Source] --> B[Parse with SWC]
    B --> C[Extract Comments with Semantic Hashes]
    B --> D[AST with Comments]
    D --> E[Strip Comments from AST]
    E --> F[Transform AST without Comments]
    F --> G[Generate Code without Comments]
    G --> H[Parse Generated Code for Node Positions]
    H --> I[Match Nodes to Semantic Hashes]
    C --> J[Reinsert Comments at Correct Positions]
    I --> J
    J --> K[Final Code with Correctly Placed Comments]
```

### Phase 1: Initial Parsing and Comment Extraction

1. Parse the source file using SWC
2. Walk the AST and for each node:
   - Generate a semantic hash (stable identifier)
   - Extract associated leading and trailing comments
   - Store mapping: `semantic_hash → comments[]`

### Phase 2: Comment-Free Transformation

1. Create a new `SingleThreadedComments` instance (empty)
2. Run normal krokfmt transformations on the AST
3. Generate code without any comments

### Phase 3: Comment Reinsertion

1. Parse the generated code to identify node positions
2. Walk the new AST and generate semantic hashes
3. Match hashes to extract original comments
4. Insert comments at appropriate positions in the string

### Semantic Hashing Algorithm

The semantic hash must be stable across AST transformations while being unique enough to avoid collisions.

```rust
struct SemanticHash {
    node_type: NodeType,        // Function, Class, Const, etc.
    name: Option<String>,       // Identifier if available
    signature: Option<String>,  // For functions: param types
    parent_hash: Option<u64>,   // Parent node's hash (for context)
}
```

Examples:
```typescript
// Hash: { type: "FunctionDecl", name: "processUser", signature: "(User) => void" }
export function processUser(user: User): void { }

// Hash: { type: "ClassDecl", name: "UserService", parent: null }
class UserService {
    // Hash: { type: "Method", name: "getUser", signature: "(number) => User", parent: "UserService" }
    getUser(id: number): User { }
}
```

## Technical Deep Dive

### The Reordering Problem Visualized

#### Before Reordering

```mermaid
graph TB
    subgraph "Source Positions"
        P1["0-10<br/>Comment1"]
        P2["11-20<br/>Code1"]
        P3["21-30<br/>Comment2"]
        P4["31-40<br/>Code2"]
    end
    
    subgraph "AST Nodes"
        V1["VarDecl1<br/>span: 11-20"]
        V2["VarDecl2<br/>span: 31-40"]
    end
    
    subgraph "Comments Storage"
        C1["leading[11] = Comment1"]
        C2["leading[31] = Comment2"]
    end
    
    P1 -.->|"Stored at"| C1
    P2 -.->|"AST node"| V1
    P3 -.->|"Stored at"| C2
    P4 -.->|"AST node"| V2
    
    C1 -->|"Associated with"| V1
    C2 -->|"Associated with"| V2
    
    style P1 fill:#fcc,stroke:#333,stroke-width:2px
    style P3 fill:#fcc,stroke:#333,stroke-width:2px
    style C1 fill:#cfc,stroke:#333,stroke-width:2px
    style C2 fill:#cfc,stroke:#333,stroke-width:2px
```

#### After Reordering (Moving VarDecl2 before VarDecl1)

```mermaid
graph TB
    subgraph "Rendered Output"
        R1["Position 1<br/>Code2"]
        R2["Position 2<br/>Code1"]
    end
    
    subgraph "AST Nodes"
        V2["VarDecl2<br/>span: 31-40<br/>⚠️ Unchanged!"]
        V1["VarDecl1<br/>span: 11-20<br/>⚠️ Unchanged!"]
    end
    
    subgraph "Comments Storage"
        C1["leading[11] = Comment1<br/>❌ Wrong position!"]
        C2["leading[31] = Comment2<br/>❌ Wrong position!"]
    end
    
    R1 -.->|"Rendered from"| V2
    R2 -.->|"Rendered from"| V1
    
    C1 -->|"Still at BytePos(11)"| V1
    C2 -->|"Still at BytePos(31)"| V2
    
    style V2 fill:#faa,stroke:#333,stroke-width:3px
    style V1 fill:#faa,stroke:#333,stroke-width:3px
    style C1 fill:#fcc,stroke:#333,stroke-width:2px
    style C2 fill:#fcc,stroke:#333,stroke-width:2px
```

**Result**: Comments appear in wrong positions because comment positions are tied to the original BytePos values in the spans.

### Why This Architecture Exists

SWC's design makes sense for its primary use case: transforming code while preserving its structure. Most transformations (transpiling, minifying, etc.) modify code in-place without reordering. The position-based system works perfectly for these scenarios.

The trade-off is that structural transformations (like sorting imports or reordering declarations) require complex comment migration logic that SWC doesn't provide out of the box.

## Code Examples

### Reading Comments in Practice

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

### The Core Problem Demonstrated

```rust
/// The core problem - why moving nodes doesn't move comments
fn demonstrate_reordering_problem() {
    // Original AST and comments:
    // Node A at BytePos(10-20), comment at leading[10]
    // Node B at BytePos(30-40), comment at leading[30]
    
    // After swapping nodes in AST:
    // Node B still has span(30-40) - span is immutable!
    // Node A still has span(10-20) - span is immutable!
    
    // So comments remain at:
    // leading[10] - originally for A, but now B is rendered first
    // leading[30] - originally for B, but now A is rendered second
    
    // Result: Comments appear with wrong nodes!
}
```

### Comment-Aware Visitor Pattern

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

### Tracking Comment Associations

```rust
struct CommentTracker {
    comments: SingleThreadedComments,
    position_map: std::collections::HashMap<BytePos, BytePos>,
}

impl CommentTracker {
    /// Collect all comments before transformation
    fn collect_comments(&self, span: Span) -> (Vec<Comment>, Vec<Comment>) {
        let leading = self.comments.get_leading(span.lo)
            .map(|c| c.to_vec())
            .unwrap_or_default();
        let trailing = self.comments.get_trailing(span.hi)
            .map(|c| c.to_vec())
            .unwrap_or_default();
        (leading, trailing)
    }
    
    /// Reapply comments at new positions
    fn apply_comments(&self, old_span: Span, new_span: Span, new_comments: &SingleThreadedComments) {
        // Get comments from old position
        let (leading, trailing) = self.collect_comments(old_span);
        
        // Add to new position
        for comment in leading {
            new_comments.add_leading(new_span.lo, comment);
        }
        for comment in trailing {
            new_comments.add_trailing(new_span.hi, comment);
        }
    }
}
```

## Lessons Learned

1. **Architecture Matters**: The choice of parser/AST library has profound implications that may not be apparent initially
2. **Comments are First-Class**: In a formatter, comments are as important as code and need first-class support
3. **Position vs. Structure**: Position-based systems are fundamentally incompatible with structural transformations
4. **Open Source Limitations**: When building on open-source tools, their architectural decisions become your constraints
5. **The Value of Documentation**: Understanding the underlying architecture is crucial for avoiding dead ends
6. **Innovation Through Constraints**: Sometimes the best solutions come from working around limitations rather than fighting them

## Future Directions

### Potential Future Solutions

1. **Fork SWC**: Create a custom fork that supports node-based comment association instead of position-based
2. **Alternative Parser**: Switch to a different TypeScript parser that supports comment preservation during AST transformation
3. **Hybrid Approach**: Use SWC for parsing but implement our own code generation that properly handles comments
4. **Limited Reordering**: Restrict the formatter to transformations that don't require moving comments across significant distances

### Success Metrics for Any Solution

1. **Correctness**: 100% of comments remain with their nodes
2. **Performance**: <2x slowdown vs. regular formatting
3. **Compatibility**: Works with all TypeScript syntax
4. **Reliability**: Graceful fallback on edge cases

### Impact on krokfmt Features

This limitation affects several key features:
- **FR1.4**: Import positioning - comments on imports don't move when imports are reordered
- **FR2.***: Export/visibility organization - comments on exported members stay in original positions
- **FR6.5**: Comment association - the core requirement that comments move with their code
- **FR3.3**: Class member ordering - comments on class members don't move during reordering

## Conclusion

The comment attachment problem represents a fundamental mismatch between krokfmt's requirements (reordering code while preserving comment associations) and SWC's architecture (position-based comment handling). While various creative solutions have been attempted, the two-phase approach provides the most promising path forward.

This journey serves as an important reminder that tool selection in the early stages of a project can have lasting implications. For future formatter projects, ensuring the parser/AST library supports comment preservation during transformation should be a key evaluation criterion.

The two-phase comment replacement approach, while adding complexity and overhead, enables correct comment preservation during code reorganization without requiring changes to SWC itself. It demonstrates that sometimes the best engineering solutions come not from fighting constraints but from creatively working within them.

For now, krokfmt users should be aware that full comment preservation during code reorganization is a work in progress. The two-phase approach shows promise but requires additional implementation and testing before it can be considered production-ready.