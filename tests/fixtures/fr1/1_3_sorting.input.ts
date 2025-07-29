// FR1.3: Imports should be sorted alphabetically within each category
// External imports out of order
import zod from 'zod';
import axios from 'axios';
import react from 'react';

// Absolute imports out of order
import { z } from '@utils/z';
import { a } from '@utils/a';
import { m } from '@utils/m';

// Relative imports out of order
import { zebra } from './zebra';
import { apple } from '../apple';
import { monkey } from '../../monkey';

export function main() {
    console.log('Sorted imports');
}