---
sidebar_position: 4
---

# std.list

List operations and utilities.

## Functions

### `range(start, end)`

Create a list of integers from start to end (exclusive).

**Parameters:**
- `start` - Starting value (inclusive)
- `end` - Ending value (exclusive)

**Returns:** List of integers

**Examples:**
```forge
import [range] from std.list;
range(0, 5);    // [0, 1, 2, 3, 4]
range(3, 7);    // [3, 4, 5, 6]
range(0, 0);    // []
```

---

### `repeat(value, count)`

Create a list with a repeated value.

**Parameters:**
- `value` - The value to repeat
- `count` - Number of times to repeat

**Returns:** List with repeated values

**Examples:**
```forge
import [repeat] from std.list;
repeat(0, 5);        // [0, 0, 0, 0, 0]
repeat("x", 3);      // ["x", "x", "x"]
```

---

### `sum(list)`

Compute the sum of all elements.

**Parameters:**
- `list` - List of integers

**Returns:** Sum of elements

**Examples:**
```forge
import [sum] from std.list;
sum([1, 2, 3, 4]);   // 10
sum([]);             // 0
```

---

### `product(list)`

Compute the product of all elements.

**Parameters:**
- `list` - List of integers

**Returns:** Product of elements

**Examples:**
```forge
import [product] from std.list;
product([1, 2, 3, 4]);   // 24
product([]);             // 1
```

---

### `filter(list, predicate)`

Filter a list by a predicate function.

**Parameters:**
- `list` - The list to filter
- `predicate` - Function that returns bool

**Returns:** Filtered list

**Examples:**
```forge
import [filter] from std.list;
var nums = [1, 2, 3, 4, 5];
var evens = filter(nums, fn(n) { return n % 2 == 0; });
// evens = [2, 4]
```

---

### `map(list, func)`

Transform each element with a function.

**Parameters:**
- `list` - The list to transform
- `func` - Function to apply

**Returns:** Transformed list

**Examples:**
```forge
import [map] from std.list;
var nums = [1, 2, 3];
var doubled = map(nums, fn(n) { return n * 2; });
// doubled = [2, 4, 6]
```

---

### `reduce(list, initial, func)`

Reduce a list to a single value.

**Parameters:**
- `list` - The list to reduce
- `initial` - Initial accumulator value
- `func` - Function (accumulator, element) -> new accumulator

**Returns:** Final accumulated value

**Examples:**
```forge
import [reduce] from std.list;
var nums = [1, 2, 3, 4];
var sum = reduce(nums, 0, fn(acc, n) { return acc + n; });
// sum = 10
```

---

### `any(list, predicate)`

Check if any element satisfies a predicate.

**Parameters:**
- `list` - The list to check
- `predicate` - Function that returns bool

**Returns:** `true` if any element matches

**Examples:**
```forge
import [any] from std.list;
var nums = [1, 2, 3, 4];
any(nums, fn(n) { return n > 3; });   // true
any(nums, fn(n) { return n > 10; });  // false
```

---

### `all(list, predicate)`

Check if all elements satisfy a predicate.

**Parameters:**
- `list` - The list to check
- `predicate` - Function that returns bool

**Returns:** `true` if all elements match

**Examples:**
```forge
import [all] from std.list;
var nums = [2, 4, 6, 8];
all(nums, fn(n) { return n % 2 == 0; });   // true
all(nums, fn(n) { return n > 3; });        // false
```

---

### `find_index(list, value)`

Find the index of a value.

**Parameters:**
- `list` - The list to search
- `value` - The value to find

**Returns:** Index or -1 if not found

**Examples:**
```forge
import [find_index] from std.list;
var fruits = ["apple", "banana", "cherry"];
find_index(fruits, "banana");   // 1
find_index(fruits, "orange");   // -1
```

---

### `count(list, value)`

Count occurrences of a value.

**Parameters:**
- `list` - The list to search
- `value` - The value to count

**Returns:** Number of occurrences

**Examples:**
```forge
import [count] from std.list;
var nums = [1, 2, 2, 3, 2];
count(nums, 2);   // 3
count(nums, 5);   // 0
```

---

### `reverse(list)`

Reverse a list.

**Parameters:**
- `list` - The list to reverse

**Returns:** Reversed list

**Examples:**
```forge
import [reverse] from std.list;
reverse([1, 2, 3]);   // [3, 2, 1]
reverse([]);          // []
```

---

### `take(list, n)`

Get the first n elements.

**Parameters:**
- `list` - The list to take from
- `n` - Number of elements

**Returns:** First n elements

**Examples:**
```forge
import [take] from std.list;
take([1, 2, 3, 4, 5], 3);   // [1, 2, 3]
take([1, 2], 5);            // [1, 2]
```

---

### `skip(list, n)`

Skip the first n elements.

**Parameters:**
- `list` - The list to skip from
- `n` - Number of elements to skip

**Returns:** Remaining elements

**Examples:**
```forge
import [skip] from std.list;
skip([1, 2, 3, 4, 5], 2);   // [3, 4, 5]
skip([1, 2], 5);            // []
```

---

### `concat(a, b)`

Concatenate two lists.

**Parameters:**
- `a` - First list
- `b` - Second list

**Returns:** Combined list

**Examples:**
```forge
import [concat] from std.list;
concat([1, 2], [3, 4]);   // [1, 2, 3, 4]
```

---

### `dedup(list)`

Remove duplicates from a list.

**Parameters:**
- `list` - The list to deduplicate

**Returns:** Deduplicated list

**Examples:**
```forge
import [dedup] from std.list;
dedup([1, 2, 2, 3, 2, 1]);   // [1, 2, 3]
```

---

### `slice(list, start, end)`

Get a sublist from start to end (exclusive).

**Parameters:**
- `list` - The list to slice
- `start` - Starting index (inclusive)
- `end` - Ending index (exclusive)

**Returns:** Sublist

**Examples:**
```forge
import [slice] from std.list;
slice([1, 2, 3, 4, 5], 1, 4);   // [2, 3, 4]
slice([1, 2, 3], 0, 2);         // [1, 2]
```

## Related

- [`std.math`](./math) - Math operations
- Higher-order functions in [Functions](../language-guide/functions)
