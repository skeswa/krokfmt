// Test file for type member sorting

// Union types with string literals
type Status = 'error' | 'success' | 'pending' | 'idle' | 'loading';
type Size = 'xl' | 'sm' | 'lg' | 'md' | 'xs' | 'xxl' | 'xxs';
type Color = 'red' | 'blue' | 'green' | 'yellow' | 'purple' | 'orange';

// Union types with type references
type Primitive = string | number | boolean | symbol | bigint | null | undefined;
type ReactNode = ReactElement | string | number | ReactFragment | ReactPortal | boolean | null | undefined;

// Intersection types
type User = Timestamped & Identifiable & Versioned & Serializable;
type Component = Styleable & Themeable & Accessible & Interactive;
type ApiResponse = Paginated & Sortable & Filterable & Cacheable;

// Complex nested types
type ComplexUnion = 
    | { type: 'user'; data: User }
    | { type: 'admin'; data: Admin }
    | { type: 'guest'; data: Guest }
    | { type: 'moderator'; data: Moderator };

// Mixed literal types  
type Mixed = 'active' | 'inactive' | 1 | 2 | 3 | true | false;

// Type with generics and constraints
type Handler<T extends Event> = MouseHandler<T> & KeyboardHandler<T> & TouchHandler<T> & FocusHandler<T>;