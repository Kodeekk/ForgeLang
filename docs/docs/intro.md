---
sidebar_position: 1
---

# Welcome to ForgeLang

<div className="hero">
  <h1 className="hero__title">ForgeLang</h1>
  <p className="hero__subtitle">
    A dynamically-typed interpreted language with a focus on simplicity and expressiveness.
    Build robust applications with elegant syntax and powerful features.
  </p>
</div>

<div className="version-badge">
  <span>Latest Version: 2.0.0</span>
</div>

## What is ForgeLang?

ForgeLang is a modern, dynamically-typed programming language designed for developers who value both simplicity and power. Whether you're building quick scripts, complex applications, or anything in between, ForgeLang provides the tools you need with an intuitive, expressive syntax.

### Key Features

<div className="feature-row">
  <div className="card feature-card">
    <div className="feature-card__icon">🦺</div>
    <h3 className="feature-card__title">Type Safety</h3>
    <p className="feature-card__description">
      Dynamic typing with optional type annotations. Get flexibility when you need it, safety when you want it.
    </p>
  </div>
  
  <div className="card feature-card">
    <div className="feature-card__icon">🧬</div>
    <h3 className="feature-card__title">Generics</h3>
    <p className="feature-card__description">
      Write reusable, type-safe code with powerful generic types and constraints.
    </p>
  </div>
  
  <div className="card feature-card">
    <div className="feature-card__icon">🎯</div>
    <h3 className="feature-card__title">Pattern Matching</h3>
    <p className="feature-card__description">
      Exhaustive pattern matching on enums and data structures for clear, bug-free code.
    </p>
  </div>
  
  <div className="card feature-card">
    <div className="feature-card__icon">📦</div>
    <h3 className="feature-card__title">Rich Stdlib</h3>
    <p className="feature-card__description">
      Comprehensive standard library with I/O, collections, math, filesystem, and more.
    </p>
  </div>
  
  <div className="card feature-card">
    <div className="feature-card__icon">🏗️</div>
    <h3 className="feature-card__title">Classes & Interfaces</h3>
    <p className="feature-card__description">
      Object-oriented features with classes, interfaces, and implementation inheritance.
    </p>
  </div>
  
  <div className="card feature-card">
    <div className="feature-card__icon">⚡</div>
    <h3 className="feature-card__title">Fast Development</h3>
    <p className="feature-card__description">
      Quick iteration with an interpreted runtime and helpful error messages.
    </p>
  </div>
</div>

## Quick Example

```forge
module hello;

import [println] from std.io;
import std.math;

// Define an enum with associated data
enum Shape {
    Circle(radius: f64),
    Rect(width: f64, height: f64)
}

// Pattern matching with exhaustive cases
fn area(s: Shape) -> f64 {
    return match s {
        Shape.Circle(r) => math.PI * r * r,
        Shape.Rect(w, h) => w * h
    };
}

// Generic class with interface implementation
class Stack<T> {
    var items: list<T>;
    
    fn new() -> Stack<T> {
        return Stack { items: [] };
    }
    
    fn push(self, item: T) -> void {
        self.items.push(item);
    }
    
    fn pop(self) -> Option<T> {
        if self.items.length() == 0 { return Option.None; }
        var top: T = self.items.last();
        self.items.pop();
        return Option.Some(top);
    }
}

fn main() -> int {
    var shapes = [
        Shape.Circle(5.0),
        Shape.Rect(4.0, 6.0)
    ];
    
    for shape in shapes {
        println("Area: {area(shape)}");
    }
    
    // Using generics
    var stack: Stack<int> = Stack.new();
    stack.push(1);
    stack.push(2);
    stack.push(3);
    
    while !stack.items.is_empty() {
        println("Popped: {stack.pop()}");
    }
    
    return 0;
}
```

## Getting Started

Ready to start building with ForgeLang? Here's how to get up and running:

1. **[Installation](./installation)** - Install ForgeLang on your system
2. **[Quick Start](./quick-start)** - Write your first ForgeLang program
3. **[Language Guide](./language-guide/overview)** - Learn the language features

## Community

ForgeLang is open source and community-driven. Join us!

- **[GitHub](https://github.com/forgelang/forgelang)** - View source code, report issues, contribute
- **[Discord](https://discord.gg/forgelang)** - Chat with other developers
- **[Twitter](https://twitter.com/forgelang)** - Follow for updates

## License

ForgeLang is released under the MIT License. See the [LICENSE](https://github.com/forgelang/forgelang/blob/main/LICENSE) file for details.
