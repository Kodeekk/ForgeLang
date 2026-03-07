# ForgeLang

**A shell-native system orchestration language.**

ForgeLang is an interpreted programming language designed to merge high-level expressiveness with deep, native shell integration. Write structured, readable code that orchestrates system commands, pipelines, and processes with ease.

```forge
import [println] from std.io;
import std.proc;
import std.env;

fn main() {
    // Execute shell commands
    var exit_code = proc.exec("echo Hello from Forge!");
    
    // Capture command output
    var files = proc.capture("ls -1");
    println("Files: {files}");
    
    // Access environment variables
    var home = env.get("HOME");
    println("Home directory: {home}");
}
```

## Features

- **Shell-Native Design** - Execute commands, capture output, build pipelines
- **High-Level Readability** - Clean syntax with type annotations
- **Structured Control Flow** - Classes, interfaces, pattern matching
- **Rich Standard Library** - Filesystem, environment, processes, strings, collections
- **Interpreted Execution** - Fast iteration, no compilation step

## Installation

```bash
# Build and install
make install

# This will:
# - Build the fl interpreter
# - Install to /usr/local/bin (or $PREFIX/bin)
# - Create symlink to stdlib at ~/.forgelang/stdlib
```

### Uninstall

```bash
make uninstall
```

## Quick Start

### Run a ForgeLang script

```bash
fl script.fl
```

### Example: System Information

```forge
module sysinfo;

import [println] from std.io;
import std.env;
import std.proc;

fn main() {
    println("╔════════════════════════════════════════╗");
    println("║       System Information              ║");
    println("╚════════════════════════════════════════╝");
    
    // User and environment
    println("User: {env.get("USER")}");
    println("Shell: {env.get("SHELL")}");
    println("Home: {env.get("HOME")}");
    
    // System info via shell
    println("\nKernel:");
    println(proc.capture("uname -r"));
    
    // Count files in home
    var count = proc.capture("ls -1 | wc -l");
    println("Files in home: {count}");
}
```

### Example: File Processing Pipeline

```forge
module log_analyzer;

import [println] from std.io;
import std.fs;
import std.proc;
import std.list;

fn analyze_logs(log_path: str) {
    // Read log file
    var content = fs.read(log_path);
    var lines = content.split("\n");
    
    // Filter errors
    var errors = lines.filter(fn(line) {
        return line.contains("ERROR");
    });
    
    println("Found {errors.length()} errors");
    
    // Use shell for additional processing
    for error in errors {
        println("  {error}");
    }
}

fn main() {
    analyze_logs("/var/log/app.log");
}
```

### Example: Process Orchestration

```forge
module deploy;

import [println] from std.io;
import std.proc;
import std.fs;

fn deploy() -> int {
    println("Starting deployment...");
    
    // Run build
    var build_code = proc.exec("make build");
    if build_code != 0 {
        println("Build failed!");
        return build_code;
    }
    
    // Run tests
    println("Running tests...");
    var test_code = proc.exec("make test");
    if test_code != 0 {
        println("Tests failed!");
        return test_code;
    }
    
    // Deploy
    println("Deploying...");
    proc.exec("sudo systemctl restart myapp");
    
    println("Deployment complete!");
    return 0;
}

fn main() -> int {
    return deploy();
}
```

## Language Basics

### Variables and Types

```forge
// Immutable constants
const VERSION: str = "2.0";
const MAX_RETRIES: int = 3;

// Mutable variables
var count: int = 0;
count = count + 1;

// Type inference
var message = "Hello";  // str
var numbers = [1, 2, 3]; // list<int>
```

### Functions

```forge
fn greet(name: str) -> str {
    return "Hello, {name}!";
}

fn add(a: int, b: int) -> int {
    return a + b;
}
```

### Classes and Interfaces

```forge
interface Displayable {
    fn to_str(self) -> str;
}

class FileInfo implements Displayable {
    var path: str;
    var size: int;

    fn new(p: str, s: int) -> FileInfo {
        return FileInfo { path: p, size: s };
    }

    fn to_str(self) -> str {
        return "{self.path} ({self.size} bytes)";
    }
}
```

### Pattern Matching

```forge
enum Status { Ok, Err(msg: str) }

match get_status() {
    Status.Ok => println("Success!"),
    Status.Err(msg) => println("Error: {msg}")
}
```

### Shell Integration

```forge
import std.proc;

// Execute and get exit code
var code = proc.exec("git status");

// Capture output
var branch = proc.capture("git branch --show-current");

// Pipeline-style
var file_count = proc.capture("find . -name '*.fl' | wc -l");
```

## Standard Library

| Module | Description |
|--------|-------------|
| `std.io` | Input/output operations |
| `std.proc` | Process execution and command capture |
| `std.fs` | Filesystem operations |
| `std.env` | Environment variables |
| `std.str` | String manipulation |
| `std.list` | List operations and utilities |
| `std.math` | Mathematical functions |
| `std.time` | Time utilities |
| `std.int` | Integer utilities |
| `std.float` | Float utilities |
| `std.bool` | Boolean utilities |

## Project Structure

```
my_project/
├── src/
│   └── main.fl        # Entry point
└── README.md
```

## Why ForgeLang?

ForgeLang fills the gap between shell scripts and full programming languages:

- **More structured than Bash** - Types, classes, interfaces, modules
- **More system-aware than Python** - Shell integration is first-class
- **Faster iteration than Rust** - Interpreted, no compilation
- **Cleaner syntax than Go** - Expressive, minimal boilerplate

### Use Cases

- **System automation** - Replace complex shell scripts
- **DevOps tooling** - Deploy, monitor, orchestrate
- **CLI utilities** - Quick tools with clean structure
- **Process pipelines** - Chain commands with structured logic
- **Configuration management** - Environment-aware setups

## Documentation

- [Introduction](docs/docs/intro.md)
- [Quick Start](docs/docs/quick-start.md)
- [Language Guide](docs/docs/language-guide/)
- [Standard Library](docs/docs/stdlib/)
- [Examples](docs/docs/examples.md)

## License

MIT License - See [LICENSE](LICENSE) for details.

## Community

ForgeLang is open source and community-driven. Contributions welcome!

- Report issues on GitHub
- Contribute to the standard library
- Share your ForgeLang projects
