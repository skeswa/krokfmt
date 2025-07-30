// FR2.4: Basic visibility grouping - exports at top, non-exports at bottom, alphabetized within groups

// Mixed order of exports and non-exports
function internalHelper() {
    return 'helper';
}

export function publicApi() {
    return 'api';
}

class PrivateClass {
    value = 42;
}

export class PublicClass {
    name = 'public';
}

const INTERNAL_CONSTANT = 100;

export const PUBLIC_CONSTANT = 200;

interface InternalInterface {
    id: number;
}

export interface ExternalInterface {
    name: string;
}

type PrivateType = string | number;

export type PublicType = 'A' | 'B' | 'C';

enum InternalEnum {
    ONE,
    TWO
}

export enum PublicEnum {
    ALPHA,
    BETA
}

// Another internal function
function anotherHelper() {
    return 'another';
}

// Another export
export function anotherPublicApi() {
    return 'another api';
}