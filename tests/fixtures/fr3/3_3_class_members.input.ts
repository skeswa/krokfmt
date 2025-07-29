// FR3.3: Class members should be sorted within their groups

class MyClass {
    // Instance fields out of order
    zebra: string;
    apple: number;
    banana: boolean;
    
    // Static fields out of order
    static zoo = 'zoo';
    static bar = 'bar';
    static foo = 'foo';
    
    // Constructor should remain in place
    constructor() {
        this.apple = 1;
        this.banana = true;
        this.zebra = 'z';
    }
    
    // Instance methods out of order
    zMethod() { return this.zebra; }
    aMethod() { return this.apple; }
    bMethod() { return this.banana; }
    
    // Static methods out of order
    static zStatic() { return 'z'; }
    static aStatic() { return 'a'; }
    static bStatic() { return 'b'; }
}