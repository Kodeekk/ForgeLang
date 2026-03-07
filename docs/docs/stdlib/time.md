---
sidebar_position: 10
---

# std.time

Time operations and utilities.

## Functions

### `now()`

Get current time in milliseconds since epoch.

**Returns:** Milliseconds since January 1, 1970

**Examples:**
```forge
import [now] from std.time;
var timestamp = now();
println("Current timestamp: {timestamp}");
```

---

### `sleep(ms)`

Sleep for a specified duration.

**Parameters:**
- `ms` - Milliseconds to sleep

**Examples:**
```forge
import [sleep] from std.time;
println("Sleeping for 1 second...");
sleep(1000);
println("Awake!");
```

---

### `sleep_secs(secs)`

Sleep for a specified number of seconds.

**Parameters:**
- `secs` - Seconds to sleep

**Examples:**
```forge
import [sleep_secs] from std.time;
sleep_secs(5);   // Sleep for 5 seconds
```

---

### `now_secs()`

Get current time in seconds since epoch.

**Returns:** Seconds since January 1, 1970

**Examples:**
```forge
import [now_secs] from std.time;
var timestamp = now_secs();
println("Current timestamp: {timestamp} seconds");
```

---

### `format_duration(ms)`

Format milliseconds as human-readable string.

**Parameters:**
- `ms` - Duration in milliseconds

**Returns:** Formatted string

**Examples:**
```forge
import [format_duration] from std.time;
format_duration(3661000);   // "1h 1m 1s"
format_duration(5000);      // "5s"
format_duration(1500);      // "1s 500ms"
```

---

### `measure(func)`

Measure execution time of a function.

**Parameters:**
- `func` - Function to measure

**Returns:** Execution time in milliseconds

**Examples:**
```forge
import [measure] from std.time;
var ms = measure(fn() {
    var sum = 0;
    for i in range(0, 1000) {
        sum = sum + i;
    }
});
println("Took {ms} ms");
```

---

### `benchmark(label, func)`

Measure and print execution time.

**Parameters:**
- `label` - Label for output
- `func` - Function to measure

**Examples:**
```forge
import [benchmark] from std.time;
benchmark("sort test", fn() {
    var data = [3, 1, 4, 1, 5, 9, 2, 6];
    sort.quick(data);
});
// Output: [sort test] completed in X ms
```

## Examples

### Timing Operations

```forge
import [now, format_duration, measure] from std.time;
import [println] from std.io;

fn main() -> int {
    var start = now();
    
    // Some operation
    var result = expensive_computation();
    
    var elapsed = now() - start;
    println("Operation took {format_duration(elapsed)}");
    
    return 0;
}
```

### Rate Limiting

```forge
import [now, sleep] from std.time;

class RateLimiter {
    var last_call: int;
    var min_interval: int;
    
    fn new(min_interval_ms: int) -> RateLimiter {
        return RateLimiter {
            last_call: 0,
            min_interval: min_interval_ms
        };
    }
    
    fn wait_if_needed(self) -> void {
        var now = std.time.now();
        var elapsed = now - self.last_call;
        if elapsed < self.min_interval {
            sleep(self.min_interval - elapsed);
        }
        self.last_call = std.time.now();
    }
}
```

### Performance Comparison

```forge
import [benchmark] from std.time;

fn main() -> int {
    var data = list.range(0, 10000);
    
    benchmark("bubble_sort", fn() {
        bubble_sort(data.clone());
    });
    
    benchmark("quick_sort", fn() {
        quick_sort(data.clone());
    });
    
    return 0;
}
```

## Related

- [`std.env`](./env) - System information
- [Benchmarking](../tooling/testing) - Performance testing
