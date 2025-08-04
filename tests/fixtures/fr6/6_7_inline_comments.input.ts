// FR6.7: Inline comment preservation - comments within expressions and parameters

// Variable declarations with inline comments
const x = /* inline comment */ 42;
let y = /* another inline */ "hello";
var z = /* number */ 100 + /* expression */ 50;

// Function parameters with inline comments
function foo(/* first param */ a: number, /* second param */ b: string) {
    return a + b.length;
}

// Arrow functions with inline comments
const arrow = (/* param1 */ x: number, /* param2 */ y: number) => x + y;

// Method with inline comments
class Example {
    method(/* param */ value: string): /* return type */ void {
        console.log(value);
    }
}

// Array with inline comments (not yet supported)
const arr = [/* first */ 1, /* second */ 2, /* third */ 3];

// Object with inline comments (not yet supported)
const obj = {
    key1: /* value1 */ "hello",
    key2: /* value2 */ 42
};

// Complex expressions with inline comments (not yet supported)
const complex = (/* a */ 10 + /* b */ 20) * /* c */ 30;

// Type annotations with inline comments
function typed(param: /* inline type */ string): /* return inline */ number {
    return param.length;
}