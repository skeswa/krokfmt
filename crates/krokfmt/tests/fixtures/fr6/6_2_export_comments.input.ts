// FR6.2: Comments on exports should stay with their exports after reordering

// Non-exported helper
function internalHelper() {
    return 'internal';
}

// Main public API
export function publicApi() { // This is the main entry point
    return 'public';
}

// Another helper
const helper2 = () => 'helper2';

// Secondary export
export const secondaryApi = () => { // Less important API
    return 'secondary';
};