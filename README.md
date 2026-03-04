# ForgeLang

A dynamically-typed interpreted language with a focus on simplicity and expressiveness.

## Installation

### From Source

```bash
# Build and install
make install

# This will:
# - Build both binaries (fl and maul)
# - Install binaries to /usr/local/bin (or $PREFIX/bin)
# - Create symlink to stdlib at ~/.forgelang/stdlib
```

### Uninstall

```bash
make uninstall
```

## Tools

- **`fl`** - The ForgeLang interpreter. Run `.fl` files directly.
- **`maul`** - The ForgeLang package manager. Manages projects and runs code.

## Usage

### Run a file directly with `fl`

```bash
fl my_script.fl
```

### Create a new project with `maul`

```bash
# Create new project
maul new my_project
cd my_project

# Run the project
maul run
```

### Project Structure

```
my_project/
├── maul.yaml          # Project manifest
├── src/
│   └── main.fl        # Entry point
└── .gitignore
```

### maul.yaml

```yaml
package:
  name: "my_project"
  version: "0.1.0"
  entry: "src/main.fl"
```

## Stdlib

The standard library is located at `~/.forgelang/stdlib` (symlinked from the source).

Modules available:
- `std.io` - Input/output operations
- `std.math` - Mathematical functions
- `std.str` - String utilities
- `std.list` - List operations
- `std.bool` - Boolean utilities
- `std.int` - Integer utilities
- `std.float` - Float utilities
- `std.fs` - Filesystem operations
- `std.time` - Time utilities
- `std.env` - Environment variables

## Example

```forge
import [println] from std.io;

fn main() -> int {
    println("Hello, ForgeLang!");
    return 0;
}
```

Run with:
```bash
maul run
```
