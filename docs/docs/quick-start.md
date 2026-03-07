---
sidebar_position: 3
---

# Quick Start

Get up and running with ForgeLang in minutes. This guide will walk you through writing and running your first programs.

## Your First Program

Create a file called `hello.fl`:

```forge
module hello;

import [println] from std.io;

fn main() -> int {
    println("Hello, ForgeLang!");
    return 0;
}
```

Run it with the ForgeLang interpreter:

```bash
fl hello.fl
```

Output:
```
Hello, ForgeLang!
```

## Using the Package Manager

For larger projects, use `maul`, the ForgeLang package manager.

### Create a New Project

```bash
maul new my_project
cd my_project
```

This creates the following structure:

```
my_project/
├── maul.yaml          # Project manifest
├── src/
│   └── main.fl        # Entry point
└── .gitignore
```

### Project Manifest

The `maul.yaml` file defines your project:

```yaml
package:
  name: "my_project"
  version: "0.1.0"
  entry: "src/main.fl"
```

### Run Your Project

```bash
maul run
```

## Language Basics

### Variables

```forge
// Immutable constants
const PI: f64 = 3.14159;
const NAME: str = "ForgeLang";

// Mutable variables
var count: int = 0;
count = count + 1;

// Type inference
var message = "Hello";  // Inferred as str
var number = 42;        // Inferred as int
```

### Functions

```forge
// Simple function
fn greet(name: str) -> str {
    return "Hello, {name}!";
}

// Function with multiple parameters
fn add(a: int, b: int) -> int {
    return a + b;
}

// Function without return value
fn print_twice(msg: str) -> void {
    println(msg);
    println(msg);
}
```

### Control Flow

```forge
// If/else
var score: int = 85;
if score >= 90 {
    println("A");
} else if score >= 80 {
    println("B");
} else {
    println("C");
}

// Match expression (pattern matching)
enum Color { Red, Green, Blue }
var c = Color.Red;

match c {
    Color.Red => println("Red!"),
    Color.Green => println("Green!"),
    Color.Blue => println("Blue!")
}

// For loop
for i in list.range(0, 5) {
    println("Count: {i}");
}

// While loop
var n: int = 10;
while n > 0 {
    println(n);
    n = n - 1;
}
```

### Collections

```forge
// Lists
var numbers: list<int> = [1, 2, 3, 4, 5];
numbers.push(6);
var first = numbers[0];

// Maps (using object literals)
var config = { name: "myapp", version: "1.0" };
println(config.name);  // "myapp"
```

### String Interpolation

```forge
var name = "Alice";
var age = 30;

// String interpolation with {}
println("My name is {name} and I'm {age} years old");

// Expressions in strings
println("Next year I'll be {age + 1}");
```

## Standard Library

ForgeLang includes a rich standard library:

```forge
import [println, read_line] from std.io;
import std.math;
import std.list;
import std.str;

fn main() -> int {
    // Math
    println(math.sqrt(144));      // 12
    println(math.pow(2, 10));     // 1024
    println(math.abs(-42));       // 42
    
    // List operations
    var nums = [1, 2, 3, 4, 5];
    var sum = list.sum(nums);     // 15
    var evens = list.filter(nums, fn(n) { return n % 2 == 0; });
    
    // String operations
    var s = "  Hello  ";
    println(s.trim());            // "Hello"
    println(s.upper());           // "  HELLO  "
    
    return 0;
}
```

## Error Handling

```forge
// Result type for error handling
fn safe_divide(a: f64, b: f64) -> Result<f64, str> {
    if b == 0.0 {
        return Result.Err("Division by zero");
    }
    return Result.Ok(a / b);
}

// Pattern match on Result
match safe_divide(10.0, 2.0) {
    Result.Ok(value) => println("Result: {value}"),
    Result.Err(e) => println("Error: {e}")
}

// Option type for nullable values
fn find_first(list: list<int>, target: int) -> Option<int> {
    for item in list {
        if item == target {
            return Option.Some(item);
        }
    }
    return Option.None;
}
```

## Next Steps

- **[Language Guide](./language-guide/overview)** - Comprehensive language features
- **[Standard Library](./stdlib/overview)** - Explore the stdlib
- **[Examples](/examples)** - Real-world code examples

## Try It Yourself

Experiment with ForgeLang by modifying the examples above. The best way to learn is by writing code!

```bash
# Create a test file
echo 'println("Your code here")' > test.fl

# Run it
fl test.fl
```

Happy coding! 🚀
