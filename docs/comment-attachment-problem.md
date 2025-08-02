# The Comment Attachment Problem in krokfmt

## Overview

One of the most challenging technical issues in krokfmt is the inability to properly preserve comment associations when reorganizing code through AST transformations. This document details the nature of the problem, why it exists, and the various solutions that have been attempted.

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

## Root Cause

The fundamental issue stems from how SWC (the TypeScript parser we use) handles comments:

1. **Position-Based Storage**: Comments are stored separately from the AST using byte positions (`BytePos`) in the original source code
2. **Immutable Spans**: When AST nodes are reorganized, they retain their original position information (spans)
3. **Position-Based Emission**: During code generation, comments are emitted based on their original byte positions, not their logical association with AST nodes

This architecture makes sense for transformations that modify code in-place but breaks down completely when nodes are reordered.

## Attempted Solutions

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

**Implementation**: Attempted to modify node spans after reorganization to match their new locations in the file.

**Why it failed**: SWC's AST nodes are designed to be immutable. The span information is deeply embedded in the node structure and cannot be modified without reconstructing the entire AST.

### 4. Manual Comment Emission

**Approach**: Disable SWC's built-in comment emission and handle it manually during code generation.

**Implementation**: Tried to intercept the code generation process and inject comments at appropriate positions.

**Why it failed**: This would require reimplementing SWC's entire comment emission system, including:
- Proper indentation handling
- Line break management
- Comment style preservation (line vs block comments)
- Integration with source maps

The complexity was prohibitive and would likely introduce more bugs than it solved.

### 5. Post-processing String Manipulation

**Approach**: Let SWC generate code with misplaced comments, then fix them with string manipulation.

**Implementation**: Parse the generated code to identify misplaced comments and move them to the correct locations.

**Why it failed**: 
- Too fragile - would need complex pattern matching for every possible scenario
- Would break source maps
- Could introduce syntax errors if not done perfectly
- Performance impact of parsing generated code

### 6. Comment Attacher Module (August 2025)

**Approach**: Create a sophisticated system to track comment associations through AST transformations.

**Implementation**:
- Created `comment_attacher.rs` module
- Implemented node identity tracking (type + name + context)
- Built comment association mappings before transformation
- Attempted to reattach comments after transformation

**Code Structure**:
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

**Why it failed**: While the infrastructure was sound, the fundamental BytePos issue remained:
- Comments were duplicated in the output
- Some comments were placed inline instead of on separate lines
- Certain comments were lost entirely
- The core issue of position-based emission couldn't be overcome

## Technical Deep Dive

### How SWC's Comment System Works

1. **Parsing Phase**:
   - Comments are extracted and stored in a `SingleThreadedComments` structure
   - Each comment is associated with a BytePos (byte position in source)
   - Comments are categorized as "leading" or "trailing" based on position

2. **AST Structure**:
   - Each AST node has a `span` field containing start and end positions
   - These positions refer to the original source code locations
   - **Important**: Spans do NOT include comments - only the actual code
   - Spans are immutable and part of the node's identity

3. **Comment Storage**:
   - Comments are NOT stored at their own positions in the source
   - Instead, they are stored at the BytePos where the associated code STARTS
   - For example, a comment on lines 1-2 before code starting at line 3 is stored at the BytePos of line 3
   - This means comments are already "attached" to specific code positions

4. **Code Generation**:
   - The emitter walks the AST and generates code
   - At each node, it checks for comments at that node's span positions
   - Comments are emitted based on their stored BytePos, which corresponds to code positions

### Why This Architecture Exists

SWC's design makes sense for its primary use case: transforming code while preserving its structure. Most transformations (transpiling, minifying, etc.) modify code in-place without reordering. The position-based system works perfectly for these scenarios.

## Impact on krokfmt

This limitation affects several key features:

1. **FR1.4**: Import positioning - comments on imports don't move when imports are reordered
2. **FR2.***: Export/visibility organization - comments on exported members stay in original positions
3. **FR6.5**: Comment association - the core requirement that comments move with their code
4. **FR3.3**: Class member ordering - comments on class members don't move during reordering

## Current Status

As of August 2025, this remains an unresolved issue. The infrastructure for comment attachment exists in the codebase but is not active due to the fundamental limitations described above.

## New Insights (August 2025)

After further investigation, we discovered that comments are NOT part of AST node spans. This is actually helpful because:

1. **Comments are already associated with code positions**: They're stored at the BytePos where the associated code starts
2. **Node spans are clean**: They only contain the actual code, making transformations cleaner
3. **The real issue is span immutability**: When we move nodes, we can't update their spans to reflect new positions

This suggests a possible approach: instead of trying to move comments with nodes, we could:
1. Generate new spans for moved nodes based on their new positions
2. Rebuild the AST with updated spans
3. Let the comment system naturally emit comments at the new positions

However, this would require either:
- Deep cloning and rebuilding the entire AST with new spans
- A way to override span information during code generation

## Attempted Solution 7: Deep Cloning with Span Updates (August 2025)

After discovering that comments are not part of AST node spans, we attempted a sophisticated approach:

**Implementation**:
1. Created `span_calculator.rs` to calculate new BytePos positions for nodes after reorganization
2. Created `ast_rebuilder.rs` to deep clone the AST with updated spans
3. Modified the formatter to use this approach when comments are present

**Why it failed**: 
- While we successfully updated all spans in the AST, the comments are still stored in the original `SingleThreadedComments` structure with their original BytePos associations
- Simply updating node spans doesn't move the comments - they need to be explicitly migrated
- The comment store would need to be rebuilt with new BytePos mappings
- Even with updated spans, SWC's emitter still looks for comments at the original positions stored in the comment map

**Key Learning**: The problem has two parts:
1. Updating AST node spans (which we can do)
2. Migrating comments to new BytePos positions in the comment store (which is complex)

Without solving both parts, comments remain at their original positions.

## Attempted Solution 8: Comment Migration (August 2025)

Building on the span updating approach, we attempted to migrate comments to their new positions:

**Implementation**:
1. Created `comment_migrator.rs` to move comments from old BytePos to new BytePos
2. Attempted to extract comments from SingleThreadedComments and re-add them at new positions
3. Updated formatter to use both AST rebuilding and comment migration

**Why it failed**:
- `SingleThreadedComments` doesn't provide a public API to iterate through all comments
- The `Comments` trait methods like `get_leading` and `add_leading` are available, but:
  - We can only query comments at known positions (we'd need to scan every BytePos)
  - There's no way to get a list of all positions that have comments
  - The internal storage is private and can't be accessed directly
- Even if we could iterate all comments, we'd need to know the maximum BytePos to scan
- The approach would be O(n) where n is the file size in bytes, making it impractical

**Key Learning**: SWC's comment system is designed for in-place transformations. The lack of iteration APIs makes it effectively impossible to migrate comments to new positions when nodes are reordered.

## Potential Future Solutions

### 1. Fork SWC
Create a custom fork of SWC that supports node-based comment association instead of position-based. This would be a massive undertaking requiring:
- Redesigning the comment storage system
- Modifying the parser to attach comments directly to AST nodes  
- Updating the code generator to emit comments from nodes
- Maintaining compatibility with SWC updates

### 2. Alternative Parser
Switch to a different TypeScript parser that supports comment preservation during AST transformation. Options might include:
- TypeScript Compiler API (if it supports comment preservation)
- Babel with custom plugins
- A custom parser built specifically for formatting

### 3. Hybrid Approach
Use SWC for parsing but implement our own code generation that properly handles comments. This would require:
- Building a complete TypeScript code generator
- Implementing proper comment placement logic
- Maintaining all TypeScript syntax support

### 4. Limited Reordering
Restrict the formatter to transformations that don't require moving comments across significant distances. This would mean:
- No import reordering
- No export reorganization  
- Limited to in-place transformations only

## Lessons Learned

1. **Architecture Matters**: The choice of parser/AST library has profound implications that may not be apparent initially
2. **Comments are First-Class**: In a formatter, comments are as important as code and need first-class support
3. **Position vs. Structure**: Position-based systems are fundamentally incompatible with structural transformations
4. **Open Source Limitations**: When building on open-source tools, their architectural decisions become your constraints

## Related Documentation

For a comprehensive understanding of how SWC handles comments and AST structures, see:
- [SWC AST and Comment Handling Guide](./swc-ast-and-comments-guide.md) - Detailed explanation with examples
- [SWC Comment Example](./swc-comment-example.ts) - Annotated TypeScript showing byte positions
- [SWC Comment Manipulation Example](./swc-comment-manipulation-example.rs) - Rust code examples

## Conclusion

The comment attachment problem represents a fundamental mismatch between krokfmt's requirements (reordering code while preserving comment associations) and SWC's architecture (position-based comment handling). While various creative solutions have been attempted, none have successfully overcome the core limitation.

This serves as an important reminder that tool selection in the early stages of a project can have lasting implications. For future formatter projects, ensuring the parser/AST library supports comment preservation during transformation should be a key evaluation criterion.

For now, krokfmt users should be aware that comments may not move with their associated code during certain transformations. This is documented as a known limitation in the tool's documentation.