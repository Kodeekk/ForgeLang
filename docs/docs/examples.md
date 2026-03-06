---
sidebar_position: 1
---

# Examples

Real-world examples demonstrating ForgeLang features.

## Hello World

```forge
module hello;

import [println] from std.io;

fn main() -> int {
    println("Hello, World!");
    return 0;
}
```

## Command Line Arguments

```forge
module cli_args;

import [println, args] from std.io;
import std.env;

fn main() -> int {
    var arguments = env.args();
    
    println("Program: {arguments[0]}");
    println("Arguments:");
    for arg in arguments.slice(1) {
        println("  - {arg}");
    }
    
    return 0;
}
```

## File Operations

```forge
module file_ops;

import [println] from std.io;
import std.fs;

fn main() -> int {
    var path = "example.txt";
    
    // Write to file
    fs.write(path, "Hello from ForgeLang!\nLine 2\nLine 3");
    
    // Read from file
    var content = fs.read(path);
    println("File contents:");
    println(content);
    
    // Append to file
    fs.append(path, "\nAppended line");
    
    // Check existence
    if fs.exists(path) {
        println("File exists!");
    }
    
    // Get file info
    println("Directory: {fs.dirname(path)}");
    println("Filename: {fs.basename(path)}");
    println("Extension: {fs.extension(path)}");
    
    return 0;
}
```

## Data Processing

```forge
module data_processing;

import [println] from std.io;
import std.math;
import std.list;

fn main() -> int {
    // Sample data
    var temperatures = [23.5, 25.1, 22.8, 27.3, 24.9, 26.2, 25.5];
    
    // Statistics
    var avg = list.sum(temperatures) / f64(temperatures.length());
    var max_temp = temperatures.reduce(0.0, fn(a, b) { return math.max(a, b); });
    var min_temp = temperatures.reduce(100.0, fn(a, b) { return math.min(a, b); });
    
    println("Temperature Statistics:");
    println("  Average: {avg}°C");
    println("  Maximum: {max_temp}°C");
    println("  Minimum: {min_temp}°C");
    
    // Filter hot days
    var hot_days = temperatures.filter(fn(t) { return t > 25.0; });
    println("  Hot days (>25°C): {hot_days.length()}");
    
    // Transform to Fahrenheit
    var fahrenheit = temperatures.map(fn(t) { return t * 9.0/5.0 + 32.0; });
    println("  In Fahrenheit: {fahrenheit}");
    
    return 0;
}
```

## Generic Data Structures

```forge
module generics;

import [println] from std.io;

// Generic Stack
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
        var top = self.items.last();
        self.items.pop();
        return Option.Some(top);
    }
    
    fn peek(self) -> Option<T> {
        if self.items.length() == 0 { return Option.None; }
        return Option.Some(self.items.last());
    }
    
    fn is_empty(self) -> bool {
        return self.items.length() == 0;
    }
}

// Generic Queue
class Queue<T> {
    var items: list<T>;
    
    fn new() -> Queue<T> {
        return Queue { items: [] };
    }
    
    fn enqueue(self, item: T) -> void {
        self.items.push(item);
    }
    
    fn dequeue(self) -> Option<T> {
        if self.items.length() == 0 { return Option.None; }
        var front = self.items.first();
        self.items.shift();
        return Option.Some(front);
    }
}

fn main() -> int {
    // Integer stack
    var int_stack: Stack<int> = Stack.new();
    int_stack.push(1);
    int_stack.push(2);
    int_stack.push(3);
    
    println("Stack operations:");
    while !int_stack.is_empty() {
        println("  Popped: {int_stack.pop()}");
    }
    
    // String queue
    var str_queue: Queue<str> = Queue.new();
    str_queue.enqueue("first");
    str_queue.enqueue("second");
    str_queue.enqueue("third");
    
    println("\nQueue operations:");
    while !str_queue.items.is_empty() {
        println("  Dequeued: {str_queue.dequeue()}");
    }
    
    return 0;
}
```

## Pattern Matching

```forge
module pattern_matching;

import [println] from std.io;

enum Message {
    Quit,
    Move { x: int, y: int },
    Write(str),
    ChangeColor(int, int, int)
}

enum Result<T, E> {
    Ok(value: T),
    Err(error: E)
}

fn handle_message(msg: Message) -> void {
    match msg {
        Message.Quit => println("Quitting..."),
        Message.Move { x, y } => println("Moving to ({x}, {y})"),
        Message.Write(text) => println("Writing: {text}"),
        Message.ChangeColor(r, g, b) => println("Color: RGB({r}, {g}, {b})")
    }
}

fn safe_divide(a: f64, b: f64) -> Result<f64, str> {
    if b == 0.0 {
        return Result.Err("Division by zero");
    }
    return Result.Ok(a / b);
}

fn main() -> int {
    // Enum matching
    var messages = [
        Message.Quit,
        Message.Move { x: 10, y: 20 },
        Message.Write("Hello"),
        Message.ChangeColor(255, 128, 0)
    ];
    
    for msg in messages {
        handle_message(msg);
    }
    
    // Result matching
    println("\nDivision results:");
    match safe_divide(10.0, 2.0) {
        Result.Ok(v) => println("  10/2 = {v}"),
        Result.Err(e) => println("  Error: {e}")
    }
    
    match safe_divide(10.0, 0.0) {
        Result.Ok(v) => println("  10/0 = {v}"),
        Result.Err(e) => println("  Error: {e}")
    }
    
    return 0;
}
```

## HTTP-like API Client

```forge
module api_client;

import [println] from std.io;
import std.fs;
import std.time;

class HttpClient {
    var base_url: str;
    var timeout_ms: int;
    
    fn new(base_url: str) -> HttpClient {
        return HttpClient {
            base_url: base_url,
            timeout_ms: 5000
        };
    }
    
    fn get(self, endpoint: str) -> Result<str, str> {
        var url = "{self.base_url}/{endpoint}";
        println("GET {url}");
        
        // Simulate network delay
        std.time.sleep(100);
        
        // In real implementation, this would make actual HTTP request
        return Result.Ok("{\"status\": \"ok\"}");
    }
    
    fn post(self, endpoint: str, data: str) -> Result<str, str> {
        var url = "{self.base_url}/{endpoint}";
        println("POST {url}");
        println("Body: {data}");
        
        std.time.sleep(100);
        return Result.Ok("{\"id\": 123}");
    }
}

fn main() -> int {
    var client = HttpClient.new("https://api.example.com");
    
    match client.get("users") {
        Result.Ok(response) => println("Response: {response}"),
        Result.Err(e) => println("Error: {e}")
    }
    
    match client.post("users", "{\"name\": \"Alice\"}") {
        Result.Ok(response) => println("Response: {response}"),
        Result.Err(e) => println("Error: {e}")
    }
    
    return 0;
}
```

## Game Example: Tic-Tac-Toe

```forge
module tictactoe;

import [println, read_line] from std.io;
import std.math;

enum Player { X, O }
enum Cell { Empty, X, O }

class Board {
    var cells: list<list<Cell>>;
    
    fn new() -> Board {
        return Board {
            cells: [
                [Cell.Empty, Cell.Empty, Cell.Empty],
                [Cell.Empty, Cell.Empty, Cell.Empty],
                [Cell.Empty, Cell.Empty, Cell.Empty]
            ]
        };
    }
    
    fn place(self, row: int, col: int, player: Player) -> bool {
        if self.cells[row][col] != Cell.Empty {
            return false;
        }
        self.cells[row][col] = match player {
            Player.X => Cell.X,
            Player.O => Cell.O
        };
        return true;
    }
    
    fn check_winner(self) -> Option<Player> {
        // Check rows
        for row in self.cells {
            if row[0] != Cell.Empty && row[0] == row[1] && row[1] == row[2] {
                return match row[0] {
                    Cell.X => Option.Some(Player.X),
                    Cell.O => Option.Some(Player.O),
                    Cell.Empty => Option.None
                };
            }
        }
        
        // Check columns
        for col in list.range(0, 3) {
            if self.cells[0][col] != Cell.Empty && 
               self.cells[0][col] == self.cells[1][col] && 
               self.cells[1][col] == self.cells[2][col] {
                return match self.cells[0][col] {
                    Cell.X => Option.Some(Player.X),
                    Cell.O => Option.Some(Player.O),
                    Cell.Empty => Option.None
                };
            }
        }
        
        // Check diagonals
        if self.cells[0][0] != Cell.Empty && 
           self.cells[0][0] == self.cells[1][1] && 
           self.cells[1][1] == self.cells[2][2] {
            return match self.cells[0][0] {
                Cell.X => Option.Some(Player.X),
                Cell.O => Option.Some(Player.O),
                Cell.Empty => Option.None
            };
        }
        
        return Option.None;
    }
    
    fn is_full(self) -> bool {
        for row in self.cells {
            for cell in row {
                if cell == Cell.Empty {
                    return false;
                }
            }
        }
        return true;
    }
    
    fn display(self) -> void {
        println("\n  0 1 2");
        for i in list.range(0, 3) {
            var row_str = "{i} ";
            for j in list.range(0, 3) {
                row_str = row_str + match self.cells[i][j] {
                    Cell.Empty => ".",
                    Cell.X => "X",
                    Cell.O => "O"
                } + " ";
            }
            println(row_str);
        }
    }
}

fn main() -> int {
    var board = Board.new();
    var current_player = Player.X;
    
    println("Tic-Tac-Toe!");
    println("Enter moves as 'row col' (e.g., '1 0')");
    
    while true {
        board.display();
        
        println("\nPlayer {match current_player { Player.X => "X", Player.O => "O" }}'s turn");
        var input = read_line();
        var parts = input.split(" ");
        
        if parts.length() != 2 {
            println("Invalid input! Use 'row col' format");
            continue;
        }
        
        var row = int(parts[0]);
        var col = int(parts[1]);
        
        if row < 0 || row > 2 || col < 0 || col > 2 {
            println("Invalid position!");
            continue;
        }
        
        if !board.place(row, col, current_player) {
            println("Cell already taken!");
            continue;
        }
        
        match board.check_winner() {
            Option.Some(winner) => {
                board.display();
                println("\nPlayer {match winner { Player.X => "X", Player.O => "O" }} wins!");
                break;
            },
            Option.None => {}
        }
        
        if board.is_full() {
            board.display();
            println("\nIt's a draw!");
            break;
        }
        
        current_player = match current_player {
            Player.X => Player.O,
            Player.O => Player.X
        };
    }
    
    return 0;
}
```

## More Examples

Explore more examples in the ForgeLang repository:

- **showcase_v2.fl** - Comprehensive feature demonstration
- **test_tuples.fl** - Tuple operations
- **stdlib/** - Standard library implementations
