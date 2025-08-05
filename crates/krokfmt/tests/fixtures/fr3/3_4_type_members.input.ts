// FR3.4: Type members should be sorted alphabetically

// Union types should be sorted
type Status = 'error' | 'success' | 'pending' | 'loading';

// Intersection types should be sorted
type Combined = TypeZ & TypeA & TypeM & TypeB;

// Complex union types
type Primitive = string | number | boolean | null | undefined | symbol | bigint;

// Object literal unions
type Action = 
    | { type: 'LOAD'; payload: string }
    | { type: 'ERROR'; error: Error }
    | { type: 'SUCCESS'; data: any }
    | { type: 'RESET' };

// Mixed type unions
type Mixed = 
    | string 
    | { kind: 'object' } 
    | number[] 
    | boolean 
    | (() => void);