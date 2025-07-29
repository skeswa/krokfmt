// Test file for FR1 requirements
// This file tests all import syntaxes and scenarios

// Some orphaned code that should move below imports
const orphanedVar = "I should be after imports";

// Side-effect imports (FR1.1, FR1.6)
import './polyfills';
import 'reflect-metadata';

// Mixed imports from relative paths (FR1.1)
import utils, { helper, type HelperType } from '../utils/helpers';

// Namespace imports (FR1.1)
import * as fs from 'fs';
import * as customLib from './lib/custom';

// Type-only imports (FR1.1, FR1.6)
import type { User } from './types/user';
import type { Config } from '@config/types';

// Import aliases (FR1.1, FR1.6)
import { foo as bar, baz as qux } from './module';
import { Component as MyComponent } from '@ui/components';

// External imports out of order (FR1.2, FR1.3)
import zod from 'zod';
import axios from 'axios';
import React, { useState, useEffect } from 'react';
import lodash from 'lodash/debounce';

// Absolute imports out of order (FR1.2, FR1.3)
import { z } from '@utils/validation';
import { a } from '@utils/array';
import { Button } from '~/components/Button';
import { api } from '@services/api';

// Relative imports with various depths (FR1.2, FR1.3)
import { deepHelper } from '../../helpers/deep';
import { siblingUtil } from './utils/sibling';
import { parentHelper } from '../helpers/parent';

// Mixed import/export
export { default as MyExport } from './my-export';
export * from './re-exports';

// Regular code
export const main = () => {
    console.log('Main function');
    return bar() + qux();
};

// This comment should stay with the function below
export function processUser(user: User): void {
    console.log(user);
}