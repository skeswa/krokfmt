// FR2: Comprehensive integration test combining all FR2 features

// Internal utilities
function internalLog(message: string) {
    console.log(`[Internal] ${message}`);
}

function internalValidate(value: unknown): boolean {
    return value != null;
}

// Base types used by exports
interface BaseConfig {
    debug: boolean;
}

type LogLevel = 'info' | 'warn' | 'error';

// Exported logger with dependencies
export class Logger {
    constructor(private level: LogLevel) {}
    
    log(message: string) {
        if (this.level === 'info') {
            internalLog(message);
        }
    }
}

export function createLogger(level: LogLevel = 'info'): Logger {
    return new Logger(level);
}

// Exported config depending on base
export interface AppConfig extends BaseConfig {
    logLevel: LogLevel;
    apiUrl: string;
}

export const defaultConfig: AppConfig = {
    debug: false,
    logLevel: 'info',
    apiUrl: 'https://api.example.com'
};

// Mixed visibility service
class BaseService {
    protected logger: Logger;
    
    constructor(config: AppConfig) {
        this.logger = createLogger(config.logLevel);
    }
}

export class DataService extends BaseService {
    async fetchData(id: string): Promise<unknown> {
        if (!internalValidate(id)) {
            throw new Error('Invalid ID');
        }
        this.logger.log(`Fetching data for ${id}`);
        // Fetch implementation
        return { id };
    }
}

export function createDataService(config: AppConfig = defaultConfig): DataService {
    return new DataService(config);
}

// Type guards and validators
export function isAppConfig(value: unknown): value is AppConfig {
    return (
        typeof value === 'object' &&
        value !== null &&
        'debug' in value &&
        'logLevel' in value &&
        'apiUrl' in value
    );
}

// Final integration
const serviceInstance = createDataService();

export async function performDataOperation(id: string) {
    return serviceInstance.fetchData(id);
}