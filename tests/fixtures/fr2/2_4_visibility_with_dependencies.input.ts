// FR2.4: Visibility grouping with dependencies - dependencies must be hoisted

// Internal config used by exported class
const DATABASE_CONFIG = {
    host: 'localhost',
    port: 5432
};

// Exported class that depends on internal config
export class DatabaseConnection {
    private config = DATABASE_CONFIG;
    
    connect() {
        return `Connecting to ${this.config.host}:${this.config.port}`;
    }
}

// Internal helper used by exported function
function validateInput(input: unknown): boolean {
    return input !== null && input !== undefined;
}

// Another internal helper used by exported function
function formatOutput(data: string): string {
    return data.toUpperCase();
}

// Exported function that depends on internal helpers
export function processData(input: unknown): string {
    if (!validateInput(input)) {
        throw new Error('Invalid input');
    }
    return formatOutput(String(input));
}

// Internal base class
class BaseService {
    protected name = 'base';
}

// Exported class extending internal base
export class ApiService extends BaseService {
    getName() {
        return this.name;
    }
}

// Internal type used by exported interface
type UserId = string;

// Exported interface using internal type
export interface User {
    id: UserId;
    name: string;
}

// Standalone internal function (no dependencies)
function standaloneHelper() {
    return 'standalone';
}

// Standalone export (no dependencies)
export function standaloneExport() {
    return 'exported standalone';
}