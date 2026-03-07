---
sidebar_position: 11
---

# std.env

Environment variables and system information.

## Functions

### `cwd()`

Get the current working directory.

**Returns:** Absolute path of current directory

**Examples:**
```forge
import [cwd] from std.env;
var dir = cwd();
println("Working in: {dir}");
```

---

### `home()`

Get the home directory of the current user.

**Returns:** Absolute path of home directory

**Examples:**
```forge
import [home] from std.env;
var home_dir = home();
println("Home: {home_dir}");
```

---

### `hostname()`

Get the hostname of the machine.

**Returns:** System hostname

**Examples:**
```forge
import [hostname] from std.env;
var host = hostname();
println("Running on: {host}");
```

---

### `os()`

Get the name of the operating system.

**Returns:** OS name (e.g., "linux", "windows", "macos")

**Examples:**
```forge
import [os] from std.env;
var os_name = os();
println("Running on: {os_name}");
```

---

### `get(name)`

Get the value of an environment variable.

**Parameters:**
- `name` - Environment variable name

**Returns:** Variable value

**Examples:**
```forge
import [get] from std.env;
var path = get("PATH");
println("PATH: {path}");
```

---

### `set(name, value)`

Set an environment variable.

**Parameters:**
- `name` - Environment variable name
- `value` - Value to set

**Examples:**
```forge
import [set] from std.env;
set("MY_VAR", "hello");
```

---

### `has(name)`

Check if an environment variable exists.

**Parameters:**
- `name` - Environment variable name

**Returns:** `true` if exists

**Examples:**
```forge
import [has] from std.env;
if has("HOME") {
    println("HOME is set");
}
```

---

### `remove(name)`

Remove (unset) an environment variable.

**Parameters:**
- `name` - Environment variable name

**Examples:**
```forge
import [remove] from std.env;
remove("MY_VAR");
```

---

### `vars()`

Get all environment variables.

**Returns:** List of key-value pairs

**Examples:**
```forge
import [vars] from std.env;
var all_vars = vars();
for (key, value) in all_vars {
    println("{key}={value}");
}
```

---

### `user()`

Get the current user name.

**Returns:** User name or "unknown"

**Examples:**
```forge
import [user] from std.env;
var username = user();
println("Hello, {username}!");
```

## Examples

### Environment Configuration

```forge
import [get, has] from std.env;

fn get_config() -> Config {
    var debug = has("DEBUG") && get("DEBUG") == "true";
    var log_level = get("LOG_LEVEL");
    var api_key = get("API_KEY");
    
    return Config {
        debug: debug,
        log_level: log_level,
        api_key: api_key
    };
}
```

### System Information

```forge
import [cwd, home, hostname, os, user] from std.env;
import [println] from std.io;

fn print_system_info() -> void {
    println("System Information:");
    println("  OS:       {os()}");
    println("  Hostname: {hostname()}");
    println("  User:     {user()}");
    println("  Home:     {home()}");
    println("  CWD:      {cwd()}");
}
```

### Temporary Environment

```forge
import [get, set, remove, has] from std.env;

fn with_env_var(name: str, value: str, task: fn() -> void) -> void {
    var had_value = has(name);
    var old_value = get(name);
    
    set(name, value);
    task();
    
    if had_value {
        set(name, old_value);
    } else {
        remove(name);
    }
}

// Usage
with_env_var("TZ", "UTC", fn() {
    // Code runs with TZ=UTC
});
```

## Related

- [`std.fs`](./fs) - Filesystem paths
- [`std.time`](./time) - System time
