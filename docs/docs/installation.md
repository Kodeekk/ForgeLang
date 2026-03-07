---
sidebar_position: 2
---

# Installation

Get ForgeLang up and running on your system in minutes.

## Prerequisites

- **Operating System**: Linux, macOS, or Windows
- **Memory**: At least 512MB RAM
- **Disk Space**: ~100MB for the compiler and standard library

## Installation Methods

### From Source (Recommended)

Building from source gives you the latest version and allows customization.

```bash
# Clone the repository
git clone https://github.com/forgelang/forgelang.git
cd forgelang

# Build and install
make install

# This will:
# - Build both binaries (fl and maul)
# - Install binaries to /usr/local/bin (or $PREFIX/bin)
# - Create symlink to stdlib at ~/.forgelang/stdlib
```

#### Build Options

```bash
# Custom installation prefix
make install PREFIX=/opt/forgelang

# Build in release mode (optimized)
make build-release

# Build in debug mode (with debug symbols)
make build-debug
```

### Uninstall

```bash
make uninstall
```

## Verify Installation

After installation, verify that ForgeLang is working correctly:

```bash
# Check the interpreter version
fl --version

# Check the package manager version
maul --version

# Run a simple test
echo 'println("Hello, ForgeLang!")' | fl
```

## Environment Setup

### Shell Completion

Add shell completions for a better development experience:

```bash
# Bash
fl completion bash >> ~/.bash_completion
maul completion bash >> ~/.bash_completion

# Zsh
fl completion zsh >> ~/.zshrc
maul completion zsh >> ~/.zshrc

# Fish
fl completion fish >> ~/.config/fish/completions/fl.fish
maul completion fish >> ~/.config/fish/completions/maul.fish
```

### IDE/Editor Support

#### Neovim/Vim

A Neovim plugin is included with ForgeLang:

```bash
# Run the setup script
./setup-nvim.sh
```

The plugin provides:
- Syntax highlighting
- Code completion
- Linting
- Formatting

#### VS Code

A VS Code extension is coming soon. For now, use the built-in syntax highlighting or install a community extension.

## Troubleshooting

### Common Issues

#### "Command not found: fl"

Ensure the installation directory is in your PATH:

```bash
# Add to your shell configuration
export PATH="/usr/local/bin:$PATH"
```

#### Permission denied during installation

```bash
# Use sudo for system-wide installation
sudo make install

# Or install to a user directory
make install PREFIX=$HOME/.local
```

#### Build fails with missing dependencies

Ensure you have Rust installed (required for building):

```bash
# Install Rust
curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh

# Then rebuild
make clean
make install
```

## Next Steps

- **[Quick Start](./quick-start)** - Write your first program
- **[Why ForgeLang?](./why-forgelang)** - Learn about the language philosophy
- **[Language Guide](./language-guide/overview)** - Dive into the features
