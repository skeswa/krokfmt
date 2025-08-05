// FR2.1: Complex export detection patterns

// Re-exports
export { foo } from './foo';
export { bar as baz } from './bar';
export * from './utils';
export * as helpers from './helpers';

// Mixed export patterns
const internal = 'internal';
export const external = 'external';

// Export list at bottom
const item1 = 'one';
const item2 = 'two';
const item3 = 'three';
export { item1, item2 };

// Type exports
type InternalType = string;
export type ExportedType = InternalType;

// Interface exports
interface InternalInterface {
    x: number;
}
export interface ExportedInterface extends InternalInterface {
    y: number;
}

// Namespace exports
namespace InternalNS {
    export const value = 42;
}
export namespace ExportedNS {
    export const value = InternalNS.value;
}

// Const assertions with exports
const VALUES = ['a', 'b', 'c'] as const;
export type Value = typeof VALUES[number];