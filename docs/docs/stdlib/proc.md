---
sidebar_position: 7
---

# std.proc

Process execution and shell command integration.

The `std.proc` module provides ForgeLang's core shell integration capabilities, allowing you to execute system commands and capture their output.

## Overview

```forge
import std.proc;
import [println] from std.io;

fn main() {
    // Execute a command
    var exit_code = proc.exec("echo Hello");
    
    // Capture output
    var output = proc.capture("ls -1");
    println("Files: {output}");
}
```

## Functions

### `exec(command: str) -> int`

Execute a shell command and return the exit code.

The command is run through the system shell (`sh -c`), so you can use pipes, redirection, and other shell features. Output is printed directly to stdout/stderr.

**Parameters:**
- `command` - The shell command to execute

**Returns:**
- Exit code of the command (0 for success)

**Example:**

```forge
import std.proc;

fn main() -> int {
    // Simple command
    var code = proc.exec("echo Hello World");
    
    // Command with pipe
    var code = proc.exec("ls -la | grep .fl");
    
    // Check exit code
    if proc.exec("make build") != 0 {
        println("Build failed!");
        return 1;
    }
    
    return 0;
}
```

**Use Cases:**
- Running build commands
- Starting/stopping services
- Executing git commands
- Running test suites

---

### `capture(command: str) -> str`

Execute a shell command and capture its stdout output.

The command is run through the system shell (`sh -c`). The stdout is captured and returned as a string with trailing newlines stripped.

**Parameters:**
- `command` - The shell command to execute

**Returns:**
- The stdout output as a string

**Example:**

```forge
import std.proc;
import [println] from std.io;

fn main() {
    // Get current branch
    var branch = proc.capture("git branch --show-current");
    println("On branch: {branch}");
    
    // Count files
    var count = proc.capture("find . -name '*.fl' | wc -l");
    println("Forge files: {count}");
    
    // Get system info
    var kernel = proc.capture("uname -r");
    println("Kernel: {kernel}");
    
    // Disk usage
    var usage = proc.capture("df -h / | tail -1 | awk '{print $5}'");
    println("Disk usage: {usage}");
}
```

**Use Cases:**
- Getting git information
- Querying system state
- Counting files or lines
- Extracting configuration values

---

## Patterns

### Command Pipeline

Chain multiple commands together using shell pipes:

```forge
import std.proc;

fn main() {
    // Count specific files
    var count = proc.capture("find . -type f -name '*.log' | wc -l");
    
    // Get top processes
    var top = proc.capture("ps aux --sort=-%mem | head -10");
    
    // Search and count
    var errors = proc.capture("grep -r ERROR logs/ | wc -l");
}
```

### Conditional Execution

Run commands based on conditions:

```forge
import std.proc;
import [println] from std.io;

fn deploy() -> int {
    // Build first
    if proc.exec("make build") != 0 {
        println("Build failed, aborting deploy");
        return 1;
    }
    
    // Then test
    if proc.exec("make test") != 0 {
        println("Tests failed, aborting deploy");
        return 1;
    }
    
    // Finally deploy
    proc.exec("sudo systemctl restart myapp");
    
    return 0;
}
```

### Output Processing

Process captured output:

```forge
import std.proc;
import [println] from std.io;

fn list_forge_files() {
    var output = proc.capture("ls -1");
    var files = output.split("\n");
    
    for file in files {
        if file.ends_with(".fl") {
            println("Found: {file}");
        }
    }
}
```

### Environment Query

Gather system information:

```forge
import std.proc;
import std.env;
import [println] from std.io;

fn system_report() {
    println("=== System Report ===\n");
    
    // User info
    println("User: {env.get("USER")}");
    println("Home: {env.get("HOME")}");
    
    // System info
    println("Hostname: {proc.capture("hostname")}");
    println("Kernel: {proc.capture("uname -r")}");
    println("Uptime: {proc.capture("uptime -p")}");
    
    // Disk
    println("Disk: {proc.capture("df -h / | tail -1 | awk '{print $5}'")}");
    
    // Memory
    println("Memory: {proc.capture("free -h | grep Mem | awk '{print $3 "/" $2}'")}");
}
```

## Best Practices

### 1. Check Exit Codes

Always check the exit code when executing important commands:

```forge
if proc.exec("make build") != 0 {
    println("Build failed!");
    return 1;
}
```

### 2. Sanitize Inputs

Be careful with user input in commands:

```forge
// Good - validate input
fn safe_query(table: str) -> str {
    // Only allow alphanumeric table names
    if !table.matches("^[a-z]+$") {
        return "Invalid table name";
    }
    return proc.capture("sqlite3 db.sqlite '.schema {table}'");
}
```

### 3. Handle Errors Gracefully

```forge
fn get_git_info() {
    var branch = proc.capture("git branch --show-current");
    
    if branch == "" {
        println("Not a git repository or git not installed");
        return;
    }
    
    println("Branch: {branch}");
}
```

### 4. Use for System Tasks

ForgeLang's `proc` module shines for system automation:

```forge
// Deployment
proc.exec("sudo systemctl restart nginx");

// Monitoring
var load = proc.capture("uptime | awk -F'load average:' '{print $2}'");

// Backup
proc.exec("tar -czf backup.tar.gz /data");
```

## See Also

- [`std.env`](./env) - Environment variable access
- [`std.fs`](./fs) - Filesystem operations
- [`std.io`](./io) - Input/output operations
