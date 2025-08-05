// Test file for function argument destructuring sorting

// Regular function with object destructuring
function processUser({ username, email, id, avatar }: UserData) {
    console.log(id, username, email, avatar);
}

// Arrow function with object destructuring  
const updateProfile = ({ bio, location, website, twitter }: ProfileUpdate) => {
    return { bio, location, twitter, website };
};

// Function with mixed parameters - only destructured object should be sorted
function createPost(userId: number, { title, content, tags, draft }: PostData, callback: Function) {
    callback(userId, { content, draft, tags, title });
}

// Nested destructuring
function handleConfig({ server: { port, host, ssl }, database: { name, user, password } }: Config) {
    console.log(`Server: ${host}:${port}, SSL: ${ssl}`);
    console.log(`DB: ${name} (user: ${user})`);
}

// With default values and renamed properties
function parseOptions({ verbose = false, quiet = true, output: outputFile }: Options) {
    console.log(verbose, quiet, outputFile);
}

// Rest parameters should sort to the end
function mergeSettings({ theme, language, ...otherSettings }: Settings) {
    return { language, theme, ...otherSettings };
}