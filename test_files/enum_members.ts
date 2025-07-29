// Test file for enum member sorting

// String enums should be sorted alphabetically
enum Status {
    Pending = "pending",
    Active = "active", 
    Disabled = "disabled",
    Archived = "archived",
    Suspended = "suspended"
}

enum LogLevel {
    Error = "ERROR",
    Warning = "WARNING",
    Info = "INFO", 
    Debug = "DEBUG",
    Trace = "TRACE"
}

// Numeric enums should preserve original order
enum Priority {
    Low = 1,
    Medium = 2,
    High = 3,
    Critical = 4
}

enum HttpStatus {
    NotFound = 404,
    OK = 200,
    ServerError = 500,
    BadRequest = 400,
    Unauthorized = 401
}

// Mixed enums should not be sorted
enum Mixed {
    First,
    Second = 10,
    Third,
    Fourth = "fourth",
    Fifth = "fifth"
}

// Implicit numeric enums should not be sorted
enum Direction {
    Up,
    Down,
    Left,
    Right
}

// Complex expressions should not be sorted
enum Flags {
    None = 0,
    Read = 1 << 0,
    Write = 1 << 1,
    Execute = 1 << 2,
    All = Read | Write | Execute
}

// Const enum with strings should be sorted
const enum Environment {
    Production = "production",
    Development = "development",
    Staging = "staging",
    Testing = "testing"
}