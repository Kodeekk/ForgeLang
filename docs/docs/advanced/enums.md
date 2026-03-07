---
sidebar_position: 2
---

# Enums

Enumerations define types with a fixed set of variants.

## Basic Enums

Simple enumeration:

```forge
enum Direction {
    North,
    South,
    East,
    West
}

enum Color {
    Red,
    Green,
    Blue
}

// Usage
var dir: Direction = Direction.North;
var color: Color = Color.Red;
```

## Enums with Data

Variants can carry associated data:

```forge
enum Message {
    Quit,
    Move { x: int, y: int },
    Write(str),
    ChangeColor(int, int, int)
}

// Creating variants
var quit = Message.Quit;
var move = Message.Move { x: 10, y: 20 };
var write = Message.Write("Hello");
var color = Message.ChangeColor(255, 128, 0);
```

## Pattern Matching

Exhaustive matching on enums:

```forge
fn handle_message(msg: Message) -> void {
    match msg {
        Message.Quit => println("Quitting..."),
        Message.Move { x, y } => println("Moving to ({x}, {y})"),
        Message.Write(text) => println("Writing: {text}"),
        Message.ChangeColor(r, g, b) => println("RGB({r}, {g}, {b})")
    }
}

// Match with guards
enum Status {
    Ok,
    Err(code: int)
}

fn status_message(status: Status) -> str {
    return match status {
        Status.Ok => "Success",
        Status.Err(code) if code == 404 => "Not Found",
        Status.Err(code) if code >= 500 => "Server Error",
        Status.Err(code) => "Error: {code}"
    };
}
```

## Option Enum

Handle nullable values:

```forge
enum Option<T> {
    Some(value: T),
    None
}

// Usage
fn find_user(id: int) -> Option<User> {
    // ...
}

match find_user(42) {
    Option.Some(user) => println("Found: {user.name}"),
    Option.None => println("User not found")
}

// Helper methods
var user = find_user(42).unwrap_or(default_user());
var has_user = find_user(42).is_some();
var no_user = find_user(42).is_none();
```

## Result Enum

Error handling:

```forge
enum Result<T, E> {
    Ok(value: T),
    Err(error: E)
}

// Usage
fn read_file(path: str) -> Result<str, str> {
    // ...
}

match read_file("config.txt") {
    Result.Ok(content) => println("Config: {content}"),
    Result.Err(e) => println("Error: {e}")
}

// Helper functions
fn unwrap_or<T>(result: Result<T, str>, fallback: T) -> T {
    return match result {
        Result.Ok(v) => v,
        Result.Err(_) => fallback
    };
}

fn map_result<T, U>(result: Result<T, str>, f: fn(T) -> U) -> Result<U, str> {
    return match result {
        Result.Ok(v) => Result.Ok(f(v)),
        Result.Err(e) => Result.Err(e)
    };
}

fn and_then<T, U>(result: Result<T, str>, f: fn(T) -> Result<U, str>) -> Result<U, str> {
    return match result {
        Result.Ok(v) => f(v),
        Result.Err(e) => Result.Err(e)
    };
}
```

## Methods on Enums

Implement methods for enums:

```forge
enum Shape {
    Circle(radius: f64),
    Rect(width: f64, height: f64),
    Triangle(base: f64, height: f64)
}

implement Shape {
    fn area(self) -> f64 {
        return match self {
            Shape.Circle(r) => math.PI * r * r,
            Shape.Rect(w, h) => w * h,
            Shape.Triangle(b, h) => 0.5 * b * h
        };
    }
    
    fn perimeter(self) -> f64 {
        return match self {
            Shape.Circle(r) => 2.0 * math.PI * r,
            Shape.Rect(w, h) => 2.0 * (w + h),
            Shape.Triangle(b, h) => b + 2.0 * math.sqrt((b/2.0)^2.0 + h^2.0)
        };
    }
    
    fn name(self) -> str {
        return match self {
            Shape.Circle(_) => "Circle",
            Shape.Rect(_, _) => "Rectangle",
            Shape.Triangle(_, _) => "Triangle"
        };
    }
}

// Usage
var circle = Shape.Circle(5.0);
println("Area: {circle.area()}");
println("Name: {circle.name()}");
```

## Enum with Interfaces

Implement interfaces for enums:

```forge
interface Displayable {
    fn to_str(self) -> str;
}

enum LogLevel {
    Debug,
    Info,
    Warn,
    Error
}

implement Displayable for LogLevel {
    fn to_str(self) -> str {
        return match self {
            LogLevel.Debug => "[DEBUG]",
            LogLevel.Info => "[INFO] ",
            LogLevel.Warn => "[WARN] ",
            LogLevel.Error => "[ERROR]"
        };
    }
}

// Usage
var level = LogLevel.Warn;
println(level.to_str());  // "[WARN] "
```

## Recursive Enums

Enums that contain themselves:

```forge
// Linked list
enum List<T> {
    Cons(T, Box<List<T>>),
    Nil
}

fn length<T>(list: List<T>) -> int {
    return match list {
        List.Nil => 0,
        List.Cons(_, rest) => 1 + length(rest)
    };
}

// Tree
enum Tree<T> {
    Leaf(T),
    Node(Tree<T>, Tree<T>)
}

fn sum_tree(tree: Tree<int>) -> int {
    return match tree {
        Tree.Leaf(n) => n,
        Tree.Node(left, right) => sum_tree(left) + sum_tree(right)
    };
}
```

## Converting to/from Enums

Type conversions:

```forge
enum HttpStatus {
    Ok = 200,
    NotFound = 404,
    ServerError = 500
}

// From int
fn status_from_code(code: int) -> Option<HttpStatus> {
    return match code {
        200 => Option.Some(HttpStatus.Ok),
        404 => Option.Some(HttpStatus.NotFound),
        500 => Option.Some(HttpStatus.ServerError),
        _ => Option.None
    };
}

// To int
fn status_to_code(status: HttpStatus) -> int {
    return match status {
        HttpStatus.Ok => 200,
        HttpStatus.NotFound => 404,
        HttpStatus.ServerError => 500
    };
}
```

## Best Practices

1. **Use descriptive variant names** - Clear and specific
2. **Include relevant data** - Don't make separate types
3. **Always match exhaustively** - Handle all cases
4. **Use Option/Result** - For nullable and error cases

```forge
// Good: All relevant data in enum
enum Command {
    Move { x: int, y: int },
    Attack(target: Enemy),
    UseItem(item: Item, target: Character)
}

// Better than separate types
class MoveCommand { var x: int; var y: int; }
class AttackCommand { var target: Enemy; }
// ...
```

## Next Steps

- **[Pattern Matching](./pattern-matching)** - Deep dive into matching
- **[Error Handling](./error-handling)** - Using Result effectively
- **[Classes](./classes)** - Compare with classes
