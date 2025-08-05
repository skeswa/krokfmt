// FR6.7: Standalone Comment Preservation

// Test 1: Class with overall descriptive comment
class DocumentProcessor {
    // This class handles document processing workflow
    // including validation, transformation, and storage
    
    #privateConfig = { maxSize: 1024 };
    
    processDocument(doc: any) {
        return doc;
    }
    
    static VERSION = '1.0.0';
    
    validateDocument(doc: any) {
        return true;
    }
}

// Test 2: Function with implementation notes
function complexAlgorithm(data: number[]) {
    // This uses a two-pass approach:
    // 1. First pass calculates statistics
    // 2. Second pass applies transformations
    
    const stats = calculateStats(data);
    const sorted = data.sort();
    return applyTransforms(sorted, stats);
}

// Test 3: Class with mixed comment types
class ServiceManager {
    // Overall service management logic
    // Handles lifecycle and dependency injection
    
    // Static configuration
    static #config = { timeout: 5000 };
    
    // Instance fields
    #services = new Map();
    name: string;
    
    // Constructor initializes the manager
    constructor(name: string) {
        this.name = name;
    }
    
    // Public API
    registerService(id: string, service: any) {
        this.#services.set(id, service);
    }
    
    getService(id: string) {
        return this.#services.get(id);
    }
}

// Test 4: Nested structures with class-level comments
class OuterClass {
    // This outer class contains nested functionality
    
    method() {
        // Method-level comment about implementation
        
        const helper = () => {
            // Arrow function with its own context
            return 42;
        };
        
        return helper();
    }
    
    #privateMethod() {
        // Private method implementation notes
        return 'private';
    }
}

// Test 5: Interface with documentation
interface APIClient {
    // Interface defining the contract for API clients
    // All implementations must support these methods
    
    get(url: string): Promise<any>;
    post(url: string, data: any): Promise<any>;
    delete(url: string): Promise<void>;
}

// Test 6: Complex class that triggered the issue
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
}

// Test 7: Function with section comments
function complexDataPipeline(rawData: any[]) {
    // Input validation
    const errors = [];
    for (const item of rawData) {
        if (!isValid(item)) errors.push(item);
    }
    
    // Data transformation phase
    // We need to normalize before processing
    
    const normalized = rawData.map(normalize);
    const filtered = normalized.filter(x => x != null);
    
    // Performance optimization:
    // Batch process in chunks of 100
    
    const chunks = [];
    for (let i = 0; i < filtered.length; i += 100) {
        chunks.push(filtered.slice(i, i + 100));
    }
    
    // Final aggregation
    
    return chunks.map(processChunk).flat();
}

// Test 8: Comments within function preserve their position
function dataProcessor(items: any[]) {
    // Initialize results container
    const results = [];
    
    // === VALIDATION PHASE ===
    // Check each item for required fields
    
    for (const item of items) {
        if (!item.id || !item.name) {
            continue;
        }
        results.push(item);
    }
    
    // === TRANSFORMATION PHASE ===
    // Apply business logic transformations
    
    const transformed = results.map(item => {
        // Normalize the data structure
        return {
            ...item,
            normalized: true,
            timestamp: Date.now()
        };
    });
    
    // === FINAL CLEANUP ===
    
    return transformed.filter(Boolean);
}

// Test 9: Complex function with multiple section comments
function analyzeCodebase(files: string[]) {
    const results = {
        totalFiles: files.length,
        errors: [],
        warnings: [],
        stats: {}
    };
    
    // Phase 1: Parse all files
    // This can take a while for large codebases
    
    const parsed = files.map(file => {
        try {
            return parseFile(file);
        } catch (e) {
            results.errors.push({ file, error: e });
            return null;
        }
    }).filter(Boolean);
    
    // Phase 2: Build dependency graph
    // Note: We're using a simplified algorithm here
    // For production, consider using a more robust solution
    
    const depGraph = buildDependencyGraph(parsed);
    const cycles = detectCycles(depGraph);
    
    if (cycles.length > 0) {
        // Circular dependencies detected!
        // This is usually a code smell
        
        results.warnings.push(...cycles.map(c => ({
            type: 'circular-dependency',
            files: c
        })));
    }
    
    // Phase 3: Calculate metrics
    
    results.stats = {
        avgFileSize: calculateAvgSize(parsed),
        complexity: calculateComplexity(parsed),
        coverage: estimateCoverage(parsed)
    };
    
    // Cleanup temporary data
    // Important: This prevents memory leaks
    
    parsed.forEach(p => p.cleanup?.());
    
    return results;
}

// Test 10: Method with contextual comments
class DataService {
    // Service configuration
    
    #apiUrl = 'https://api.example.com';
    #timeout = 5000;
    
    async fetchData(id: string) {
        // Validate input first
        if (!id) {
            throw new Error('ID required');
        }
        
        // Prepare the request
        // Note: We use a custom header for tracking
        
        const headers = {
            'X-Request-ID': generateId(),
            'Content-Type': 'application/json'
        };
        
        try {
            // Make the actual request
            const response = await fetch(`${this.#apiUrl}/data/${id}`, {
                headers,
                timeout: this.#timeout
            });
            
            // Process the response
            // Important: Check status before parsing
            
            if (!response.ok) {
                throw new Error(`HTTP ${response.status}`);
            }
            
            return await response.json();
        } catch (error) {
            // Log and re-throw
            // This helps with debugging production issues
            
            console.error('Fetch failed:', error);
            throw error;
        }
    }
}

// Helper functions for tests
function calculateStats(data: number[]) {
    return { mean: 0, median: 0 };
}

function applyTransforms(data: number[], stats: any) {
    return data;
}

// Helper functions for the examples
function generateId(): string {
    return Math.random().toString(36).substring(7);
}

function isValid(item: any): boolean {
    return item != null && item.id;
}

function normalize(item: any): any {
    return { ...item, normalized: true };
}

function processChunk(chunk: any[]): any[] {
    return chunk.map(x => ({ ...x, processed: true }));
}

// Stub functions for the examples
function parseFile(file: string): any {
    return { file, ast: {} };
}

function buildDependencyGraph(parsed: any[]): any {
    return {};
}

function detectCycles(graph: any): any[] {
    return [];
}

function calculateAvgSize(parsed: any[]): number {
    return 0;
}

function calculateComplexity(parsed: any[]): number {
    return 0;
}

function estimateCoverage(parsed: any[]): number {
    return 0;
}