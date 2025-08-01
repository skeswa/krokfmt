// FR3.3: Test complex class with mixed visibility patterns

class MixedVisibilityClass {
    // Mix everything up to ensure proper sorting
    
    // Some public methods
    zPublicMethod() { return 'z'; }
    
    // Private static field
    static #config = { api: 'https://api.example.com' };
    
    // Public field
    name: string;
    
    // Private method
    #validateName(name: string): boolean {
        return name.length > 0;
    }
    
    // Static public field
    static VERSION = '1.0.0';
    
    // Another private field
    #id: number;
    
    // Public static method
    static getInstance() {
        return new MixedVisibilityClass('default');
    }
    
    // Constructor
    constructor(name: string) {
        this.name = name;
        this.#id = Math.random();
    }
    
    // Another public field
    active = true;
    
    // Private static method
    static #generateId() {
        return Math.random().toString(36);
    }
    
    // More public methods
    aPublicMethod() { return 'a'; }
    bPublicMethod() { return 'b'; }
    
    // Another private method
    #processData(data: any) {
        return data;
    }
    
    // Static fields mixed in
    static API_KEY = 'abc123';
    static #SECRET_KEY = 'secret';
    
    // Getters and setters (treated as methods)
    get id() { return this.#id; }
    set id(value: number) { this.#id = value; }
    
    // Private getter/setter
    #privateValue = 100;
    get #value() { return this.#privateValue; }
    set #value(v: number) { this.#privateValue = v; }
}