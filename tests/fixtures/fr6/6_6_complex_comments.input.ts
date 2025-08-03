// FR6.6: Complex comment scenarios with multiple types

/**
 * Main application configuration
 * 
 * This file contains all the configuration needed
 * for the application to run properly.
 */

// Import section
import { z } from 'zod'; // Schema validation
import type { Config } from './types'; // Type definitions

/* Configuration schema */
const schema = z.object({
    // Required fields
    apiUrl: z.string(), // API endpoint
    apiKey: z.string(), // Secret key
    
    // Optional settings
    timeout: z.number().optional(), // Request timeout
    retries: z.number().optional() // Retry count
});

/**
 * Validates configuration
 * @param config - The config object to validate
 * @returns Validated configuration
 */
export function validateConfig(config: unknown): Config {
    // Parse and validate
    return schema.parse(config); // Throws on error
}

// Default configuration
export const defaultConfig = {
    apiUrl: 'https://api.example.com', // Production API
    apiKey: process.env.API_KEY || '', // From environment
    timeout: 5000, // 5 seconds
    retries: 3 // Three attempts
}; // End of default config