// FR3.3: Test private field syntax with # prefix

class PrivateFieldsClass {
    // Mix of private and public members to test proper categorization
    
    // Private fields with initializers
    #zebra = 'private zebra';
    #apple = 42;
    #banana = { type: 'fruit' };
    
    // Public fields
    public cherry = 'public cherry';
    durian = 'durian';
    elderberry: string;
    
    // Private static fields
    static #staticZebra = 'static private zebra';
    static #staticApple = 100;
    
    // Public static fields
    static staticBanana = 'static public banana';
    static staticCherry = 'static public cherry';
    
    // Private methods
    #zeroMethod() {
        return this.#zebra;
    }
    
    #alphaMethod() {
        return this.#apple;
    }
    
    // Public methods
    betaMethod() {
        return this.#banana;
    }
    
    alphaMethod() {
        return this.durian;
    }
    
    // Private static methods
    static #staticZeroMethod() {
        return this.#staticZebra;
    }
    
    static #staticAlphaMethod() {
        return this.#staticApple;
    }
    
    // Public static methods
    static staticBetaMethod() {
        return this.staticBanana;
    }
    
    static staticAlphaMethod() {
        return this.staticCherry;
    }
    
    constructor() {
        this.elderberry = 'initialized';
    }
}