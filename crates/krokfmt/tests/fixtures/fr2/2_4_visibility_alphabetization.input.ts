// FR2.4: Test alphabetization within visibility groups

// Exports in random order
export function zebra() { return 'z'; }
export class Monkey { name = 'm'; }
export interface Apple { type: string; }
export type Banana = string;
export const ELEPHANT = 'e';
export enum Dog { WOOF }
export function aardvark() { return 'a'; }
export class Cat { meow = true; }

// Non-exports in random order
function walrus() { return 'w'; }
class Tiger { roar = true; }
interface Giraffe { height: number; }
type Lion = 'king';
const HIPPO = 'h';
enum Fox { RED }
function bear() { return 'b'; }
class Rabbit { hop = true; }