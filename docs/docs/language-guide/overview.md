---
sidebar_position: 1
---

# Language Overview

Welcome to the ForgeLang Language Guide! This comprehensive guide covers all aspects of the language.

## Table of Contents

1. **[Syntax Basics](./syntax)** - Lexical structure, comments, naming
2. **[Types](./types)** - Type system, primitives, complex types
3. **[Variables](./variables)** - Declarations, scope, mutability
4. **[Operators](./operators)** - Arithmetic, comparison, logical
5. **[Control Flow](./control-flow)** - Conditionals, loops, pattern matching
6. **[Functions](./functions)** - Definition, parameters, closures
7. **[Modules](./modules)** - Organization, imports, visibility

## A Quick Tour

Here's a taste of what ForgeLang looks like:

```forge
module example;

import [println] from std.io;
import std.math;

// Enum with associated data
enum Result<T, E> {
    Ok(value: T),
    Err(error: E)
}

// Interface definition
interface Displayable {
    fn to_str(self) -> str;
}

// Class with generics
class Vector<T> implements Displayable {
    var x: T;
    var y: T;
    
    fn new(x: T, y: T) -> Vector<T> {
        return Vector { x: x, y: y };
    }
    
    fn magnitude(self) -> f64 
    where T: Into<f64> {
        return math.sqrt(self.x * self.x + self.y * self.y);
    }
    
    fn to_str(self) -> str {
        return "Vector({self.x}, {self.y})";
    }
}

// Generic function with constraints
fn sum<T>(numbers: list<T>) -> T 
where T: Add + Zero {
    return numbers.reduce(T.zero(), fn(a, b) { return a + b; });
}

// Pattern matching
fn process_result(r: Result<int, str>) -> void {
    match r {
        Result.Ok(n) => println("Success: {n}"),
        Result.Err(e) => println("Error: {e}")
    }
}

fn main() -> int {
    var v = Vector.new(3.0, 4.0);
    println("Magnitude: {v.magnitude()}");
    println("As string: {v.to_str()}");
    
    return 0;
}
```

## Key Concepts

### Everything is an Expression

In ForgeLang, almost everything is an expression that returns a value:

```forge
// if/else is an expression
var result = if x > 0 { "positive" } else { "negative" };

// match is an expression
var status = match code {
    200 => "OK",
    404 => "Not Found",
    _ => "Error"
};

// Blocks return their last value
fn get_value() -> int {
    var x = 10;
    var y = 20;
    x + y  // Returns 30
}
```

### Type Inference

ForgeLang infers types when possible:

```forge
var x = 42;           // int
var y = 3.14;         // f64
var name = "Alice";   // str
var items = [1, 2];   // list<int>
```

### Immutability by Default

Constants are immutable; use `var` for mutable values:

```forge
const MAX = 100;  // Cannot be changed
var count = 0;    // Can be modified
count = 1;        // OK
```

### Pattern Matching

Exhaustive pattern matching for safe, clear code:

```forge
enum Message {
    Quit,
    Move { x: int, y: int },
    Write(str),
    ChangeColor(int, int, int)
}

fn handle(msg: Message) -> void {
    match msg {
        Message.Quit => println("Quitting"),
        Message.Move { x, y } => println("Moving to ({x}, {y})"),
        Message.Write(text) => println("Writing: {text}"),
        Message.ChangeColor(r, g, b) => println("Color: RGB({r}, {g}, {b})")
    }
}
```

## Next Steps

Choose a topic to dive deeper:

- **[Syntax Basics](./syntax)** - Learn the lexical structure
- **[Types](./types)** - Understand the type system
- **[Functions](./functions)** - Master function definitions
