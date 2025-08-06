# TypeScript Support

krokfmt is built specifically for TypeScript and TSX, supporting all modern TypeScript features.

## Supported Features

### Type Annotations

All TypeScript type annotations are preserved and formatted:

```typescript
let count: number = 0;
const user: User | null = null;
function greet(name: string): void {
  console.log(`Hello, ${name}`);
}
```

### Interfaces and Types

Interfaces and type aliases are formatted with sorted properties:

```typescript
interface User {
  email: string;
  id: number;
  name: string;
  roles: Role[];
}

type Status = 'active' | 'inactive' | 'pending';

type ApiResponse<T> = {
  data: T;
  error: string | null;
  status: number;
};
```

### Generics

Generic types and constraints are fully supported:

```typescript
function identity<T>(value: T): T {
  return value;
}

class Container<T extends object> {
  constructor(private value: T) {}
}

type Nullable<T> = T | null;
```

### Decorators

TypeScript decorators are preserved and formatted:

```typescript
@Component({
  selector: 'app-user',
  template: './user.component.html',
})
class UserComponent {
  @Input() user: User;
  @Output() userChange = new EventEmitter<User>();
  
  @Debounce(300)
  handleInput(value: string) {
    // ...
  }
}
```

### Enums

Both regular and const enums are supported:

```typescript
enum Color {
  Blue,
  Green,
  Red,
}

const enum Status {
  Active = 'ACTIVE',
  Inactive = 'INACTIVE',
}
```

### Type Assertions

Type assertions and non-null assertions are preserved:

```typescript
const element = document.getElementById('app') as HTMLDivElement;
const value = data!.property;
```

### Conditional Types

Complex conditional types are formatted properly:

```typescript
type IsString<T> = T extends string ? true : false;
type UnwrapPromise<T> = T extends Promise<infer U> ? U : T;
```

## TSX Support

### JSX Elements

JSX/TSX elements are formatted with proper indentation:

```typescript
const Button: React.FC<ButtonProps> = ({ children, onClick, variant = 'primary' }) => {
  return (
    <button
      className={`btn btn-${variant}`}
      onClick={onClick}
      type="button"
    >
      {children}
    </button>
  );
};
```

### JSX Attributes

JSX attributes are formatted consistently:

```typescript
<Component
  booleanProp
  numberProp={42}
  objectProp={{ key: 'value' }}
  stringProp="text"
  onEvent={() => handleEvent()}
/>
```

### Fragments

React fragments are supported:

```typescript
return (
  <>
    <Header />
    <Main />
    <Footer />
  </>
);
```

## Module Systems

### ES Modules

Full support for ES module syntax:

```typescript
export { default as Button } from './Button';
export * from './types';
export type { User } from './models';
```

### Namespace

TypeScript namespaces are supported:

```typescript
namespace API {
  export interface Response {
    data: any;
    status: number;
  }
}
```

## Advanced Features

- Mapped types
- Template literal types
- Tuple types
- Intersection and union types
- Type guards and predicates
- Const assertions
- Import type and export type