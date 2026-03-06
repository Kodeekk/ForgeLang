---
sidebar_position: 9
---

# std.fs

Filesystem operations for reading, writing, and managing files.

## Functions

### `read(path)`

Read file contents as a string.

**Parameters:**
- `path` - Path to the file

**Returns:** File contents

**Examples:**
```forge
import [read] from std.fs;
var content = read("hello.txt");
println(content);
```

---

### `write(path, content)`

Write a string to a file.

**Parameters:**
- `path` - Path to the file
- `content` - Content to write

**Examples:**
```forge
import [write] from std.fs;
write("hello.txt", "Hello, World!");
```

---

### `append(path, content)`

Append a string to a file.

**Parameters:**
- `path` - Path to the file
- `content` - Content to append

**Examples:**
```forge
import [append] from std.fs;
append("log.txt", "New log entry\n");
```

---

### `exists(path)`

Check if a file or directory exists.

**Parameters:**
- `path` - Path to check

**Returns:** `true` if exists

**Examples:**
```forge
import [exists] from std.fs;
if exists("config.yaml") {
    println("Config found!");
}
```

---

### `remove(path)`

Remove (delete) a file.

**Parameters:**
- `path` - Path to the file

**Examples:**
```forge
import [remove] from std.fs;
remove("temp.txt");
```

---

### `dirname(path)`

Get the directory name from a path.

**Parameters:**
- `path` - The path

**Returns:** Directory portion

**Examples:**
```forge
import [dirname] from std.fs;
dirname("/home/user/file.txt");   // "/home/user"
dirname("file.txt");              // ""
dirname("/");                     // "/"
```

---

### `basename(path)`

Get the file name from a path.

**Parameters:**
- `path` - The path

**Returns:** File name portion

**Examples:**
```forge
import [basename] from std.fs;
basename("/home/user/file.txt");   // "file.txt"
basename("/home/user");            // "user"
basename("file.txt");              // "file.txt"
```

---

### `extension(path)`

Get the file extension.

**Parameters:**
- `path` - The path

**Returns:** Extension (without dot)

**Examples:**
```forge
import [extension] from std.fs;
extension("file.txt");        // "txt"
extension("archive.tar.gz");  // "gz"
extension("README");          // ""
```

---

### `file_stem(path)`

Get the file name without extension.

**Parameters:**
- `path` - The path

**Returns:** File name without extension

**Examples:**
```forge
import [file_stem] from std.fs;
file_stem("file.txt");        // "file"
file_stem("archive.tar.gz");  // "archive.tar"
file_stem("README");          // "README"
```

---

### `create_dir(path)`

Create a directory.

**Parameters:**
- `path` - Path for the new directory

**Examples:**
```forge
import [create_dir] from std.fs;
create_dir("output");
```

---

### `read_dir(path)`

Read directory entries.

**Parameters:**
- `path` - Path to the directory

**Returns:** List of entries

**Examples:**
```forge
import [read_dir] from std.fs;
var entries = read_dir(".");
for entry in entries {
    println(entry);
}
```

## Examples

### Read-Modify-Write

```forge
import [read, write, exists] from std.fs;

fn main() -> int {
    var path = "config.txt";
    
    if exists(path) {
        var content = read(path);
        var modified = content.upper();
        write(path, modified);
    }
    
    return 0;
}
```

### Directory Traversal

```forge
import [read_dir, exists, dirname, basename] from std.fs;

fn list_files_recursive(path: str, indent: int) -> void {
    if !exists(path) { return; }
    
    var prefix = "  ".repeat(indent);
    println("{prefix}{basename(path)}");
    
    var entries = read_dir(path);
    for entry in entries {
        list_files_recursive("{path}/{entry}", indent + 1);
    }
}
```

## Related

- [`std.io`](./io) - Basic I/O
- [`std.env`](./env) - Environment paths
