// FR1.7: Re-export organization and categorization

// Mixed imports and re-exports that need to be separated and organized
import { useState } from 'react';
export { Fragment } from 'react';
import { debounce } from 'lodash';
export * from 'react-dom';
import { Button } from '@components/Button';
export { IconButton } from '@components/IconButton';
import { config } from '~/config';
export * as theme from '@theme/default';
import { helper } from './utils';
export { utilityA, utilityB } from './utilities';
import { api } from '../api';
export * from './common';

// Regular exports and declarations
export const APP_NAME = 'MyApp';

const internalConstant = 42;

export function processData(data: unknown) {
    return data;
}

function internalHelper() {
    return 'helper';
}