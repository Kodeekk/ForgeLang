---
sidebar_position: 1
---

# API: std.io

Complete API reference for `std.io` module.

## Functions

### `print(value: any) -> void`

Print a value to stdout without a trailing newline.

**Parameters:**
| Name | Type | Description |
|------|------|-------------|
| `value` | `any` | Any value to print |

**Examples:**
```forge
import [print] from std.io;
print("Hello");
print(" World");
// Output: Hello World
```

---

### `println(value: any) -> void`

Print a value to stdout with a trailing newline.

**Parameters:**
| Name | Type | Description |
|------|------|-------------|
| `value` | `any` | Any value to print |

**Examples:**
```forge
import [println] from std.io;
println("Hello, ForgeLang!");
```

---

### `eprint(value: any) -> void`

Print a value to stderr without a trailing newline.

**Parameters:**
| Name | Type | Description |
|------|------|-------------|
| `value` | `any` | Value to print to stderr |

**Examples:**
```forge
import [eprint] from std.io;
eprint("Error: ");
eprint("Something went wrong");
```

---

### `eprintln(value: any) -> void`

Print a value to stderr with a trailing newline.

**Parameters:**
| Name | Type | Description |
|------|------|-------------|
| `value` | `any` | Value to print to stderr |

**Examples:**
```forge
import [eprintln] from std.io;
eprintln("Error: File not found");
```

---

### `read_line() -> str`

Read a single line from stdin.

**Returns:** `str` - Input line without trailing newline

**Examples:**
```forge
import [read_line, println] from std.io;
println("Enter your name:");
var name = read_line();
println("Hello, {name}!");
```

---

### `read_all() -> str`

Read all input from stdin until EOF.

**Returns:** `str` - All input from stdin

**Examples:**
```forge
import [read_all, println] from std.io;
var input = read_all();
println("Received {input.length()} characters");
```

---

### `format(value: any) -> str`

Format a value as a string.

**Parameters:**
| Name | Type | Description |
|------|------|-------------|
| `value` | `any` | Value to convert |

**Returns:** `str` - String representation

**Examples:**
```forge
import [format, println] from std.io;
var num = 42;
var str = format(num);
println("The answer is {str}");
```

## See Also

- [`std.str`](./str) - String manipulation
- [`std.fs`](./fs) - File I/O
