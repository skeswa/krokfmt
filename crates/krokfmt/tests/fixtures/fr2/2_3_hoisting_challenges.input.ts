// FR2.3: Hoisting and temporal dead zone challenges

// Function hoisting - exported function uses hoisted function
export function publicApi() {
    return helperFunction();
}

function helperFunction() {
    return 'helper';
}

// Class hoisting (not hoisted, should fail if reordered incorrectly)
export const instance = new MyClass();

class MyClass {
    value = 42;
}

// Const temporal dead zone
export const earlyUse = computeValue();

const base = 10;
function computeValue() {
    return base * 2;
}

// Mixed declarations
var hoistedVar = initialValue();
export const exportedConst = hoistedVar;

function initialValue() {
    return 100;
}