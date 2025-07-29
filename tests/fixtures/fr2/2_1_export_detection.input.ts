// FR2.1: Exported members should be detected and prioritized

// Non-exported function
function privateHelper() {
    return 'private';
}

// Exported function
export function publicApi() {
    return privateHelper() + ' api';
}

// Non-exported class
class PrivateClass {
    value = 1;
}

// Exported class
export class PublicClass {
    instance = new PrivateClass();
}

// Non-exported interface
interface PrivateInterface {
    id: number;
}

// Exported interface
export interface PublicInterface extends PrivateInterface {
    name: string;
}

// Non-exported type
type PrivateType = string | number;

// Exported type
export type PublicType = PrivateType | boolean;

// Named exports
const a = 1;
const b = 2;
export { a, b };