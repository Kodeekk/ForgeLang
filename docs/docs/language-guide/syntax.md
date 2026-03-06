---
sidebar_position: 2
---

# Syntax Basics

Learn the fundamental syntax rules of ForgeLang.

## Comments

```forge
// Single-line comment

/*
 * Multi-line comment
 * Can span multiple lines
 */

/// Documentation comment (for functions, types, etc.)
/// These are extracted for documentation generation
fn documented_function() -> void { ... }
```

## Identifiers and Naming

```forge
// Valid identifiers
var count = 0;
var user_name = "Alice";
var _private = "underscore prefix allowed";
var HTTPClient = "PascalCase for types";

// Invalid identifiers
// var 123abc = 0;     // Cannot start with number
// var my-var = 0;     // Hyphens not allowed
// var my var = 0;     // Spaces not allowed

// Naming conventions
var snake_case_variable = 0;    // Variables and functions
fn snake_case_function() {}
class PascalCaseClass {}        // Classes, enums, interfaces
enum PascalCaseEnum {}
type PascalCaseType = int;      // Type aliases
const SCREAMING_CASE = 0;       // Constants
```

## Keywords

Reserved keywords that cannot be used as identifiers:

```
as        break     case      class     const     continue
default   else      enum      export    extends   false
fn        for       if        implements import   in
interface let       match     module    new       null
return    self      static    super     switch    true
type      var       where     while     with      yield
Option    Result    Some      None      Ok        Err
```

## Semicolons

Semicolons terminate statements:

```forge
// Semicolons required for statements
var x = 5;
var y = 10;

// Optional at end of blocks
fn example() -> int {
    var a = 1;
    var b = 2;
    a + b  // No semicolon needed for return value
}

// Required in function bodies for non-return statements
fn multi_statement() -> void {
    var x = 1;  // semicolon required
    var y = 2;  // semicolon required
}
```

## Whitespace and Formatting

```forge
// Indentation: 4 spaces recommended
fn example() -> void {
    var x = 1;
    if x > 0 {
        println("positive");
    }
}

// Blank lines for separation
var first = 1;
var second = 2;

fn function_one() -> void { ... }

fn function_two() -> void { ... }

// Spaces around operators
var sum = a + b;
var result = x * y;

// No spaces in function calls
println("Hello");
list.map(items, fn);

// Space after comma
var items = [1, 2, 3];
fn(a, b, c) { ... }
```

## String Literals

```forge
// Basic strings
var greeting = "Hello, World!";

// Escape sequences
var with_newline = "Line 1\nLine 2";
var with_tab = "Column 1\tColumn 2";
var with_quote = "She said \"Hello\"";
var with_backslash = "C:\\path\\to\\file";

// Raw strings (no escape processing)
var raw = r#"Raw string with "quotes" and \backslash"#;

// Multi-line strings
var multi = "Line 1
Line 2
Line 3";

// String interpolation
var name = "Alice";
var age = 30;
var message = "My name is {name} and I'm {age} years old";
var expression = "Next year: {age + 1}";
```

## Numeric Literals

```forge
// Integers
var decimal = 42;
var hex = 0x2A;         // 42 in hexadecimal
var octal = 0o52;       // 42 in octal
var binary = 0b101010;  // 42 in binary

// Underscores for readability
var million = 1_000_000;
var ipv4 = 192_168_1_1;

// Floats
var pi = 3.14159;
var scientific = 1.5e10;     // 1.5 × 10^10
var negative_exp = 2.5e-3;   // 0.0025

// Type suffixes (optional)
var int_val: int = 42;
var float_val: f64 = 3.14;
```

## Boolean Literals

```forge
var is_true = true;
var is_false = false;

// Case-sensitive (True, TRUE, False, FALSE are invalid)
```

## Arrays and Lists

```forge
// Array literals
var numbers = [1, 2, 3, 4, 5];
var empty: list<int> = [];

// Nested arrays
var matrix = [
    [1, 2, 3],
    [4, 5, 6],
    [7, 8, 9]
];

// Access and modification
var first = numbers[0];
numbers[0] = 10;
```

## Tuples

```forge
// Tuple literals
var pair = (1, 2);
var triple = (1, "two", 3.0);

// Access by index
var x = pair.0;
var y = pair.1;

// Destructuring
var (a, b) = pair;
var (_, second, _) = triple;  // Ignore values with _
```

## Blocks and Scope

```forge
// Block expressions
var result = {
    var x = 10;
    var y = 20;
    x + y  // Returns 30
};

// Nested scope
{
    var inner = "visible only here";
    println(inner);
}
// inner is not accessible here

// Shadowing
var x = 1;
{
    var x = 2;  // Shadows outer x
    println(x);  // Prints 2
}
println(x);  // Prints 1
```

## Module Declaration

Every file starts with a module declaration:

```forge
module my_module;

// Submodules
module my_module.submodule;

// Module paths
module com.example.project;
```

## Next Steps

- **[Types](./types)** - Learn about ForgeLang's type system
- **[Variables](./variables)** - Understand declarations and scope
- **[Operators](./operators)** - Explore available operators
