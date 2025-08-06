# Formatting Rules

A comprehensive list of all formatting rules applied by krokfmt.

## Import Organization

### Categorization
- **External**: Packages from node_modules
- **Absolute**: Paths starting with @ or ~
- **Relative**: Paths starting with ./ or ../

### Ordering
1. Categories are separated by blank lines
2. Within categories, sorted alphabetically
3. Default imports before named imports
4. Side-effect imports maintain position

## Code Structure

### Indentation
- **Always 2 spaces**
- No tabs allowed
- Consistent throughout file

### Line Length
- **Maximum 100 characters**
- Intelligent breaking for readability
- Preserves string literals when possible

## Syntax Rules

### Semicolons
```typescript
// Always added
const x = 1; // ✓
const y = 2  // ✗ (will add semicolon)
```

### Quotes
```typescript
// Single quotes for strings
const name = 'John'; // ✓
const city = "NYC";  // ✗ (will change to single)

// Double quotes in JSX
<Component attr="value" /> // ✓
<Component attr='value' /> // ✗ (will change to double)
```

### Commas
```typescript
// Trailing commas in multi-line
const obj = {
  a: 1,
  b: 2, // ✓
};

// No trailing comma in single-line
const obj = { a: 1, b: 2 }; // ✓
```

## Object Formatting

### Property Ordering
```typescript
// Alphabetically sorted
const user = {
  age: 30,      // ✓
  email: '...', // ✓
  name: '...',  // ✓
};
```

### Method Shorthand
```typescript
// Preserved as-is
const obj = {
  method() { }, // ✓ Shorthand preserved
  prop: () => { }, // ✓ Arrow preserved
};
```

## Array Formatting

### Multi-line Arrays
```typescript
const items = [
  'apple',
  'banana',
  'cherry', // ✓ Trailing comma
];
```

### Single-line Arrays
```typescript
const nums = [1, 2, 3]; // ✓ No trailing comma
```

## Function Formatting

### Parameters
```typescript
// Multi-line when many parameters
function create(
  name: string,
  age: number,
  email: string,
) { }

// Single-line when few
function greet(name: string) { }
```

### Arrow Functions
```typescript
// Parentheses always for clarity
const fn = () => { };      // ✓
const add = (a, b) => a + b; // ✓
```

### Destructuring
```typescript
// Properties sorted alphabetically
function Component({ className, id, onClick, title }) { }
```

## Class Formatting

### Member Ordering
1. Static properties
2. Static methods
3. Instance properties
4. Constructor
5. Public methods
6. Protected methods
7. Private methods

### Access Modifiers
```typescript
class Example {
  public prop = 1;    // ✓ Explicit
  private _prop = 2;  // ✓ Explicit
  method() { }        // ✓ Implicit public
}
```

## Type Formatting

### Interface Properties
```typescript
// Alphabetically sorted
interface User {
  age: number;
  email: string;
  id: string;
  name: string;
}
```

### Union Types
```typescript
// Formatted on single line when short
type Status = 'active' | 'inactive' | 'pending';

// Multi-line when long
type LongUnion =
  | 'very-long-option-one'
  | 'very-long-option-two'
  | 'very-long-option-three';
```

## JSX/TSX Formatting

### Self-closing Tags
```typescript
<Component /> // ✓ Space before />
<Component/>  // ✗ Will add space
```

### Props
```typescript
// Multi-line when many props
<Component
  className="btn"
  id="submit"
  onClick={handleClick}
  type="button"
/>

// Single-line when few
<Button onClick={handleClick}>Text</Button>
```

### Conditional Rendering
```typescript
// Ternary formatting
{condition ? (
  <ComponentA />
) : (
  <ComponentB />
)}

// Logical AND
{isVisible && <Component />}
```

## Comment Formatting

### Inline Comments
```typescript
const x = 1; // Preserved at end of line
const y = /* inline */ 2; // Preserved inline
```

### Block Comments
```typescript
/**
 * JSDoc comments preserved
 * with formatting intact
 */
function documented() { }
```

## Whitespace Rules

### Binary Operators
```typescript
const sum = a + b;  // ✓ Spaces around operator
const sum = a+b;    // ✗ Will add spaces
```

### Unary Operators
```typescript
!value    // ✓ No space
++count   // ✓ No space
```

### Keywords
```typescript
if (condition) { }     // ✓ Space after keyword
for (let i = 0; ...) { } // ✓ Space after keyword
```

## Special Cases

### Template Literals
```typescript
// Preserved as-is
const msg = `Hello ${name}`;
const sql = `
  SELECT *
  FROM users
  WHERE active = true
`;
```

### Regex
```typescript
// Preserved as-is
const pattern = /^[a-z]+$/gi;
```

### Empty Blocks
```typescript
// Formatted consistently
function empty() {}
class Empty {}
if (condition) {}
```

## Non-Rules

Things krokfmt does NOT change:

1. **Variable names** - Never renamed
2. **Logic** - Never altered
3. **Comments** - Never removed
4. **String content** - Never modified
5. **Number formats** - Preserved as written

## Precedence

When rules conflict:
1. **Correctness** over style
2. **Readability** over brevity
3. **Consistency** over preference
4. **Preservation** over modification