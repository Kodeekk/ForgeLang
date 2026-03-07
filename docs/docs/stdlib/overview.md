---
sidebar_position: 1
---

# Standard Library Overview

The ForgeLang Standard Library provides essential functionality for system orchestration and automation tasks.

## Modules

### System Integration

| Module | Description |
|--------|-------------|
| [`std.proc`](./proc) | Process execution and shell commands |
| [`std.env`](./env) | Environment variables and system info |
| [`std.fs`](./fs) | Filesystem operations |
| [`std.io`](./io) | Input/Output operations |

### Data Types

| Module | Description |
|--------|-------------|
| [`std.str`](./str) | String manipulation |
| [`std.list`](./list) | List operations and utilities |
| [`std.math`](./math) | Mathematical functions and constants |
| [`std.int`](./int) | Integer utilities |
| [`std.float`](./float) | Floating-point utilities |
| [`std.bool`](./bool) | Boolean utilities |

### Utilities

| Module | Description |
|--------|-------------|
| [`std.time`](./time) | Time and duration utilities |

## Import Patterns

```forge
// Import specific functions
import [println, read_line] from std.io;

// Import entire module
import std.proc;
var output = proc.capture("ls -1");

// Import with alias
import std.io as io;
io.println("Hello");

// Import multiple modules
import std.proc;
import std.env;
import std.fs;
```

## Design Principles

The standard library follows these principles:

1. **Shell-Native** - System integration is first-class
2. **Consistency** - Similar functions have similar names and signatures
3. **Completeness** - Cover common automation use cases
4. **Safety** - Handle edge cases gracefully
5. **Documentation** - Clear examples for all functions

## Quick Examples

### Process Execution

```forge
import std.proc;

// Execute command
var code = proc.exec("make build");

// Capture output
var branch = proc.capture("git branch --show-current");
```

### Environment Access

```forge
import std.env;

var user = env.get("USER");
var home = env.get("HOME");
var cwd = env.cwd();
```

### Filesystem Operations

```forge
import std.fs;

var content = fs.read("config.yaml");
fs.write("output.txt", content);
var exists = fs.exists("file.txt");
```

### String Processing

```forge
import std.str;

var trimmed = "  hello  ".trim();
var upper = "hello".upper();
var parts = "a,b,c".split(",");
```

### List Operations

```forge
import std.list;

var nums = [1, 2, 3, 4, 5];
var sum = list.sum(nums);
var evens = list.filter(nums, fn(n) { return n % 2 == 0; });
```

## Module Categories

### Shell & System

Use these modules for system automation:

```forge
import std.proc;    // Run commands
import std.env;     // Environment variables
import std.fs;      // File operations
```

### Data Processing

Use these modules for data manipulation:

```forge
import std.str;     // String handling
import std.list;    // List operations
import std.math;    // Math functions
```

### I/O

Use these modules for input/output:

```forge
import std.io;      // Print, read
import std.time;    // Timing, sleep
```

## Next Steps

Explore the modules:

- **[std.proc](./proc)** - Execute commands, capture output
- **[std.env](./env)** - Environment variables
- **[std.fs](./fs)** - Filesystem operations
- **[std.io](./io)** - Print, read, format
- **[std.str](./str)** - String manipulation
- **[std.list](./list)** - List utilities
- **[std.math](./math)** - Math functions
