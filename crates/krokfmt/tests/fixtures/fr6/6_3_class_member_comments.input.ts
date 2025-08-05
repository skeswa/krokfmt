// FR6.3: Comments on class members should stay with their members after reordering

/**
 * User service class
 */
export class UserService {
    // Instance method
    getUser(id: number) { // Fetches a user by ID
        return this.users.find(u => u.id === id);
    }

    // Private storage
    private users: User[] = []; // In-memory storage

    // Constructor
    constructor() { // Initialize service
        this.loadUsers();
    }

    // Static utility
    static validateId(id: number) { // Check if ID is valid
        return id > 0;
    }

    // Private helper
    private loadUsers() { // Load initial data
        // Implementation here
    }
}