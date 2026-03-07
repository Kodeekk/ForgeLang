---
sidebar_position: 5
---

# std.str

String class and manipulation methods.

## Class `str`

String wrapper with utility methods.

### Constructor

#### `str.new(value)`

Convert any value to string.

**Parameters:**
- `value` - Any value to convert

**Returns:** New string wrapper

**Examples:**
```forge
import std.str;
var s1 = str.new("hello");
var s2 = str.new(42);
var s3 = str.new(3.14);
```

---

### Methods

#### `length()`

Get the string length.

**Returns:** Number of characters

**Examples:**
```forge
import std.str;
var s = str.new("Hello");
s.length();   // 5
```

---

#### `upper()`

Convert to uppercase.

**Returns:** Uppercase string

**Examples:**
```forge
import std.str;
var s = str.new("Hello");
s.upper();    // "HELLO"
```

---

#### `lower()`

Convert to lowercase.

**Returns:** Lowercase string

**Examples:**
```forge
import std.str;
var s = str.new("HELLO");
s.lower();    // "hello"
```

---

#### `reverse()`

Reverse the string.

**Returns:** Reversed string

**Examples:**
```forge
import std.str;
var s = str.new("Hello");
s.reverse();  // "olleH"
```

---

#### `trim()`

Trim whitespace from both ends.

**Returns:** Trimmed string

**Examples:**
```forge
import std.str;
var s = str.new("  Hello  ");
s.trim();     // "Hello"
```

---

#### `contains(substr)`

Check if string contains a substring.

**Parameters:**
- `substr` - Substring to search for

**Returns:** `true` if found

**Examples:**
```forge
import std.str;
var s = str.new("Hello, World!");
s.contains("World");   // true
s.contains("Rust");    // false
```

---

#### `split(delimiter)`

Split by delimiter.

**Parameters:**
- `delimiter` - Delimiter string

**Returns:** List of substrings

**Examples:**
```forge
import std.str;
var s = str.new("apple,banana,cherry");
s.split(",");   // ["apple", "banana", "cherry"]
```

---

#### `char_at(index)`

Get character at index.

**Parameters:**
- `index` - Character index (0-based)

**Returns:** Character at index

**Examples:**
```forge
import std.str;
var s = str.new("Hello");
s.char_at(0);   // "H"
s.char_at(4);   // "o"
```

---

#### `find(substr)`

Find substring index.

**Parameters:**
- `substr` - Substring to find

**Returns:** Index or -1 if not found

**Examples:**
```forge
import std.str;
var s = str.new("Hello, World!");
s.find("World");   // 7
s.find("Rust");    // -1
```

---

#### `replace(old, new)`

Replace all occurrences.

**Parameters:**
- `old` - Substring to replace
- `new` - Replacement string

**Returns:** String with replacements

**Examples:**
```forge
import std.str;
var s = str.new("Hello, World!");
s.replace("World", "ForgeLang");   // "Hello, ForgeLang!"
```

---

#### `starts_with(prefix)`

Check if string starts with prefix.

**Parameters:**
- `prefix` - Prefix to check

**Returns:** `true` if starts with prefix

**Examples:**
```forge
import std.str;
var s = str.new("Hello, World!");
s.starts_with("Hello");   // true
s.starts_with("World");   // false
```

---

#### `ends_with(suffix)`

Check if string ends with suffix.

**Parameters:**
- `suffix` - Suffix to check

**Returns:** `true` if ends with suffix

**Examples:**
```forge
import std.str;
var s = str.new("Hello, World!");
s.ends_with("World!");   // true
s.ends_with("World");    // false
```

---

#### `to_str()`

Convert to string (identity).

**Returns:** The underlying string

**Examples:**
```forge
import std.str;
var s = str.new("Hello");
s.to_str();   // "Hello"
```

---

#### `to_int()`

Convert to integer.

**Returns:** Integer value

**Examples:**
```forge
import std.str;
var s = str.new("42");
s.to_int();   // 42
```

---

#### `to_float()`

Convert to float.

**Returns:** Float value

**Examples:**
```forge
import std.str;
var s = str.new("3.14");
s.to_float();   // 3.14
```

## Related

- [`std.io`](./io) - I/O operations
- String interpolation in [Syntax](../language-guide/syntax)
