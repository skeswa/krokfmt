// FR1.7: Re-exports with comments

// External imports
import React from 'react';
import axios from 'axios';

// UI re-exports from our component library
export { Button, TextField } from 'ui-library';
// Re-export React utilities
export { Fragment, StrictMode } from 'react';

// Absolute path re-exports
export { theme } from '@styles/theme';
export * as icons from '@assets/icons';

// Local re-exports
// Common utilities used across the app
export * from './common';
// Specific named exports
export { formatDate, parseDate } from './date-utils';

// Regular module content
export interface Config {
    apiUrl: string;
    timeout: number;
}

const defaultConfig: Config = {
    apiUrl: '/api',
    timeout: 5000
};