---
sidebar_position: 3
---

# Quick Start

Get up and running with ForgeLang in minutes. This guide will walk you through writing and running your first system automation scripts.

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

## Shell Integration Basics

ForgeLang's power comes from its seamless shell integration.

### Execute Commands

```forge
module sysinfo;

import [println] from std.io;
import std.proc;

fn main() {
    // Run a command, get exit code
    var exit_code = proc.exec("echo Hello from shell!");
    println("Exit code: {exit_code}");
}
```

### Capture Output

```forge
module filecount;

import [println] from std.io;
import std.proc;

fn main() {
    // Capture command output
    var count = proc.capture("ls -1 | wc -l");
    println("Files in directory: {count}");
}
```

### Environment Variables

```forge
module userinfo;

import [println] from std.io;
import std.env;

fn main() {
    var user = env.get("USER");
    var home = env.get("HOME");
    var shell = env.get("SHELL");
    
    println("User: {user}");
    println("Home: {home}");
    println("Shell: {shell}");
}
```

## Language Basics

### Variables

```forge
// Immutable constants
const VERSION: str = "2.0";
const MAX_RETRIES: int = 3;

// Mutable variables
var count: int = 0;
count = count + 1;

// Type inference
var message = "Hello";  // Inferred as str
var numbers = [1, 2, 3]; // Inferred as list<int>
```

### Functions

```forge
// Simple function
fn greet(name: str) -> str {
    return "Hello, {name}!";
}

// Function with shell command
fn get_git_branch() -> str {
    return proc.capture("git branch --show-current");
}

// Function returning exit code
fn run_build() -> int {
    return proc.exec("make build");
}
```

### Control Flow

```forge
// If/else
var exit_code = proc.exec("make test");
if exit_code != 0 {
    println("Tests failed!");
    return 1;
}

// Match expression
enum Status { Ok, Err(msg: str) }

match get_status() {
    Status.Ok => println("Success!"),
    Status.Err(msg) => println("Error: {msg}")
}

// For loop
var files = proc.capture("ls -1").split("\n");
for file in files {
    println("File: {file}");
}

// While loop
var retries: int = 0;
while retries < 3 {
    if proc.exec("curl -s http://api.example.com/health") == 0 {
        break;
    }
    retries = retries + 1;
}
```

### String Interpolation

```forge
var name = "myapp";
var port = 8080;

// String interpolation with {}
println("Starting {name} on port {port}");

// Expressions in strings
println("Next port will be {port + 1}");
// Lists
var numbers: list<int> = [1, 2, 3, 4, 5];
numbers.push(6);
var first = numbers[0];

// Maps (using object literals)
var config = { name: "myapp", version: "1.0" };
println(config.name);  // "myapp"
```

## Practical Examples

### Example 1: Log Analyzer

```forge
module log_analyzer;

import [println] from std.io;
import std.fs;

fn analyze_logs(path: str) {
    var content = fs.read(path);
    var lines = content.split("\n");
    
    var error_count = 0;
    for line in lines {
        if line.contains("ERROR") {
            error_count = error_count + 1;
            println("  {line}");
        }
    }
    
    println("Total errors: {error_count}");
}

fn main() {
    analyze_logs("/var/log/app.log");
}
```

### Example 2: Deployment Script

```forge
module deploy;

import [println] from std.io;
import std.proc;

fn deploy() -> int {
    println("Starting deployment...");
    
    // Build
    println("Building...");
    if proc.exec("make build") != 0 {
        println("Build failed!");
        return 1;
    }
    
    // Test
    println("Testing...");
    if proc.exec("make test") != 0 {
        println("Tests failed!");
        return 1;
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

### Example 3: System Monitor

```forge
module monitor;

import [println] from std.io;
import std.proc;
import std.env;

fn check_disk_usage(threshold: int) {
    var usage_str = proc.capture("df -h / | tail -1 | awk '{print $5}'");
    var usage = usage_str.replace("%", "").to_int();
    
    if usage > threshold {
        println("WARNING: Disk at {usage}%");
    } else {
        println("Disk OK: {usage}%");
    }
}

fn check_services() {
    var services = ["nginx", "postgres", "redis"];
    
    for service in services {
        var status = proc.capture("systemctl is-active {service}");
        if status == "active" {
            println("{service}: running");
        } else {
            println("{service}: DOWN!");
        }
    }
}

fn main() {
    check_disk_usage(80);
    check_services();
}
```

### Example 4: Git Utilities

```forge
module git_utils;

import [println] from std.io;
import std.proc;
import std.env;

fn git_status() {
    println("=== Git Status ===");
    println(proc.capture("git status"));
}

fn git_branch_info() {
    var branch = proc.capture("git branch --show-current");
    var commits = proc.capture("git rev-list --count HEAD");
    
    println("Branch: {branch}");
    println("Commits: {commits}");
}

fn git_recent() {
    println("=== Recent Commits ===");
    println(proc.capture("git log --oneline -5"));
}

fn main() {
    git_status();
    git_branch_info();
    git_recent();
}
```

## Standard Library Overview

ForgeLang includes a rich standard library for system tasks:

```forge
import [println] from std.io;
import std.proc;
import std.fs;
import std.env;
import std.str;
import std.list;
import std.math;

fn main() -> int {
    // Process execution
    proc.exec("echo Hello");
    var output = proc.capture("ls -1");
    
    // Filesystem
    var content = fs.read("file.txt");
    fs.write("output.txt", content);
    var exists = fs.exists("config.yaml");
    
    // Environment
    var home = env.get("HOME");
    var cwd = env.cwd();
    
    // String operations
    var trimmed = "  hello  ".trim();
    var upper = "hello".upper();
    
    // List operations
    var nums = [1, 2, 3, 4, 5];
    var sum = list.sum(nums);
    
    // Math
    var result = math.sqrt(144);
    
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
- **[Standard Library](./stdlib/overview)** - Explore the stdlib modules
- **[Examples](./examples)** - Real-world automation scripts

## Try It Yourself

Experiment with ForgeLang by creating automation scripts:

```bash
# Create a test file
echo 'println(proc.capture("uname -a"))' > test.fl

# Run it
fl test.fl
```

Happy automating! 🚀
