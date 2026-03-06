---
sidebar_position: 8
---

# std.float

Float class and operations.

## Class `float`

Floating-point wrapper with utility methods.

### Constructor

#### `float.new(value)`

Convert value to float.

**Parameters:**
- `value` - Any value to convert

**Returns:** New float wrapper

**Examples:**
```forge
import std.float;
var f1 = float.new(3.14);
var f2 = float.new(42);
```

---

### Methods

#### `to_str()`

Convert to string.

**Returns:** String representation

**Examples:**
```forge
import std.float;
float.new(3.14).to_str();   // "3.14"
```

---

#### `to_int()`

Convert to integer (truncates).

**Returns:** Integer value

**Examples:**
```forge
import std.float;
float.new(3.14).to_int();   // 3
float.new(-2.7).to_int();   // -2
```

---

#### `abs()`

Get absolute value.

**Returns:** Non-negative value

**Examples:**
```forge
import std.float;
float.new(-3.14).abs();   // 3.14
float.new(2.5).abs();     // 2.5
```

---

#### `floor()`

Round down to nearest integer.

**Returns:** Floored value

**Examples:**
```forge
import std.float;
float.new(3.7).floor();    // 3.0
float.new(-2.3).floor();   // -3.0
float.new(4.0).floor();    // 4.0
```

---

#### `ceil()`

Round up to nearest integer.

**Returns:** Ceiled value

**Examples:**
```forge
import std.float;
float.new(3.2).ceil();    // 4.0
float.new(-2.7).ceil();   // -2.0
float.new(4.0).ceil();    // 4.0
```

---

#### `round()`

Round to nearest integer.

**Returns:** Rounded value

**Examples:**
```forge
import std.float;
float.new(3.5).round();    // 4.0
float.new(3.4).round();    // 3.0
float.new(-2.6).round();   // -3.0
```

---

#### `fract()`

Get fractional part.

**Returns:** Fractional part (value - floor(value))

**Examples:**
```forge
import std.float;
float.new(3.75).fract();   // 0.75
float.new(-2.3).fract();   // 0.7
```

---

#### `is_positive()`

Check if positive.

**Returns:** `true` if greater than 0.0

**Examples:**
```forge
import std.float;
float.new(3.14).is_positive();   // true
float.new(-2.5).is_positive();   // false
```

---

#### `is_negative()`

Check if negative.

**Returns:** `true` if less than 0.0

**Examples:**
```forge
import std.float;
float.new(-2.5).is_negative();   // true
float.new(3.14).is_negative();   // false
```

---

#### `is_zero()`

Check if zero.

**Returns:** `true` if equal to 0.0

**Examples:**
```forge
import std.float;
float.new(0.0).is_zero();    // true
float.new(0.001).is_zero();  // false
```

---

#### `is_finite()`

Check if finite (not infinity or NaN).

**Returns:** `true` if finite

**Examples:**
```forge
import std.float;
float.new(3.14).is_finite();   // true
```

---

#### `is_nan()`

Check if NaN (Not a Number).

**Returns:** `true` if NaN

**Examples:**
```forge
import std.float;
float.new(3.14).is_nan();    // false
```

---

#### `sign()`

Get the sign.

**Returns:** -1.0 if negative, 0.0 if zero, 1.0 if positive

**Examples:**
```forge
import std.float;
float.new(-3.14).sign();   // -1.0
float.new(0.0).sign();     // 0.0
float.new(2.5).sign();     // 1.0
```

## Related

- [`std.math`](./math) - Math operations
- [`std.int`](./int) - Integer operations
