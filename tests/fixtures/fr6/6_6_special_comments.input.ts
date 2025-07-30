// FR6.6: Special comment handling - TypeScript pragmas and directives

// @ts-nocheck

import { helper } from './helper';
import React from 'react';

// #region Types

interface User {
    name: string;
    // @ts-ignore
    legacyField: any;
}

// #endregion

// eslint-disable-next-line no-unused-vars
const unused = 42;

function riskyFunction() {
    // @ts-expect-error
    return nonExistentVariable;
}

// prettier-ignore
const uglyFormatted = {z: 1,
                      a: 2,b:3};

// eslint-disable no-console
console.log('test');
// eslint-enable no-console

// #region Utils

// @ts-ignore: Deprecated but still needed
function oldFunction() {
    // prettier-ignore
    return    42    +    10;
}

// #endregion

/* eslint-disable */
const x = 1;
const y = 2;
/* eslint-enable */

// TODO: Fix this later
// FIXME: This is broken
// HACK: Temporary workaround
// NOTE: Important information