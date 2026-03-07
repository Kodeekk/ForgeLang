---
sidebar_position: 3
---

# Types

ForgeLang has a rich type system that balances flexibility with safety.

## Type Categories

### Primitive Types

```forge
// Integers
var int_val: int = 42;

// Floating-point numbers
var float_val: f64 = 3.14159;

// Booleans
var bool_val: bool = true;

// Strings
var str_val: str = "Hello, ForgeLang!";

// Unit type (no value)
var unit_val: void = ();  // Only one value: ()
```

### Compound Types

```forge
// Lists (dynamic arrays)
var numbers: list<int> = [1, 2, 3];
var names: list<str> = ["Alice", "Bob"];

// Object literals (key-value pairs)
var config = { name: "app", version: 1 };

// Tuples
var pair: (int, str) = (42, "answer");
var triple: (int, f64, bool) = (1, 3.14, true);
```

### User-Defined Types

```forge
// Classes
class Point {
    var x: int;
    var y: int;
}

// Enums
enum Direction {
    North,
    South,
    East,
    West
}

// Type aliases
type Coordinate = (int, int);
type Predicate<T> = fn(T) -> bool;
```

## Type Inference

ForgeLang infers types automatically:

```forge
// Obvious types
var x = 42;           // int
var y = 3.14;         // f64
var name = "Alice";   // str
var flag = true;      // bool

// Collections
var nums = [1, 2, 3];           // list<int>
var items = ["a", "b"];         // list<str>
var empty: list<int> = [];      // Explicit for empty

// Function return types
fn get_number() { 42 }  // Inferred as int
```

## Type Annotations

Explicit type annotations when needed:

```forge
// Variable annotations
var count: int = 0;
var ratio: f64 = 0.5;

// Function parameters
fn add(a: int, b: int) -> int {
    return a + b;
}

// Function return type
fn greet(name: str) -> str {
    return "Hello, {name}!";
}

// Complex types
var matrix: list<list<f64>> = [];
var callback: fn(int) -> str;
```

## Generic Types

Generics enable reusable, type-safe code:

```forge
// Generic class
class Stack<T> {
    var items: list<T>;
    
    fn push(self, item: T) -> void { ... }
    fn pop(self) -> Option<T> { ... }
}

// Generic function
fn identity<T>(value: T) -> T {
    return value;
}

// Multiple type parameters
fn zip<A, B>(a: list<A>, b: list<B>) -> list<(A, B)> { ... }

// Generic constraints
fn sort<T: Comparable<T>>(items: list<T>) -> list<T> { ... }
```

## Option Type

Handle nullable values safely:

```forge
enum Option<T> {
    Some(value: T),
    None
}

// Usage
fn find(list: list<int>, target: int) -> Option<int> {
    for item in list {
        if item == target {
            return Option.Some(item);
        }
    }
    return Option.None;
}

// Pattern matching
match find(numbers, 42) {
    Option.Some(n) => println("Found: {n}"),
    Option.None => println("Not found")
}

// unwrap_or
var result = find(numbers, 42).unwrap_or(0);
```

## Result Type

Error handling without exceptions:

```forge
enum Result<T, E> {
    Ok(value: T),
    Err(error: E)
}

// Usage
fn safe_divide(a: f64, b: f64) -> Result<f64, str> {
    if b == 0.0 {
        return Result.Err("Division by zero");
    }
    return Result.Ok(a / b);
}

// Pattern matching
match safe_divide(10.0, 2.0) {
    Result.Ok(value) => println("Result: {value}"),
    Result.Err(e) => println("Error: {e}")
}

// Helper functions
fn unwrap_or<T>(result: Result<T, str>, fallback: T) -> T {
    return match result {
        Result.Ok(v) => v,
        Result.Err(_) => fallback
    };
}

var value = unwrap_or(safe_divide(10.0, 0.0), -1.0);
```

## Type Conversions

```forge
// Explicit conversions
var int_val: int = 42;
var float_val: f64 = f64(int_val);  // int to f64

var float_num: f64 = 3.14;
var int_num: int = int(float_num);  // f64 to int (truncates)

var num: int = 42;
var str_val: str = str(num);        // int to str

var str_num: str = "123";
var parsed: int = int(str_num);     // str to int

// Using methods
var n: int = 42;
var s: str = n.to_str();

var f: f64 = 3.14;
var i: int = f.to_int();
```

## Type Checking

```forge
// Runtime type checks
var x: any = 42;

if x is int {
    println("It's an int!");
}

if x is str {
    println("It's a string!");
}

// Type narrowing
fn process(value: any) -> void {
    if value is int {
        // value is narrowed to int here
        println(value + 1);
    } else if value is str {
        // value is narrowed to str here
        println(value.length());
    }
}
```

## Any Type

Dynamic typing when needed:

```forge
// Accept any type
fn print_any(value: any) -> void {
    println(value);
}

print_any(42);
print_any("Hello");
print_any([1, 2, 3]);

// Store heterogeneous collections
var mixed: list<any> = [1, "two", 3.0, true];
```

## Type Aliases

Create meaningful names for complex types:

```forge
// Simple alias
type UserId = int;
type Username = str;

// Function types
type Comparator<T> = fn(T, T) -> int;
type Predicate<T> = fn(T) -> bool;
type Transform<T> = fn(T) -> T;

// Complex types
type Matrix = list<list<f64>>;
type Graph<V, E> = map<V, list<(V, E)>>;

// Usage
fn compare_users: Comparator<User> = ...;
fn is_valid: Predicate<Form> = ...;
```

## Next Steps

- **[Variables](./variables)** - Learn about declarations and scope
- **[Operators](./operators)** - Explore operators
- **[Generics](../advanced/generics)** - Deep dive into generics
