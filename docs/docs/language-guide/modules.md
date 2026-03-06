---
sidebar_position: 8
---

# Modules

Organize your code into modules for better structure and reusability.

## Module Declaration

Every file starts with a module declaration:

```forge
// Basic module
module my_app;

// Nested module
module my_app.utils;

// Deep nesting
module com.example.project.utils;
```

## Imports

Import functionality from other modules:

```forge
// Import entire module
import std.io;
io.println("Hello");

// Import with alias
import std.io as my_io;
my_io.println("Hello");

// Import specific items
import [println, read_line] from std.io;
println("Hello");  // Direct access

// Import with renaming
import [println as print] from std.io;
print("Hello");

// Multiple imports
import std.io;
import std.math;
import std.list;

// Import from custom modules
import [helper_function] from my_app.utils;
```

## Module Structure

Organize code across files:

```
my_project/
├── src/
│   ├── main.fl           # module my_project;
│   ├── lib.fl            # module my_project;
│   ├── utils/
│   │   ├── mod.fl        # module my_project.utils;
│   │   ├── strings.fl    # module my_project.utils.strings;
│   │   └── collections.fl# module my_project.utils.collections;
│   └── models/
│       ├── user.fl       # module my_project.models.user;
│       └── product.fl    # module my_project.models.product;
```

## Visibility

Control what's exported from modules:

```forge
module my_module;

// Public (exported)
export fn public_function() -> void { ... }
export class PublicClass { ... }
export const PUBLIC_VALUE: int = 42;

// Private (module-only)
fn private_function() -> void { ... }
class PrivateClass { ... }
const PRIVATE_VALUE: int = 0;

// Re-export from other modules
export [println, read_line] from std.io;
```

## Circular Dependencies

Avoid circular imports:

```forge
// ❌ Bad: Circular dependency
// module_a.fl
module module_a;
import [func_b] from module_b;

// module_b.fl
module module_b;
import [func_a] from module_a;

// ✅ Good: Extract shared code
// module_common.fl
module module_common;
export fn shared_function() { ... }

// module_a.fl
module module_a;
import [shared_function] from module_common;

// module_b.fl
module module_b;
import [shared_function] from module_common;
```

## Using Modules

Practical module organization:

```forge
// src/utils/strings.fl
module my_app.utils.strings;

export fn slugify(s: str) -> str {
    return s.trim().lower().replace(" ", "-");
}

export fn truncate(s: str, max_len: int) -> str {
    if s.length() <= max_len { return s; }
    return s.slice(0, max_len - 3) + "...";
}

// src/utils/collections.fl
module my_app.utils.collections;

export fn sum(numbers: list<int>) -> int {
    return numbers.reduce(0, fn(a, b) { return a + b; });
}

export fn average(numbers: list<int>) -> f64 {
    return f64(sum(numbers)) / f64(numbers.length());
}

// src/main.fl
module my_app;

import [slugify, truncate] from my_app.utils.strings;
import [sum, average] from my_app.utils.collections;
import [println] from std.io;

fn main() -> int {
    println(slugify("Hello World"));
    println(sum([1, 2, 3, 4, 5]));
    return 0;
}
```

## Module Patterns

Common organization patterns:

```forge
// Facade pattern - single export point
module my_app.utils;

export [slugify, truncate, capitalize] from my_app.utils.strings;
export [sum, average, max, min] from my_app.utils.collections;
export [read_json, write_json] from my_app.utils.json;

// Usage
import [slugify, sum] from my_app.utils;

// Namespace pattern
module my_app.http;

export class Request { ... }
export class Response { ... }
export fn get(url: str) -> Response { ... }
export fn post(url: str, data: any) -> Response { ... }

// Usage
import std.http;
var resp = http.get("https://api.example.com");
```

## Standard Library Modules

ForgeLang stdlib organization:

```forge
// I/O operations
import [println, read_line] from std.io;

// Mathematics
import std.math;
var result = math.sqrt(144);

// List utilities
import std.list;
var sum = list.sum([1, 2, 3]);

// String operations
import std.str;
var upper = "hello".upper();

// Filesystem
import std.fs;
var content = std.fs.read("file.txt");

// Time operations
import std.time;
var now = std.time.now();

// Environment
import std.env;
var home = std.env.home();
```

## Creating Modules

Step-by-step module creation:

```forge
// 1. Create module file: src/math/vector.fl
module my_app.math.vector;

export class Vector2 {
    var x: f64;
    var y: f64;
    
    fn new(x: f64, y: f64) -> Vector2 {
        return Vector2 { x: x, y: y };
    }
    
    fn magnitude(self) -> f64 {
        return builtin_math_sqrt(self.x * self.x + self.y * self.y);
    }
    
    fn add(self, other: Vector2) -> Vector2 {
        return Vector2 {
            x: self.x + other.x,
            y: self.y + other.y
        };
    }
}

// 2. Export from parent module: src/math/mod.fl
module my_app.math;

export [Vector2] from my_app.math.vector;

// 3. Use in main: src/main.fl
module my_app;

import [Vector2] from my_app.math;

fn main() -> int {
    var v = Vector2.new(3.0, 4.0);
    println("Magnitude: {v.magnitude()}");
    return 0;
}
```

## Best Practices

1. **One module per file** - Keep files focused
2. **Descriptive module names** - `utils.strings` not `u.s`
3. **Minimize exports** - Export only what's needed
4. **Avoid deep nesting** - 2-3 levels max
5. **Document public API** - Explain what each export does

```forge
// Good module structure
module my_app.http;

/// Makes an HTTP GET request to the specified URL.
/// Returns a Response object or an error.
export fn get(url: str) -> Result<Response, str> { ... }

/// Makes an HTTP POST request with JSON body.
export fn post_json(url: str, data: any) -> Result<Response, str> { ... }

// Internal helper (not exported)
fn parse_headers(raw: str) -> map<str, str> { ... }
```

## Next Steps

- **[Generics](../advanced/generics)** - Type parameters
- **[Interfaces](../advanced/interfaces)** - Define contracts
- **[Standard Library](../stdlib/overview)** - Explore stdlib modules
