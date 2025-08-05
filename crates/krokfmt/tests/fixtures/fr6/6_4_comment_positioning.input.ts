// FR6.4: Comment positioning - maintain correct positions relative to code

// Header comment at top of file
// Should stay at top even with reordering

import { z } from './z';
// Leading comment for y import
import { y } from './y';
import { x } from './x'; // Trailing comment on x

// Floating comment 1

// Leading comment for const a
const a = 1; // Trailing on a

// Floating comment 2

const b = 2;

// Floating comment 3

// This comment belongs to function foo
// It has multiple lines
function foo() {
    // Inside foo
}

// Gap before bar

// This belongs to bar
function bar() {} // Inline with bar

// Multiple floating comments
// in a row
// should be preserved

/*
 * Block floating comment
 */

// Leading for export
export const c = 3;

// Final floating comment