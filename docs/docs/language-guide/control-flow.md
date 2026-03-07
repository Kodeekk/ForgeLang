---
sidebar_position: 6
---

# Control Flow

Control the execution flow of your programs with conditionals, loops, and pattern matching.

## If Expressions

Conditional execution with if/else:

```forge
// Basic if
var x: int = 10;
if x > 0 {
    println("Positive");
}

// If/else
if x % 2 == 0 {
    println("Even");
} else {
    println("Odd");
}

// If/else-if/else
var score: int = 85;
if score >= 90 {
    println("A");
} else if score >= 80 {
    println("B");
} else if score >= 70 {
    println("C");
} else {
    println("D");
}

// If as expression (returns value)
var result = if x > 0 { "positive" } else { "negative" };

// Single-line if
if x > 0 { println("Positive"); }
```

## Match Expressions

Pattern matching for complex branching:

```forge
// Basic match
enum Color { Red, Green, Blue }
var c = Color.Red;

match c {
    Color.Red => println("Red!"),
    Color.Green => println("Green!"),
    Color.Blue => println("Blue!")
}

// Match with values
enum Shape {
    Circle(radius: f64),
    Rect(width: f64, height: f64)
}

fn area(s: Shape) -> f64 {
    return match s {
        Shape.Circle(r) => math.PI * r * r,
        Shape.Rect(w, h) => w * h
    };
}

// Match with guards
enum Status { Ok, Err(code: int) }
var status = Status.Err(404);

match status {
    Status.Ok => println("Success"),
    Status.Err(code) if code == 404 => println("Not Found"),
    Status.Err(code) if code >= 500 => println("Server Error"),
    Status.Err(code) => println("Error: {code}")
}

// Match as expression
var message = match status {
    Status.Ok => "All good",
    Status.Err(_) => "Something went wrong"
};

// Wildcard pattern
var code: int = 404;
match code {
    200 => println("OK"),
    404 => println("Not Found"),
    500 => println("Server Error"),
    _ => println("Other: {code}")  // Catch-all
}
```

## For Loops

Iterate over collections:

```forge
// Iterate over list
var items = ["apple", "banana", "cherry"];
for item in items {
    println(item);
}

// With index using enumerate
for (i, item) in items.enumerate() {
    println("{i}: {item}");
}

// Range-based
for i in list.range(0, 5) {
    println(i);  // 0, 1, 2, 3, 4
}

// Reverse iteration
for i in list.range(0, 5).reverse() {
    println(i);  // 4, 3, 2, 1, 0
}

// Destructuring in for
var pairs = [("a", 1), ("b", 2), ("c", 3)];
for (key, value) in pairs {
    println("{key} = {value}");
}

// Filtering with if
for i in list.range(0, 10) {
    if i % 2 == 0 {
        println(i);  // Only even numbers
    }
}
```

## While Loops

Loop while condition is true:

```forge
// Basic while
var count = 0;
while count < 5 {
    println(count);
    count = count + 1;
}

// Infinite loop with break
var n = 0;
while true {
    if n >= 5 { break; }
    println(n);
    n = n + 1;
}

// While with condition
var input: str = "";
while input != "quit" {
    input = read_line();
    println("You said: {input}");
}
```

## Loop Control

Control loop execution:

```forge
// Break - exit loop
for i in list.range(0, 10) {
    if i == 5 { break; }
    println(i);  // 0, 1, 2, 3, 4
}

// Continue - skip to next iteration
for i in list.range(0, 10) {
    if i % 2 == 0 { continue; }
    println(i);  // 1, 3, 5, 7, 9
}

// Nested loops with labeled break
'outer: for i in list.range(0, 3) {
    for j in list.range(0, 3) {
        if i * j == 4 {
            break 'outer;  // Break outer loop
        }
    }
}
```

## Pattern Matching with Enums

Exhaustive matching on enums:

```forge
enum Message {
    Quit,
    Move { x: int, y: int },
    Write(str),
    ChangeColor(int, int, int)
}

fn handle_message(msg: Message) -> void {
    match msg {
        Message.Quit => println("Quitting..."),
        Message.Move { x, y } => println("Moving to ({x}, {y})"),
        Message.Write(text) => println("Writing: {text}"),
        Message.ChangeColor(r, g, b) => println("Color: RGB({r}, {g}, {b})")
    }
}

// Nested pattern matching
enum Result<T, E> {
    Ok(value: T),
    Err(error: E)
}

fn process(r: Result<Option<int>, str>) -> void {
    match r {
        Result.Ok(Option.Some(n)) => println("Got: {n}"),
        Result.Ok(Option.None) => println("Nothing"),
        Result.Err(e) => println("Error: {e}")
    }
}
```

## Option Pattern Matching

Handle nullable values:

```forge
fn find_user(id: int) -> Option<User> {
    // ...
}

// Match on Option
match find_user(42) {
    Option.Some(user) => println("Found: {user.name}"),
    Option.None => println("User not found")
}

// if let (shorthand)
if let Option.Some(user) = find_user(42) {
    println("Found: {user.name}");
}

// while let
var stack: Stack<int> = Stack.new();
while let Option.Some(value) = stack.pop() {
    println("Popped: {value}");
}
```

## Result Pattern Matching

Error handling with match:

```forge
fn read_file(path: str) -> Result<str, str> {
    // ...
}

// Match on Result
match read_file("config.txt") {
    Result.Ok(content) => println("Config: {content}"),
    Result.Err(e) => println("Failed to read: {e}")
}

// Propagate errors
fn load_config() -> Result<Config, str> {
    var content = match read_file("config.txt") {
        Result.Ok(c) => c,
        Result.Err(e) => return Result.Err(e),
    };
    // ...
}
```

## Early Returns

Return early from functions:

```forge
fn find_positive(numbers: list<int>) -> Option<int> {
    for n in numbers {
        if n > 0 {
            return Option.Some(n);  // Early return
        }
    }
    return Option.None;
}

fn validate_user(user: User) -> Result<(), str> {
    if user.name.length() == 0 {
        return Result.Err("Name required");
    }
    if user.age < 0 {
        return Result.Err("Invalid age");
    }
    if !user.email.contains("@") {
        return Result.Err("Invalid email");
    }
    return Result.Ok(());
}
```

## Guard Clauses

Early exits for cleaner code:

```forge
// Without guard clauses
fn process_data(data: Option<Data>) -> Result<(), str> {
    if data != Option.None {
        var d = data.unwrap();
        if d.is_valid {
            if d.value > 0 {
                // Actual logic
            } else {
                return Result.Err("Value must be positive");
            }
        } else {
            return Result.Err("Invalid data");
        }
    } else {
        return Result.Err("No data");
    }
}

// With guard clauses (cleaner)
fn process_data(data: Option<Data>) -> Result<(), str> {
    var d = match data {
        Option.Some(d) => d,
        Option.None => return Result.Err("No data")
    };
    
    if !d.is_valid { return Result.Err("Invalid data"); }
    if d.value <= 0 { return Result.Err("Value must be positive"); }
    
    // Actual logic here
}
```

## Next Steps

- **[Functions](./functions)** - Define reusable code
- **[Pattern Matching](../advanced/pattern-matching)** - Advanced patterns
- **[Error Handling](../advanced/error-handling)** - Result and Option
