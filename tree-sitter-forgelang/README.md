# Tree-sitter ForgeLang

Tree-sitter grammar for the **ForgeLang** programming language - a modern systems programming language with Rust-like syntax, inspired by Python's ergonomics.

## Features

- ✅ Full syntax highlighting
- ✅ Code folding
- ✅ Incremental selection
- ✅ Indentation support
- ✅ Tree-sitter queries for LSP integration

## Installation

### Quick Setup

Run the setup script:

```bash
cd ~/RustroverProjects/ForgeLang
./setup-nvim.sh
```

Then restart Neovim.

### Manual Installation

1. **Install dependencies:**

```bash
npm install
npm install -g tree-sitter-cli
```

2. **Generate the parser:**

```bash
tree-sitter generate
tree-sitter build
```

3. **Copy to Neovim:**

```bash
mkdir -p ~/.local/share/nvim/lazy/nvim-treesitter/parser
cp build/Release/tree-sitter_forgelang.so ~/.local/share/nvim/lazy/nvim-treesitter/parser/forgelang.so
```

4. **Add to your nvim-treesitter config:**

```lua
require('nvim-treesitter.configs').setup({
  ensure_installed = {
    'forgelang',
    -- other languages...
  },
  highlight = {
    enable = true,
  },
})
```

5. **Add filetype detection:**

Create `~/.config/nvim/after/ftdetect/forgelang.lua`:

```lua
vim.filetype.add({
  extension = { fl = 'forgelang' },
})
```

## Usage

Open a `.fl` file in Neovim and syntax highlighting should work automatically.

### Commands

- `:TSInstallFromGrammar forgelang` - Install from grammar
- `:TSUpdate forgelang` - Update the parser
- `:checkhealth nvim-treesitter` - Verify installation

### Keybindings (suggested)

```lua
vim.keymap.set('n', '<leader>r', ':!ForgeLang %<CR>', { desc = 'Run ForgeLang' })
vim.keymap.set('n', '<leader>b', ':!ForgeLang build<CR>', { desc = 'Build ForgeLang' })
```

## Syntax Highlighting Groups

| Group | Description | Example |
|-------|-------------|---------|
| `@keyword` | Language keywords | `fn`, `let`, `match` |
| `@keyword.function` | Function keywords | `fn`, `return` |
| `@keyword.type` | Type keywords | `class`, `enum`, `interface` |
| `@type` | Type names | `int`, `str`, custom types |
| `@type.builtin` | Built-in types | `int`, `f64`, `bool` |
| `@function` | Function definitions | `fn main()` |
| `@function.call` | Function calls | `println()` |
| `@method` | Method definitions | `fn to_str(self)` |
| `@method.call` | Method calls | `string.trim()` |
| `@variable` | Variables | `let x = 5` |
| `@variable.parameter` | Function parameters | `fn foo(x: int)` |
| `@constant` | Constants | `const PI = 3.14` |
| `@string` | String literals | `"hello"` |
| `@string.escape` | Escape sequences | `\n`, `\t` |
| `@string.interpolation` | String interpolation | `{variable}` |
| `@number` | Numeric literals | `42`, `3.14` |
| `@boolean` | Boolean literals | `true`, `false` |
| `@comment` | Comments | `// comment` |
| `@operator` | Operators | `+`, `-`, `=>` |
| `@punctuation.bracket` | Brackets | `()`, `[]`, `{}` |
| `@punctuation.delimiter` | Delimiters | `.`, `,`, `;` |

## Language Features

### Module System

```forgelang
module my.module;

import [print, println] from std.io;
import std.math as math;
```

### Type System

```forgelang
const PI: f64 = 3.14159;
var count: int = 0;

type Predicate<T> = fn(T) -> bool;
type Matrix = list<list<f64>>;
```

### Enums & Pattern Matching

```forgelang
enum Option<T> {
    Some(value: T),
    None
}

enum Result<T, E> {
    Ok(value: T),
    Err(error: E)
}

match result {
    Result.Ok(v) => println("Success: {v}"),
    Result.Err(e) => println("Error: {e}")
}
```

### Classes & Interfaces

```forgelang
interface Displayable {
    fn to_str(self) -> str;
}

class Point implements Displayable {
    var x: int;
    var y: int;

    fn new(x: int, y: int) -> Point {
        return Point { x: x, y: y };
    }

    fn to_str(self) -> str {
        return "Point({self.x}, {self.y})";
    }
}
```

### Generics

```forgelang
fn identity<T>(value: T) -> T {
    return value;
}

fn map<T, U>(list: list<T>, f: fn(T) -> U) -> list<U> {
    var result: list<U> = [];
    for item in list {
        result.push(f(item));
    }
    return result;
}
```

## Development

### Testing the Grammar

```bash
# Parse a file
tree-sitter parse test.fl

# Test highlights
tree-sitter highlight test.fl

# Run tests
tree-sitter test
```

### Modifying the Grammar

1. Edit `grammar.js`
2. Run `tree-sitter generate`
3. Run `tree-sitter build`
4. Test with `tree-sitter parse`

## License

MIT License - See LICENSE file for details.

## Contributing

Contributions welcome! Please read CONTRIBUTING.md first.

## Links

- [ForgeLang Compiler](https://github.com/forgelang/forgelang)
- [ForgeLang Documentation](https://forgelang.dev)
- [Tree-sitter](https://tree-sitter.github.io)
- [nvim-treesitter](https://github.com/nvim-treesitter/nvim-treesitter)
