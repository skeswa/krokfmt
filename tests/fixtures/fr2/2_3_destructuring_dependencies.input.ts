// FR2.3: Destructuring and spread dependencies

// Object with dependencies
const baseConfig = {
    host: 'localhost',
    port: 3000
};

const extendedConfig = {
    ...baseConfig,
    ssl: true
};

export const { host, port } = extendedConfig;

// Array destructuring
const values = [1, 2, 3];
const [first, second] = values;
export const sum = first + second;

// Function parameter destructuring
interface Options {
    timeout: number;
    retries: number;
}

const defaultOptions: Options = {
    timeout: 5000,
    retries: 3
};

export function createClient({ timeout, retries }: Options = defaultOptions) {
    return { timeout, retries };
}

// Nested destructuring
const nested = {
    user: {
        profile: {
            name: 'test'
        }
    }
};

const { user: { profile: { name } } } = nested;
export const username = name;