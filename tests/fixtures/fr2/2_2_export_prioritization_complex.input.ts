// FR2.2: Complex export prioritization with dependencies

// Deep dependency chain
const level1 = 'base';
const level2 = level1 + '_extended';
const level3 = level2 + '_final';
export const publicLevel3 = level3;

// Mixed private and public chain
function privateHelper() {
    return 42;
}

export function publicHelper() {
    return privateHelper() * 2;
}

function usesPublicHelper() {
    return publicHelper() + 10;
}

export function finalPublicFunction() {
    return usesPublicHelper();
}

// Interleaved exports and non-exports
const privateA = 1;
export const publicA = privateA * 2;
const privateB = publicA + 1;
export const publicB = privateB * 2;
const privateC = publicB + 1;
export const publicC = privateC * 2;

// Class inheritance with mixed visibility
class BaseClass {
    protected value = 10;
}

export class MiddleClass extends BaseClass {
    getValue() {
        return this.value;
    }
}

class ExtendedClass extends MiddleClass {
    doubleValue() {
        return this.getValue() * 2;
    }
}

export class FinalClass extends ExtendedClass {
    tripleValue() {
        return this.doubleValue() + this.getValue();
    }
}