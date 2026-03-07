---
sidebar_position: 6
---

# std.bool

Boolean class and logical operations.

## Class `bool`

Boolean wrapper with utility methods.

### Constructor

#### `bool.new(value)`

Convert value to boolean.

**Parameters:**
- `value` - Any value to convert

**Returns:** New boolean wrapper

**Examples:**
```forge
import std.bool;
var b1 = bool.new(true);
var b2 = bool.new(false);
```

---

### Methods

#### `to_str()`

Convert to string.

**Returns:** `"true"` or `"false"`

**Examples:**
```forge
import std.bool;
var b = bool.new(true);
b.to_str();   // "true"
```

---

#### `to_int()`

Convert to integer.

**Returns:** `1` for true, `0` for false

**Examples:**
```forge
import std.bool;
bool.new(true).to_int();   // 1
bool.new(false).to_int();  // 0
```

---

#### `and(other)`

Logical AND operation.

**Parameters:**
- `other` - Another boolean value

**Returns:** `true` if both are true

**Examples:**
```forge
import std.bool;
bool.new(true).and(true);    // true
bool.new(true).and(false);   // false
```

---

#### `or(other)`

Logical OR operation.

**Parameters:**
- `other` - Another boolean value

**Returns:** `true` if either is true

**Examples:**
```forge
import std.bool;
bool.new(true).or(false);    // true
bool.new(false).or(false);   // false
```

---

#### `xor(other)`

Logical XOR (exclusive OR).

**Parameters:**
- `other` - Another boolean value

**Returns:** `true` if exactly one is true

**Examples:**
```forge
import std.bool;
bool.new(true).xor(false);   // true
bool.new(true).xor(true);    // false
bool.new(false).xor(false);  // false
```

---

#### `not()`

Logical NOT operation.

**Returns:** Inverted boolean

**Examples:**
```forge
import std.bool;
bool.new(true).not();    // false
bool.new(false).not();   // true
```

---

#### `toggle()`

Toggle the boolean value.

**Returns:** Opposite value

**Examples:**
```forge
import std.bool;
bool.new(true).toggle();   // false
bool.new(false).toggle();  // true
```

## Related

- [Operators](../language-guide/operators) - Logical operators
- [Control Flow](../language-guide/control-flow) - Conditionals
