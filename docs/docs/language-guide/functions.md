---
sidebar_position: 7
---

# Functions

Functions are the building blocks of ForgeLang programs.

## Function Definition

Basic function syntax:

```forge
// Function with no parameters
fn greet() -> void {
    println("Hello!");
}

// Function with parameters
fn add(a: int, b: int) -> int {
    return a + b;
}

// Function with multiple return types (tuple)
fn get_point() -> (int, int) {
    return (10, 20);
}

// Expression body (single expression)
fn double(x: int) -> int { x * 2 }

// No return type (returns void)
fn print_twice(msg: str) -> void {
    println(msg);
    println(msg);
}
```

## Parameters

Different parameter patterns:

```forge
// Multiple parameters
fn greet(name: str, greeting: str) -> void {
    println("{greeting}, {name}!");
}

// Same type shorthand
fn add(a: int, b: int) -> int { a + b }

// Default parameters (via overloading pattern)
fn greet(name: str) -> void {
    greet_with_greeting(name, "Hello");
}

fn greet_with_greeting(name: str, greeting: str) -> void {
    println("{greeting}, {name}!");
}

// Variadic (using list)
fn sum(numbers: list<int>) -> int {
    return numbers.reduce(0, fn(a, b) { return a + b; });
}
```

## Return Values

Different return patterns:

```forge
// Explicit return
fn max(a: int, b: int) -> int {
    if a > b { return a; }
    return b;
}

// Implicit return (last expression)
fn min(a: int, b: int) -> int {
    if a < b { a } else { b }
}

// Early return
fn find_positive(numbers: list<int>) -> Option<int> {
    for n in numbers {
        if n > 0 { return Option.Some(n); }
    }
    return Option.None;
}

// Return tuple
fn get_coordinates() -> (int, int, int) {
    return (10, 20, 30);
}

// Destructure return
var (x, y, z) = get_coordinates();
```

## Function Types

Functions as first-class values:

```forge
// Function type annotation
var callback: fn(int) -> str;

// Assign function to variable
fn to_str(n: int) -> str { str(n) }
var converter = to_str;
var result = converter(42);

// Function as parameter
fn apply(value: int, f: fn(int) -> int) -> int {
    return f(value);
}

var doubled = apply(5, fn(x) { return x * 2; });

// Function as return value
fn make_adder(n: int) -> fn(int) -> int {
    return fn(x) { return x + n; };
}

var add_five = make_adder(5);
var result = add_five(10);  // 15
```

## Closures

Anonymous functions with captured variables:

```forge
// Basic closure
var numbers = [1, 2, 3, 4, 5];
var doubled = numbers.map(fn(n) { return n * 2; });

// Capturing variables
var multiplier = 3;
var times_three = fn(x) { return x * multiplier; };
var result = times_three(5);  // 15

// Mutable capture
var count = 0;
var increment = fn() {
    count = count + 1;
    return count;
};
println(increment());  // 1
println(increment());  // 2

// Closure in struct
class Counter {
    var count: int;
    var increment: fn() -> int;
    
    fn new() -> Counter {
        var count = 0;
        return Counter {
            count: count,
            increment: fn() {
                count = count + 1;
                return count;
            }
        };
    }
}
```

## Higher-Order Functions

Functions that take or return functions:

```forge
// Map - transform each element
var numbers = [1, 2, 3];
var squares = numbers.map(fn(n) { return n * n; });

// Filter - keep elements matching predicate
var evens = numbers.filter(fn(n) { return n % 2 == 0; });

// Reduce - combine elements
var sum = numbers.reduce(0, fn(acc, n) { return acc + n; });

// Any - check if any element matches
var has_positive = numbers.any(fn(n) { return n > 0; });

// All - check if all elements match
var all_positive = numbers.all(fn(n) { return n > 0; });

// Compose - combine functions
fn compose<A, B, C>(f: fn(B) -> C, g: fn(A) -> B) -> fn(A) -> C {
    return fn(x: A) -> C { return f(g(x)); };
}

var double_then_add_one = compose(
    fn(n) { return n + 1; },
    fn(n) { return n * 2; }
);
var result = double_then_add_one(5);  // 11
```

## Recursion

Functions that call themselves:

```forge
// Factorial
fn factorial(n: int) -> int {
    if n <= 1 { return 1; }
    return n * factorial(n - 1);
}

// Fibonacci (naive)
fn fib(n: int) -> int {
    if n <= 1 { return n; }
    return fib(n - 1) + fib(n - 2);
}

// Fibonacci (tail-recursive style)
fn fib_fast(n: int) -> int {
    fn helper(a: int, b: int, n: int) -> int {
        if n <= 0 { return a; }
        return helper(b, a + b, n - 1);
    }
    return helper(0, 1, n);
}

// Recursive data structures
enum List<T> {
    Cons(T, Box<List<T>>),
    Nil
}

fn length<T>(list: List<T>) -> int {
    return match list {
        List.Nil => 0,
        List.Cons(_, rest) => 1 + length(rest)
    };
}
```

## Generic Functions

Functions that work with any type:

```forge
// Basic generic
fn identity<T>(value: T) -> T {
    return value;
}

var num = identity(42);       // T = int
var str_val = identity("hi"); // T = str

// Multiple type parameters
fn swap<A, B>(a: A, b: B) -> (B, A) {
    return (b, a);
}

var (x, y) = swap(1, "one");

// Generic with constraints
fn sort<T: Comparable<T>>(items: list<T>) -> list<T> {
    // ...
}

// Generic with multiple constraints
fn process<T>(value: T) -> void 
where T: Displayable + Clone {
    println(value.to_str());
}
```

## Function Overloading

Multiple functions with same name:

```forge
// Different parameter types
fn print(value: int) -> void {
    println("Integer: {value}");
}

fn print(value: str) -> void {
    println("String: {value}");
}

fn print(value: f64) -> void {
    println("Float: {value}");
}

// Different parameter count
fn greet(name: str) -> void {
    println("Hello, {name}!");
}

fn greet(name: str, greeting: str) -> void {
    println("{greeting}, {name}!");
}
```

## Inline Functions

Performance-critical functions:

```forge
// Hint for inlining (compiler decides)
fn small_function(x: int) -> int { x + 1 }

// Typically inlined automatically
fn add(a: int, b: int) -> int { a + b }
fn max(a: int, b: int) -> int { if a > b { a } else { b } }
```

## Best Practices

1. **Keep functions small** - Single responsibility
2. **Use descriptive names** - `calculate_total` not `calc`
3. **Document parameters and returns**
4. **Prefer pure functions** - No side effects when possible
5. **Use early returns** - Avoid deep nesting

```forge
// Good
fn calculate_total(price: f64, quantity: int, tax_rate: f64) -> f64 {
    var subtotal = price * f64(quantity);
    return subtotal * (1.0 + tax_rate);
}

// Avoid
fn calc(p, q, t) { p * q * (1 + t) }  // Unclear names, no types
```

## Next Steps

- **[Modules](./modules)** - Organize code
- **[Closures](../advanced/closures)** - Advanced closure patterns
- **[Generics](../advanced/generics)** - Deep dive into generics
