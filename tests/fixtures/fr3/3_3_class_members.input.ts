// FR3.3: Class members should be sorted within visibility groups

class MyClass {
    // Public static fields (should be first, alphabetically)
    static zoo = 'zoo';
    static bar = 'bar';
    static foo = 'foo';
    
    // Private static fields (should be second, alphabetically)
    static #privateZoo = 'pzoo';
    static #privateBar = 'pbar';
    static #privateFoo = 'pfoo';
    
    // Public instance fields (should be fifth, alphabetically)
    zebra: string;
    apple: number;
    banana: boolean;
    
    // Private instance fields (should be sixth, alphabetically)
    #privateZebra = 'pz';
    #privateApple = 1;
    #privateBanana = true;
    
    // Constructor should be seventh
    constructor() {
        this.apple = 1;
        this.banana = true;
        this.zebra = 'z';
    }
    
    // Public static methods (should be third, alphabetically)
    static zStatic() { return 'z'; }
    static aStatic() { return 'a'; }
    static bStatic() { return 'b'; }
    
    // Private static methods (should be fourth, alphabetically)
    static #privateZStatic() { return 'pz'; }
    static #privateAStatic() { return 'pa'; }
    static #privateBStatic() { return 'pb'; }
    
    // Public instance methods (should be eighth, alphabetically)
    zMethod() { return this.zebra; }
    aMethod() { return this.apple; }
    bMethod() { return this.banana; }
    
    // Private instance methods (should be ninth, alphabetically)
    #privateZMethod() { return this.#privateZebra; }
    #privateAMethod() { return this.#privateApple; }
    #privateBMethod() { return this.#privateBanana; }
}