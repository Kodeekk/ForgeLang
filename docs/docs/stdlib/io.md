---
sidebar_position: 2
---

# std.io

Input/Output operations for console and streams.

## Functions

### `print(value)`

Print a value to stdout without a trailing newline.

**Parameters:**
- `value` - Any value to print

**Examples:**
```forge
import [print] from std.io;
print("Hello");
print(" World");
// Output: Hello World
```

---

### `println(value)`

Print a value to stdout with a trailing newline.

**Parameters:**
- `value` - Any value to print

**Examples:**
```forge
import [println] from std.io;
println("Hello, ForgeLang!");
// Output: Hello, ForgeLang!
//         (with newline)
```

---

### `eprint(value)`

Print a value to stderr without a trailing newline.

**Parameters:**
- `value` - Any value to print to stderr

**Examples:**
```forge
import [eprint] from std.io;
eprint("Error: ");
eprint("Something went wrong");
```

---

### `eprintln(value)`

Print a value to stderr with a trailing newline.

**Parameters:**
- `value` - Any value to print to stderr

**Examples:**
```forge
import [eprintln] from std.io;
eprintln("Error: File not found");
```

---

### `read_line()`

Read a single line from stdin.

**Returns:** `str` - The input line (without trailing newline)

**Examples:**
```forge
import [read_line, println] from std.io;
println("Enter your name:");
var name = read_line();
println("Hello, {name}!");
```

---

### `read_all()`

Read all input from stdin until EOF.

**Returns:** `str` - All input from stdin

**Examples:**
```forge
import [read_all, println] from std.io;
var input = read_all();
println("Received {input.length()} characters");
```

---

### `format(value)`

Format a value as a string.

**Parameters:**
- `value` - Any value to convert

**Returns:** `str` - String representation

**Examples:**
```forge
import [format, println] from std.io;
var num = 42;
var str = format(num);
println("The answer is {str}");
```

## Related

- [`std.str`](./str) - String manipulation
- [`std.fs`](./fs) - File I/O
