// FR3.1: Function arguments should be sorted when destructured

// Object destructuring parameters should be sorted
function processOptions({ zebra, apple, banana }: Options) {
    return { apple, banana, zebra };
}

// Arrow function with destructured params
const handleConfig = ({ timeout, retries, baseURL, headers }: Config) => {
    console.log(baseURL, headers, retries, timeout);
};

// Nested destructuring
function processUser({ user: { name, age, id }, settings: { theme, language } }: UserData) {
    return { age, id, language, name, theme };
}

// Mixed parameters - only destructured should be sorted
function mixedParams(first: number, { zebra, apple }: Options, last: string) {
    return `${first} ${apple} ${zebra} ${last}`;
}

// Positional parameters should NOT be sorted
function add(b: number, a: number) {
    return a + b;
}