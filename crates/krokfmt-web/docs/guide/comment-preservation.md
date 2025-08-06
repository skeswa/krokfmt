# Comment Preservation

krokfmt intelligently preserves all types of comments while organizing and formatting your code.

## Comment Types

### Inline Comments

Comments within expressions are preserved in their exact positions:

```typescript
const result = calculate(
  value, // The input value
  options /* Configuration object */
);
```

### Leading Comments

Comments before declarations are kept with their associated code:

```typescript
// User authentication service
// Handles login, logout, and session management
class AuthService {
  // ...
}
```

### Trailing Comments

End-of-line comments are preserved:

```typescript
const MAX_RETRIES = 3; // Maximum number of retry attempts
const TIMEOUT = 5000; // Timeout in milliseconds
```

### JSDoc Comments

JSDoc comments are treated as leading comments and stay with their functions:

```typescript
/**
 * Calculates the sum of two numbers
 * @param a First number
 * @param b Second number
 * @returns The sum of a and b
 */
function add(a: number, b: number): number {
  return a + b;
}
```

## Import Comments

Comments in import statements are preserved:

```typescript
import {
  UserService, // Main user service
  AuthService, // Authentication
  // ApiService, // TODO: Uncomment when ready
} from './services';
```

## Block Comments

Multi-line block comments maintain their formatting:

```typescript
/*
 * This is a complex algorithm that requires
 * careful consideration of edge cases.
 * 
 * TODO: Optimize for performance
 * FIXME: Handle null values
 */
function complexAlgorithm() {
  // ...
}
```

## Comment Association

krokfmt uses intelligent heuristics to associate comments with the correct code:

- Comments separated by blank lines may be treated as standalone
- Comments immediately before code are associated with that code
- Comments at the end of lines stay with that line

## Preservation During Reorganization

When krokfmt reorganizes code (like sorting imports or class members), associated comments move with their code:

```typescript
// Before
import { z } from 'zod'; // Schema validation
import React from 'react'; // UI framework

// After
import React from 'react'; // UI framework
import { z } from 'zod'; // Schema validation
```

## Best Practices

- Use JSDoc for function and class documentation
- Keep inline comments brief
- Use leading comments for important notes
- Avoid excessive commenting - let the code speak for itself