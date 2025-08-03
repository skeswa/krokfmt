// FR6.1: Comments on imports should stay with their imports after sorting

// External dependencies
import React from 'react'; // UI library
import axios from 'axios'; // HTTP client

// Absolute imports
import { Button } from '@ui/components'; // Reusable button
import { api } from '@services/api';

// Relative imports
import { helper } from '../utils/helper'; // Utility functions
import { config } from './config'; // Local configuration