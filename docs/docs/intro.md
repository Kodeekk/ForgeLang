---
sidebar_position: 1
---

# Welcome to ForgeLang

<div className="hero">
  <h1 className="hero__title">ForgeLang</h1>
  <p className="hero__subtitle">
    A shell-native system orchestration language.
    Merge high-level expressiveness with deep shell integration.
  </p>
</div>

<div className="version-badge">
  <span>Latest Version: 2.0</span>
</div>

## What is ForgeLang?

ForgeLang is an interpreted programming language designed for **system orchestration and environment control**. It fills the gap between shell scripts and full programming languages—more structured than Bash, more system-aware than Python, and faster to iterate than Rust.

```forge
import [println] from std.io;
import std.proc;
import std.env;

fn main() {
    // Execute shell commands
    proc.exec("echo Building project...");
    
    // Capture output
    var branch = proc.capture("git branch --show-current");
    println("On branch: {branch}");
    
    // Access environment
    println("User: {env.get("USER")}");
}
```

### Key Features

<div className="feature-row">
  <div className="card feature-card">
    <div className="feature-card__icon">🐚</div>
    <h3 className="feature-card__title">Shell-Native</h3>
    <p className="feature-card__description">
      Execute commands, capture output, build pipelines. Shell integration is first-class, not an afterthought.
    </p>
  </div>

  <div className="card feature-card">
    <div className="feature-card__icon">📁</div>
    <h3 className="feature-card__title">System Aware</h3>
    <p className="feature-card__description">
      Filesystem operations, environment variables, process management. Deep OS integration built in.
    </p>
  </div>

  <div className="card feature-card">
    <div className="feature-card__icon">🏗️</div>
    <h3 className="feature-card__title">Structured Logic</h3>
    <p className="feature-card__description">
      Classes, interfaces, pattern matching. Write maintainable code that scales beyond shell scripts.
    </p>
  </div>

  <div className="card feature-card">
    <div className="feature-card__icon">📦</div>
    <h3 className="feature-card__title">Rich Stdlib</h3>
    <p className="feature-card__description">
      Process execution, filesystem, environment, strings, collections. Everything for system automation.
    </p>
  </div>

  <div className="card feature-card">
    <div className="feature-card__icon">⚡</div>
    <h3 className="feature-card__title">Interpreted</h3>
    <p className="feature-card__description">
      Fast iteration with no compilation step. Run scripts directly with `fl script.fl`.
    </p>
  </div>

  <div className="card feature-card">
    <div className="feature-card__icon">🔧</div>
    <h3 className="feature-card__title">DevOps Ready</h3>
    <p className="feature-card__description">
      Deploy scripts, automation tools, CLI utilities. Built for real-world system tasks.
    </p>
  </div>
</div>

## Quick Example

```forge
module deploy;

import [println] from std.io;
import std.proc;
import std.fs;
import std.env;

// Class for deployment tasks
class Deployer {
    var app_name: str;
    var deploy_dir: str;

    fn new(name: str, dir: str) -> Deployer {
        return Deployer { app_name: name, deploy_dir: dir };
    }

    fn build(self) -> int {
        println("Building {self.app_name}...");
        return proc.exec("make build");
    }

    fn test(self) -> int {
        println("Running tests...");
        return proc.exec("make test");
    }

    fn deploy(self) -> int {
        println("Deploying to {self.deploy_dir}...");
        proc.exec("sudo systemctl restart {self.app_name}");
        return 0;
    }
}

fn main() -> int {
    var deployer = Deployer.new("myapp", "/var/www");
    
    // Build
    if deployer.build() != 0 {
        println("Build failed!");
        return 1;
    }
    
    // Test
    if deployer.test() != 0 {
        println("Tests failed!");
        return 1;
    }
    
    // Deploy
    deployer.deploy();
    
    println("Deployment complete!");
    return 0;
}
```

## Use Cases

### System Automation
Replace complex shell scripts with structured, maintainable ForgeLang code.

### DevOps Tooling
Build deployment scripts, monitoring tools, and infrastructure automation.

### CLI Utilities
Create command-line tools with clean syntax and robust error handling.

### Process Orchestration
Chain commands, manage pipelines, and coordinate multiple processes.

### Configuration Management
Environment-aware setups with filesystem and variable manipulation.

## Getting Started

Ready to start building with ForgeLang?

1. **[Installation](./installation)** - Install ForgeLang on your system
2. **[Quick Start](./quick-start)** - Write your first ForgeLang program
3. **[Language Guide](./language-guide/overview)** - Learn the language features
4. **[Standard Library](./stdlib/overview)** - Explore available modules

## Community

ForgeLang is open source and community-driven. Join us!

- **[GitHub](https://github.com/forgelang/forgelang)** - View source code, report issues, contribute
- **[Discord](https://discord.gg/forgelang)** - Chat with other developers
- **[Twitter](https://twitter.com/forgelang)** - Follow for updates

## License

ForgeLang is released under the MIT License. See the [LICENSE](https://github.com/forgelang/forgelang/blob/main/LICENSE) file for details.
