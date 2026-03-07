---
sidebar_position: 5
---

# Operators

ForgeLang supports a variety of operators for different operations.

## Arithmetic Operators

Basic mathematical operations:

```forge
var a = 10;
var b = 3;

// Addition
var sum = a + b;        // 13

// Subtraction
var diff = a - b;       // 7

// Multiplication
var product = a * b;    // 30

// Division (integer)
var quotient = a / b;   // 3

// Division (float)
var float_div = f64(a) / f64(b);  // 3.333...

// Modulo (remainder)
var remainder = a % b;  // 1

// Exponentiation
var power = math.pow(a, b);  // 1000

// Negation
var negative = -a;      // -10
```

## Comparison Operators

Compare values and get boolean results:

```forge
var x = 10;
var y = 5;

// Equal
x == y;   // false
x == 10;  // true

// Not equal
x != y;   // true
x != 10;  // false

// Greater than
x > y;    // true
y > x;    // false

// Less than
x < y;    // false
y < x;    // true

// Greater than or equal
x >= 10;  // true
x >= 11;  // false

// Less than or equal
y <= 5;   // true
y <= 4;   // false
```

## Logical Operators

Boolean logic operations:

```forge
var is_admin = true;
var is_active = false;

// AND (both must be true)
is_admin && is_active;  // false

// OR (at least one true)
is_admin || is_active;  // true

// NOT (invert)
!is_admin;   // false
!is_active;  // true

// Short-circuit evaluation
is_admin && perform_admin_action();  // Only called if is_admin is true
is_active || activate_user();        // Only called if is_active is false
```

## Bitwise Operators

Operate on individual bits:

```forge
var a = 0b1100;  // 12
var b = 0b1010;  // 10

// AND
a & b;   // 0b1000 (8)

// OR
a | b;   // 0b1110 (14)

// XOR
a ^ b;   // 0b0110 (6)

// NOT (complement)
~a;      // 0b0011 (3) for 4-bit

// Left shift
a << 1;  // 0b11000 (24)

// Right shift
a >> 1;  // 0b0110 (6)
```

## Assignment Operators

Assign and modify values:

```forge
var x = 10;

// Simple assignment
x = 20;

// Add and assign
x += 5;   // x = x + 5

// Subtract and assign
x -= 3;   // x = x - 3

// Multiply and assign
x *= 2;   // x = x * 2

// Divide and assign
x /= 4;   // x = x / 4

// Modulo and assign
x %= 3;   // x = x % 3
```

## String Operators

String-specific operations:

```forge
// Concatenation
var greeting = "Hello" + " " + "World";  // "Hello World"

// String interpolation (preferred)
var name = "Alice";
var message = "Hello, {name}!";  // "Hello, Alice!"

// Length
var len = "Hello".length();  // 5

// Access character
var first = "Hello"[0];  // "H"
```

## List Operators

Operations on lists:

```forge
var nums = [1, 2, 3];

// Access
var first = nums[0];  // 1

// Modify
nums[0] = 10;  // [10, 2, 3]

// Length
var len = nums.length();  // 3

// Concatenation
var combined = [1, 2] + [3, 4];  // [1, 2, 3, 4]

// Contains
var has_two = nums.contains(2);  // true
```

## Map Operators

Operations on maps:

```forge
var ages: map<str, int> = map.new();

// Set value
ages.set("Alice", 30);

// Get value
var alice_age = ages.get("Alice");  // 30

// Check existence
var has_alice = ages.has("Alice");  // true

// Delete
ages.delete("Alice");
```

## Operator Precedence

From highest to lowest:

```forge
// 1. Postfix
expr()    // Function call
expr[]    // Indexing
expr.field // Field access

// 2. Unary
-expr     // Negation
!expr     // Logical NOT
~expr     // Bitwise NOT

// 3. Multiplicative
* / %     // Multiply, divide, modulo

// 4. Additive
+ -       // Add, subtract

// 5. Shift
<< >>     // Left, right shift

// 6. Comparison
< > <= >= // Relational

// 7. Equality
== !=     // Equal, not equal

// 8. Bitwise AND
&

// 9. Bitwise XOR
^

// 10. Bitwise OR
|

// 11. Logical AND
&&

// 12. Logical OR
||

// 13. Assignment
= += -= *= /= %=  // Assignment operators
```

## Using Parentheses

Control evaluation order:

```forge
// Without parentheses
var result = 2 + 3 * 4;  // 14 (multiplication first)

// With parentheses
var result = (2 + 3) * 4;  // 20 (addition first)

// Complex expressions
var total = (price * quantity) + (price * quantity * tax_rate);

// Improve readability
var is_valid = (age >= 18) && (has_license || has_permit);
```

## Custom Operators via Methods

Implement operator-like behavior:

```forge
class Vector {
    var x: f64;
    var y: f64;
    
    fn add(self, other: Vector) -> Vector {
        return Vector {
            x: self.x + other.x,
            y: self.y + other.y
        };
    }
    
    fn eq(self, other: Vector) -> bool {
        return self.x == other.x && self.y == other.y;
    }
}

var v1 = Vector { x: 1.0, y: 2.0 };
var v2 = Vector { x: 3.0, y: 4.0 };
var v3 = v1.add(v2);
var equal = v1.eq(v2);
```

## Next Steps

- **[Control Flow](./control-flow)** - Conditionals and loops
- **[Functions](./functions)** - Function definitions
- **[Classes](../advanced/classes)** - Custom types with operators
