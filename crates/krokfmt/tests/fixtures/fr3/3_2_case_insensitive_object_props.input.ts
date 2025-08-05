// FR3.2: Test case-insensitive object property sorting

const config = {
    zebra: 'last',
    Apple: 'fruit',
    banana: 'yellow',
    CAR: 'vehicle',
    dog: 'animal',
    ELEPHANT: 'big',
    fish: 'swim'
};

const user = {
    Username: 'john',
    email: 'john@example.com',
    AGE: 25,
    isActive: true,
    Role: 'admin',
    lastLogin: new Date(),
    DEPARTMENT: 'IT'
};

// Nested object with mixed case keys
const settings = {
    Theme: {
        backgroundColor: 'white',
        FontSize: 14,
        COLOR_SCHEME: 'light',
        margin: 10
    },
    api: {
        URL: 'http://api.example.com',
        timeout: 5000,
        RetryCount: 3,
        headers: {}
    }
};