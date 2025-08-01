// FR3.3: Test all 9 visibility groups in proper order

class CompleteClass {
    // All members intentionally out of order to test sorting
    
    // Private instance methods (should be 9th)
    #zPrivateMethod() { return 'z'; }
    #aPrivateMethod() { return 'a'; }
    
    // Public instance fields (should be 5th)
    zField: string = 'z';
    aField: string = 'a';
    
    // Private static methods (should be 4th)
    static #zPrivateStatic() { return 'z'; }
    static #aPrivateStatic() { return 'a'; }
    
    // Public static fields (should be 1st)
    static zStatic = 'z';
    static aStatic = 'a';
    
    // Constructor (should be 7th)
    constructor() {
        this.aField = 'a';
        this.zField = 'z';
    }
    
    // Private instance fields (should be 6th)
    #zPrivateField = 'z';
    #aPrivateField = 'a';
    
    // Public static methods (should be 3rd)
    static zStaticMethod() { return 'z'; }
    static aStaticMethod() { return 'a'; }
    
    // Private static fields (should be 2nd)
    static #zPrivateStaticField = 'z';
    static #aPrivateStaticField = 'a';
    
    // Public instance methods (should be 8th)
    zMethod() { return this.zField; }
    aMethod() { return this.aField; }
}