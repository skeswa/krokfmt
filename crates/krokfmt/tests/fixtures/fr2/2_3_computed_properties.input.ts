// FR2.3: Computed property dependencies

// Base values for computed properties
const PREFIX = 'api';
const VERSION = 'v1';

// Object with computed properties
const endpoints = {
    [`${PREFIX}_${VERSION}_users`]: '/users',
    [`${PREFIX}_${VERSION}_posts`]: '/posts'
};

export const apiEndpoints = endpoints;

// Computed method names
const methodName = 'getData';

class DataService {
    [methodName]() {
        return 'data';
    }
}

export const service = new DataService();

// Symbol-based properties
const privateKey = Symbol('private');

const obj = {
    [privateKey]: 'secret',
    public: 'visible'
};

export function getPrivate() {
    return obj[privateKey];
}