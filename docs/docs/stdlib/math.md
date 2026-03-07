---
sidebar_position: 3
---

# std.math

Mathematical operations and constants.

## Constants

### `PI`

Mathematical constant π (Pi) ≈ 3.1415926535897932

```forge
import std.math;
var circumference = 2.0 * math.PI * radius;
```

---

### `E`

Mathematical constant e (Euler's number) ≈ 2.7182818284590452

```forge
import std.math;
var exponential = math.pow(math.E, x);
```

## Functions

### `abs(n)`

Get the absolute value of a number.

**Parameters:**
- `n` - A number (int or float)

**Returns:** The absolute value

**Examples:**
```forge
import [abs] from std.math;
abs(-5);    // 5
abs(3.14);  // 3.14
abs(0);     // 0
```

---

### `min(a, b)`

Get the minimum of two values.

**Parameters:**
- `a` - First value
- `b` - Second value

**Returns:** The smaller value

**Examples:**
```forge
import [min] from std.math;
min(3, 5);   // 3
min(-1, 1);  // -1
```

---

### `max(a, b)`

Get the maximum of two values.

**Parameters:**
- `a` - First value
- `b` - Second value

**Returns:** The larger value

**Examples:**
```forge
import [max] from std.math;
max(3, 5);   // 5
max(-1, 1);  // 1
```

---

### `pow(base, exp)`

Raise a base to an exponent power.

**Parameters:**
- `base` - The base number
- `exp` - The exponent

**Returns:** base^exp

**Examples:**
```forge
import [pow] from std.math;
pow(2, 3);    // 8
pow(10, 2);   // 100
pow(2, -1);   // 0.5
```

---

### `sqrt(n)`

Get the square root of a number.

**Parameters:**
- `n` - A non-negative number

**Returns:** √n

**Examples:**
```forge
import [sqrt] from std.math;
sqrt(16);    // 4.0
sqrt(2.25);  // 1.5
sqrt(0);     // 0.0
```

---

### `sign(n)`

Sign function: returns -1, 0, or 1.

**Parameters:**
- `n` - A number

**Returns:** -1 if negative, 0 if zero, 1 if positive

**Examples:**
```forge
import [sign] from std.math;
sign(-5);   // -1
sign(0);    // 0
sign(10);   // 1
```

---

### `is_even(n)`

Check if a number is even.

**Parameters:**
- `n` - An integer

**Returns:** `true` if divisible by 2

**Examples:**
```forge
import [is_even] from std.math;
is_even(4);   // true
is_even(7);   // false
is_even(0);   // true
```

---

### `clamp(value, min_val, max_val)`

Clamp a value between min and max.

**Parameters:**
- `value` - The value to clamp
- `min_val` - Minimum allowed value
- `max_val` - Maximum allowed value

**Returns:** Value clamped to range

**Examples:**
```forge
import [clamp] from std.math;
clamp(5, 0, 10);    // 5
clamp(-1, 0, 10);   // 0
clamp(15, 0, 10);   // 10
```

---

### `gcd(a, b)`

Greatest common divisor of two integers.

**Parameters:**
- `a` - First integer
- `b` - Second integer

**Returns:** GCD of a and b

**Examples:**
```forge
import [gcd] from std.math;
gcd(48, 18);   // 6
gcd(17, 13);   // 1
gcd(-12, 8);   // 4
```

---

### `lcm(a, b)`

Least common multiple of two integers.

**Parameters:**
- `a` - First integer
- `b` - Second integer

**Returns:** LCM of a and b

**Examples:**
```forge
import [lcm] from std.math;
lcm(4, 6);     // 12
lcm(3, 7);     // 21
lcm(0, 5);     // 0
```

---

### `is_prime(n)`

Check if a number is prime.

**Parameters:**
- `n` - An integer

**Returns:** `true` if prime

**Examples:**
```forge
import [is_prime] from std.math;
is_prime(2);    // true
is_prime(17);   // true
is_prime(4);    // false
is_prime(1);    // false
```

---

### `factorial(n)`

Compute the factorial of a number.

**Parameters:**
- `n` - A non-negative integer

**Returns:** n!

**Examples:**
```forge
import [factorial] from std.math;
factorial(0);   // 1
factorial(1);   // 1
factorial(5);   // 120
```

---

### `fib(n)`

Compute the n-th Fibonacci number.

**Parameters:**
- `n` - Position in Fibonacci sequence (0-indexed)

**Returns:** The n-th Fibonacci number

**Examples:**
```forge
import [fib] from std.math;
fib(0);   // 0
fib(1);   // 1
fib(10);  // 55
```

## Related

- [`std.int`](./int) - Integer operations
- [`std.float`](./float) - Float operations
