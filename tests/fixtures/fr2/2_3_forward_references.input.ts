// FR2.3: Forward references that are allowed in TypeScript
// These declarations can be reordered without breaking functionality

// Function declarations can be used before declaration (hoisting)
export const result1 = helperFunction1();
function helperFunction1() {
    return 'helper1';
}

// Another function using forward reference
export function mainFunction() {
    return helperFunction2();
}
function helperFunction2() {
    return 'helper2';
}

// Interfaces can extend interfaces declared later
export interface ExtendedUser extends BaseUser {
    premium: boolean;
}
interface BaseUser {
    id: string;
    name: string;
}

// Type aliases can reference types declared later
export type Status = ActiveStatus | InactiveStatus;
type ActiveStatus = 'active';
type InactiveStatus = 'inactive';

// Another type alias example
export type Result = Success | Failure;
type Success = { kind: 'success'; value: string };
type Failure = { kind: 'error'; error: Error };

// Class declarations in type positions
export function createMyClass(): MyClass {
    return new MyClass();
}
class MyClass {
    value = 42;
}

// Enum in type position
export let currentStatus: Status;
enum Status {
    Pending,
    Active,
    Completed
}

// Complex interface inheritance chain
export interface C extends B {
    c: string;
}
interface B extends A {
    b: string;
}
interface A {
    a: string;
}

// Mixed forward references
export type Handler = (data: Data) => Result;
interface Data {
    payload: string;
}
enum Result {
    Success,
    Failure
}

// Function calling another function declared later
export function processData(input: string): string {
    return transform(validate(input));
}
function validate(input: string): string {
    return input.trim();
}
function transform(input: string): string {
    return input.toUpperCase();
}