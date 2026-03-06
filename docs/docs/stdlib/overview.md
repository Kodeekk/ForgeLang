---
sidebar_position: 1
---

# Standard Library Overview

The ForgeLang Standard Library provides essential functionality for common programming tasks.

## Modules

| Module | Description |
|--------|-------------|
| [`std.io`](./io) | Input/Output operations |
| [`std.math`](./math) | Mathematical functions and constants |
| [`std.list`](./list) | List operations and utilities |
| [`std.str`](./str) | String manipulation |
| [`std.bool`](./bool) | Boolean operations |
| [`std.int`](./int) | Integer operations |
| [`std.float`](./float) | Floating-point operations |
| [`std.fs`](./fs) | Filesystem operations |
| [`std.time`](./time) | Time and duration utilities |
| [`std.env`](./env) | Environment variables and system info |

## Import Patterns

```forge
// Import specific functions
import [println, read_line] from std.io;

// Import entire module
import std.math;
var result = math.sqrt(144);

// Import with alias
import std.io as io;
io.println("Hello");

// Import multiple modules
import std.io;
import std.math;
import std.list;
```

## Design Principles

The standard library follows these principles:

1. **Consistency** - Similar functions have similar names and signatures
2. **Completeness** - Cover common use cases thoroughly
3. **Performance** - Efficient implementations
4. **Safety** - Handle edge cases gracefully
5. **Documentation** - Clear examples for all functions

## Next Steps

Explore the modules:

- **[std.io](./io)** - Print, read, format
- **[std.math](./math)** - Math functions
- **[std.list](./list)** - List utilities
