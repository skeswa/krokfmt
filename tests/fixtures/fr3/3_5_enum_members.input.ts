// FR3.5: Enum members should be sorted (string enums only)

// String enum - should be sorted
enum Status {
    Error = 'error',
    Success = 'success',
    Pending = 'pending',
    Loading = 'loading'
}

// Numeric enum - should NOT be sorted (preserve values)
enum Priority {
    Low = 1,
    High = 3,
    Medium = 2,
    Critical = 4
}

// Mixed enum - should NOT be sorted
enum Mixed {
    A,
    B = 5,
    C,
    D = 'delta'
}

// Another string enum
enum Color {
    Red = 'red',
    Blue = 'blue',
    Green = 'green',
    Yellow = 'yellow'
}