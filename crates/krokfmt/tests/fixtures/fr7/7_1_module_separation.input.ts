// FR7.1: Module-level declaration separation
// Should add empty lines between different declaration types and visibility groups

import { helper } from './utils';
import React from 'react';

// Multiple constants should stay together
export const API_URL = '/api';
export const TIMEOUT = 5000;
const INTERNAL_CONSTANT = 100;

// Functions of same visibility should stay together  
export function fetchData() {
    return fetch(API_URL);
}
export function processData(data: unknown) {
    return transform(data);
}
function internalHelper() {
    return INTERNAL_CONSTANT;
}

// Classes should be separated from functions
export class DataService {
    fetch() { return fetchData(); }
}
class InternalService {
    process() { return 'internal'; }
}

// Interfaces should be separated from classes
export interface Config {
    url: string;
    timeout: number;
}
interface InternalConfig {
    debug: boolean;
}

// Types should be separated from interfaces
export type Status = 'idle' | 'loading' | 'error';
type InternalStatus = 'ready' | 'busy';

// Enums should be separated from types
export enum Color {
    Red = 'red',
    Blue = 'blue'
}
enum InternalColor {
    Green = 'green'
}