// FR3.3: Test TypeScript visibility keywords (private, protected, public)

class TypeScriptVisibility {
    // TypeScript visibility keywords mixed with # private syntax
    
    // TypeScript private keyword (treated as public for sorting since it's a runtime construct)
    private tsPrivateField = 'ts private';
    
    // Protected fields (treated as public for sorting)
    protected protectedField = 'protected';
    
    // Explicit public keyword
    public publicField = 'public';
    
    // Implicit public (no keyword)
    implicitPublic = 'implicit';
    
    // True private with # syntax
    #truePrivate = 'true private';
    
    // Static with TypeScript keywords
    private static staticTsPrivate = 'static ts private';
    protected static staticProtected = 'static protected';
    public static staticPublic = 'static public';
    
    // True private static
    static #staticTruePrivate = 'static true private';
    
    // Methods with TypeScript keywords
    private tsPrivateMethod() { return 'ts private method'; }
    protected protectedMethod() { return 'protected method'; }
    public publicMethod() { return 'public method'; }
    implicitPublicMethod() { return 'implicit public'; }
    
    // True private methods
    #truePrivateMethod() { return 'true private method'; }
    
    // Constructor with parameter properties
    constructor(
        private ctorPrivate: string,
        protected ctorProtected: string,
        public ctorPublic: string,
        readonly ctorReadonly: string
    ) {}
    
    // Static methods with keywords
    private static staticTsPrivateMethod() { return 'static ts private'; }
    protected static staticProtectedMethod() { return 'static protected'; }
    public static staticPublicMethod() { return 'static public'; }
    
    // True private static method
    static #staticTruePrivateMethod() { return 'static true private'; }
    
    // Readonly modifier (treated as public)
    readonly readonlyField = 'readonly';
    static readonly staticReadonly = 'static readonly';
}