# Member Ordering

krokfmt automatically orders class members and object properties for consistency.

## Class Member Ordering

Class members are ordered by:

1. **Static members** before instance members
2. **Properties** before methods
3. **Public** members before private members
4. **Constructor** always comes after properties
5. **Lifecycle methods** in React components follow React conventions

### Example

```typescript
// Before
class UserService {
  private users: User[] = [];
  constructor(private api: ApiClient) {}
  static getInstance() { /* ... */ }
  public getUser(id: string) { /* ... */ }
  private validateUser(user: User) { /* ... */ }
  static VERSION = '1.0.0';
}

// After
class UserService {
  static VERSION = '1.0.0';
  
  private users: User[] = [];
  
  static getInstance() { /* ... */ }
  
  constructor(private api: ApiClient) {}
  
  public getUser(id: string) { /* ... */ }
  
  private validateUser(user: User) { /* ... */ }
}
```

## Object Property Ordering

Object properties are sorted alphabetically:

```typescript
// Before
const config = {
  timeout: 5000,
  baseUrl: 'https://api.example.com',
  headers: { 'Content-Type': 'application/json' },
  retry: 3,
};

// After
const config = {
  baseUrl: 'https://api.example.com',
  headers: { 'Content-Type': 'application/json' },
  retry: 3,
  timeout: 5000,
};
```

## Function Parameters

Object destructuring parameters are also sorted alphabetically:

```typescript
// Before
function createUser({ name, email, id, role }: UserProps) {
  // ...
}

// After
function createUser({ email, id, name, role }: UserProps) {
  // ...
}
```

## Interface and Type Properties

Interface and type properties are sorted alphabetically:

```typescript
// Before
interface User {
  name: string;
  email: string;
  id: number;
  createdAt: Date;
}

// After
interface User {
  createdAt: Date;
  email: string;
  id: number;
  name: string;
}
```

## Exceptions

- Array elements are never reordered
- Function call arguments are never reordered
- JSX props maintain their original order for readability