---
sidebar_position: 7
---

# std.int

Integer class and operations.

## Class `int`

Integer wrapper with utility methods.

### Constructor

#### `int.new(value)`

Convert value to integer.

**Parameters:**
- `value` - Any value to convert

**Returns:** New integer wrapper

**Examples:**
```forge
import std.int;
var n1 = int.new(42);
var n2 = int.new("123");
```

---

### Methods

#### `to_str()`

Convert to string.

**Returns:** String representation

**Examples:**
```forge
import std.int;
int.new(42).to_str();   // "42"
int.new(-1).to_str();   // "-1"
```

---

#### `to_float()`

Convert to float.

**Returns:** Float representation

**Examples:**
```forge
import std.int;
int.new(42).to_float();   // 42.0
```

---

#### `abs()`

Get absolute value.

**Returns:** Non-negative value

**Examples:**
```forge
import std.int;
int.new(-5).abs();   // 5
int.new(5).abs();    // 5
int.new(0).abs();    // 0
```

---

#### `is_even()`

Check if even.

**Returns:** `true` if divisible by 2

**Examples:**
```forge
import std.int;
int.new(4).is_even();   // true
int.new(7).is_even();   // false
int.new(0).is_even();   // true
```

---

#### `is_odd()`

Check if odd.

**Returns:** `true` if not divisible by 2

**Examples:**
```forge
import std.int;
int.new(7).is_odd();    // true
int.new(4).is_odd();    // false
```

---

#### `is_positive()`

Check if positive.

**Returns:** `true` if greater than 0

**Examples:**
```forge
import std.int;
int.new(5).is_positive();   // true
int.new(-3).is_positive();  // false
int.new(0).is_positive();   // false
```

---

#### `is_negative()`

Check if negative.

**Returns:** `true` if less than 0

**Examples:**
```forge
import std.int;
int.new(-3).is_negative();  // true
int.new(5).is_negative();   // false
int.new(0).is_negative();   // false
```

---

#### `is_zero()`

Check if zero.

**Returns:** `true` if equal to 0

**Examples:**
```forge
import std.int;
int.new(0).is_zero();     // true
int.new(5).is_zero();     // false
```

---

#### `sign()`

Get the sign.

**Returns:** -1 if negative, 0 if zero, 1 if positive

**Examples:**
```forge
import std.int;
int.new(-5).sign();   // -1
int.new(0).sign();    // 0
int.new(5).sign();    // 1
```

---

#### `clamp(min_val, max_val)`

Clamp between min and max.

**Parameters:**
- `min_val` - Minimum value
- `max_val` - Maximum value

**Returns:** Clamped value

**Examples:**
```forge
import std.int;
int.new(5).clamp(0, 10);    // 5
int.new(-1).clamp(0, 10);   // 0
int.new(15).clamp(0, 10);   // 10
```

## Related

- [`std.math`](./math) - Math operations
- [`std.float`](./float) - Float operations
