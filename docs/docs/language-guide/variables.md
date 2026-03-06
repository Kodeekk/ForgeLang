---
sidebar_position: 4
---

# Variables

Learn how to declare and use variables in ForgeLang.

## Declarations

### Constants (`const`)

Immutable values that cannot be changed:

```forge
// Basic constant
const PI: f64 = 3.14159265358979;

// Type inference
const VERSION = "2.0.0";

// Compile-time constants
const MAX_USERS: int = 1000;
const DEBUG_MODE: bool = false;

// Constants must be initialized
// const INVALID;  // Error: missing initializer
```

### Mutable Variables (`var`)

Values that can be modified:

```forge
// Basic variable
var count: int = 0;

// Type inference
var name = "Alice";

// Modify value
count = count + 1;
name = "Bob";

// Change type (dynamic typing)
var x = 42;       // int
x = "now string"; // str - valid!
```

## Scope

Variables are scoped to their block:

```forge
// Module-level (global)
var global_count: int = 0;

fn example() -> void {
    // Function-level scope
    var local = "visible in function";
    
    if true {
        // Block scope
        var inner = "visible only in this block";
        println(inner);  // OK
    }
    // println(inner);  // Error: out of scope
}

// Loops create scope
for i in list.range(0, 10) {
    println(i);  // i visible here
}
// println(i);  // Error: i out of scope
```

## Shadowing

Create new variables with the same name:

```forge
var x = 1;
println(x);  // 1

{
    var x = 2;  // Shadows outer x
    println(x);  // 2
}

println(x);  // 1 - outer x still exists

// Common pattern: transform value
let value: any = "42";
let value = int(value);  // Now int
let value = value * 2;   // Now 84
```

## Naming Conventions

```forge
// snake_case for variables and functions
var user_count = 0;
fn calculate_total() { ... }

// PascalCase for types
class UserProfile { ... }
enum HttpStatusCode { ... }

// SCREAMING_CASE for constants
const MAX_RETRIES = 3;
const API_VERSION = "v2";

// Private convention (underscore prefix)
var _internal_cache = map.new();
fn _helper_function() { ... }
```

## Destructuring

Unpack values from tuples and objects:

```forge
// Tuple destructuring
var point: (int, int) = (10, 20);
var (x, y) = point;
println(x);  // 10
println(y);  // 20

// Ignore values
var (_, y) = point;  // Ignore x

// Object destructuring
class Point { var x: int; var y: int; }
var p = Point { x: 10, y: 20 };
var Point { x, y } = p;

// Rename in destructuring
var Point { x: x_coord, y: y_coord } = p;
```

## Global Variables

Module-level variables:

```forge
module my_app;

// Global mutable state
var total_requests: int = 0;
var config: map<str, any> = map.new();

// Global constants
const APP_NAME: str = "MyApp";
const VERSION: str = "1.0.0";

fn increment_requests() -> void {
    total_requests = total_requests + 1;
}

fn get_config(key: str) -> any {
    return config.get(key);
}
```

## Variable Hoisting

Variables are not hoisted in ForgeLang:

```forge
// println(x);  // Error: x not defined yet
var x = 42;

// Must declare before use
fn example() -> void {
    var y = 10;  // Declaration
    y = 20;      // Assignment
}
```

## Lazy Initialization

Initialize variables when needed:

```forge
// Declare without initializing
var cache: map<str, any>;

fn get_cache() -> map<str, any> {
    if cache == null {
        cache = map.new();
    }
    return cache;
}

// Or use Option
var cache: Option<map<str, any>> = Option.None;

fn get_cache() -> map<str, any> {
    match cache {
        Option.Some(c) => c,
        Option.None => {
            var new_cache = map.new();
            cache = Option.Some(new_cache);
            new_cache
        }
    }
}
```

## Type Annotations

When to use explicit types:

```forge
// Can omit - type is obvious
var count = 0;
var name = "Alice";

// Recommended for clarity
var user_count: int = 0;
var ratio: f64 = 0.75;

// Required for empty collections
var items: list<int> = [];
var mapping: map<str, int> = map.new();

// Required for function signatures
fn add(a: int, b: int) -> int { ... }
```

## Best Practices

1. **Prefer `const`** - Use constants unless mutation is needed
2. **Descriptive names** - `user_count` not `uc`
3. **Limit scope** - Declare variables close to use
4. **Initialize promptly** - Don't leave variables uninitialized
5. **Use type annotations** - For complex types or API boundaries

```forge
// Good
const MAX_RETRIES = 3;
var retry_count: int = 0;

fn process_user(user_id: int, name: str) -> Result<User, str> {
    var attempts = 0;
    while attempts < MAX_RETRIES {
        // ...
        attempts = attempts + 1;
    }
}

// Avoid
var m = 3;  // What is m?
var x;      // Uninitialized
var d = []; // What type?
```

## Next Steps

- **[Operators](./operators)** - Learn about operators
- **[Control Flow](./control-flow)** - Conditionals and loops
- **[Functions](./functions)** - Function definitions
