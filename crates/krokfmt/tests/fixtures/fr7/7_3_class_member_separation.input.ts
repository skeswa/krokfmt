// FR7.3: Class member group separation
// Should add empty lines between different member visibility groups

class CompleteExample {
    // Members are intentionally mixed to test grouping and separation
    
    // Private instance methods
    #privateMethod() { return 'private'; }
    
    // Public static fields
    static defaultTimeout = 5000;
    static version = '1.0.0';
    
    // Private instance fields
    #authToken?: string;
    #retryCount = 0;
    
    // Public static methods
    static getInstance() {
        return new CompleteExample();
    }
    static configure(options: any) {
        // configure
    }
    
    // Public instance fields
    baseUrl: string;
    timeout: number;
    
    // Private static fields
    static #instance: CompleteExample;
    static #secretKey = 'secret';
    
    // Constructor
    constructor(baseUrl = '/api') {
        this.baseUrl = baseUrl;
        this.timeout = CompleteExample.defaultTimeout;
    }
    
    // Private static methods
    static #validateKey(key: string) {
        return key === this.#secretKey;
    }
    
    // Public instance methods  
    async get(endpoint: string) {
        return this.#request('GET', endpoint);
    }
    post(endpoint: string, data: any) {
        return this.#request('POST', endpoint, data);
    }
    
    // Another private instance method
    async #request(method: string, endpoint: string, data?: any) {
        this.#retryCount++;
        return { method, endpoint, data };
    }
}