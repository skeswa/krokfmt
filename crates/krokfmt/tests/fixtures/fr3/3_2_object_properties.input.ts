// FR3.2: Object literal properties should be sorted alphabetically

const config = {
    zebra: true,
    apple: 42,
    banana: "yellow",
    cat: null,
    dog: undefined
};

// Nested objects should also be sorted
const nested = {
    outer: {
        zebra: 1,
        apple: 2,
        middle: {
            zoo: 'a',
            bar: 'b',
            foo: 'c'
        }
    },
    another: {
        last: true,
        first: false
    }
};

// Spread operators should be preserved
const withSpread = {
    zebra: 1,
    ...defaults,
    apple: 2,
    banana: 3,
    ...overrides,
    cat: 4
};

// Computed properties
const computed = {
    zebra: 1,
    [dynamicKey]: 2,
    apple: 3,
    ['literal']: 4,
    banana: 5
};