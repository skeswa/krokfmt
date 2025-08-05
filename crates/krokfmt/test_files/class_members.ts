// Test file for class member sorting

class UserModel {
    // Mixed order instance fields
    private lastName: string;
    public firstName: string;
    protected email: string;
    readonly id: number;
    
    // Mixed order static fields
    static VERSION = "1.0.0";
    static readonly API_ENDPOINT = "/api/users";
    private static cache = new Map();
    
    // Constructor should stay in the middle
    constructor(data: any) {
        this.id = data.id;
        this.firstName = data.firstName;
        this.lastName = data.lastName;
        this.email = data.email;
    }
    
    // Mixed order instance methods
    private validateEmail(): boolean {
        return this.email.includes('@');
    }
    
    public getFullName(): string {
        return `${this.firstName} ${this.lastName}`;
    }
    
    protected updateEmail(newEmail: string): void {
        if (this.validateEmail()) {
            this.email = newEmail;
        }
    }
    
    // Mixed order static methods
    static fromJSON(json: string): UserModel {
        return new UserModel(JSON.parse(json));
    }
    
    private static clearCache(): void {
        this.cache.clear();
    }
    
    static async fetchById(id: number): Promise<UserModel> {
        const response = await fetch(`${this.API_ENDPOINT}/${id}`);
        const data = await response.json();
        return new UserModel(data);
    }
}

// Test with decorators and getters/setters
class ConfigManager {
    @observable private _theme: string = "light";
    @observable public language: string = "en";
    
    static instance: ConfigManager;
    static readonly SUPPORTED_THEMES = ["light", "dark"];
    
    get theme(): string {
        return this._theme;
    }
    
    set theme(value: string) {
        if (ConfigManager.SUPPORTED_THEMES.includes(value)) {
            this._theme = value;
        }
    }
    
    constructor() {
        if (ConfigManager.instance) {
            return ConfigManager.instance;
        }
        ConfigManager.instance = this;
    }
    
    resetToDefaults(): void {
        this._theme = "light";
        this.language = "en";
    }
    
    static getInstance(): ConfigManager {
        if (!this.instance) {
            this.instance = new ConfigManager();
        }
        return this.instance;
    }
}