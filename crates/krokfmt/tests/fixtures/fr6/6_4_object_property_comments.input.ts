// FR6.4: Comments on object properties should stay with properties after sorting

const config = {
    // Server settings
    port: 3000, // Default port
    host: 'localhost', // Local development
    
    // Database config
    database: {
        name: 'myapp', // Database name
        host: 'db.local', // Database server
        port: 5432 // PostgreSQL default
    },
    
    // Feature flags
    enableLogging: true, // Enable debug logs
    enableCache: false // Caching disabled in dev
};