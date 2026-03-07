---
sidebar_position: 1
---

# API Reference Overview

Complete API reference for the ForgeLang Standard Library.

## Module Index

### Core Modules

| Module | Description | Functions | Classes |
|--------|-------------|-----------|---------|
| [`std.io`](./io) | Input/Output | 7 | - |
| [`std.math`](./math) | Mathematics | 14 | - |
| [`std.list`](./list) | List operations | 17 | - |
| [`std.str`](./str) | String manipulation | - | 1 (15 methods) |
| [`std.bool`](./bool) | Boolean operations | - | 1 (7 methods) |
| [`std.int`](./int) | Integer operations | - | 1 (10 methods) |
| [`std.float`](./float) | Float operations | - | 1 (13 methods) |
| [`std.fs`](./fs) | Filesystem | 11 | - |
| [`std.time`](./time) | Time utilities | 8 | - |
| [`std.env`](./env) | Environment | 10 | - |

## Quick Reference

### I/O Functions
```forge
print(value)              // Print without newline
println(value)            // Print with newline
eprint(value)             // Print to stderr
eprintln(value)           // Print to stderr with newline
read_line() -> str        // Read line from stdin
read_all() -> str         // Read all from stdin
format(value) -> str      // Format as string
```

### Math Functions
```forge
abs(n)                    // Absolute value
min(a, b)                 // Minimum
max(a, b)                 // Maximum
pow(base, exp)            // Exponentiation
sqrt(n)                   // Square root
sign(n)                   // Sign (-1, 0, 1)
is_even(n)                // Check if even
clamp(v, min, max)        // Clamp to range
gcd(a, b)                 // Greatest common divisor
lcm(a, b)                 // Least common multiple
is_prime(n)               // Check if prime
factorial(n)              // Factorial
fib(n)                    // Fibonacci number
```

### List Functions
```forge
range(start, end)         // Create range
repeat(value, count)      // Repeat value
sum(list)                 // Sum elements
product(list)             // Product of elements
filter(list, pred)        // Filter by predicate
map(list, func)           // Transform elements
reduce(list, init, func)  // Reduce to value
any(list, pred)           // Any match
all(list, pred)           // All match
find_index(list, val)     // Find index
count(list, val)          // Count occurrences
reverse(list)             // Reverse list
take(list, n)             // Take first n
skip(list, n)             // Skip first n
concat(a, b)              // Concatenate
dedup(list)               // Remove duplicates
slice(list, start, end)   // Get sublist
```

## Type Hierarchy

```
any (top type)
├── int
├── f64
├── bool
├── str
├── void
├── list<T>
├── map<K, V>
├── set<T>
├── (T, U, ...)  // tuples
├── Option<T>
│   ├── Some(T)
│   └── None
└── Result<T, E>
    ├── Ok(T)
    └── Err(E)
```

## Interfaces

```forge
interface Displayable {
    fn to_str(self) -> str;
}

interface Runnable {
    fn run(self) -> void;
}

interface Comparable<T> {
    fn compare(self, other: T) -> int;
    fn eq(self, other: T) -> bool;
    fn lt(self, other: T) -> bool;
    fn gt(self, other: T) -> bool;
}

interface Serializable {
    fn to_json(self) -> str;
    fn from_json(raw: str) -> Self;
}

interface Iterator<T> {
    fn next(self) -> Option<T>;
    fn has_next(self) -> bool;
    fn reset(self) -> void;
}
```

## Error Handling

All stdlib functions use `Result` and `Option` for error handling:

```forge
// Option for nullable values
fn find(list: list<int>, target: int) -> Option<int>

// Result for operations that can fail
fn read_file(path: str) -> Result<str, str>
fn safe_divide(a: f64, b: f64) -> Result<f64, str>
```

## Next Steps

- **[std.io](./io)** - I/O operations
- **[std.math](./math)** - Math functions
- **[std.list](./list)** - List utilities
