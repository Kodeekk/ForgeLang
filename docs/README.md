# ForgeLang Documentation

Comprehensive documentation for the ForgeLang programming language.

## Structure

```
docs/
├── index.html              # Static HTML documentation (Material 3 design)
├── package.json            # Node.js dependencies (optional Docusaurus)
├── docusaurus.config.ts    # Docusaurus configuration
├── sidebars.ts             # Navigation sidebar configuration
├── tsconfig.json           # TypeScript configuration
├── src/
│   └── css/
│       └── custom.css      # Material 3 custom styles
├── static/
│   └── img/
│       ├── logo.svg        # ForgeLang logo (light mode)
│       └── logo-dark.svg   # ForgeLang logo (dark mode)
├── docs/
│   ├── intro.md                    # Welcome page
│   ├── installation.md             # Installation guide
│   ├── quick-start.md              # Quick start tutorial
│   ├── why-forgelang.md            # Language philosophy
│   ├── examples.md                 # Code examples
│   ├── language-guide/             # Language features
│   │   ├── overview.md
│   │   ├── syntax.md
│   │   ├── types.md
│   │   ├── variables.md
│   │   ├── operators.md
│   │   ├── control-flow.md
│   │   ├── functions.md
│   │   └── modules.md
│   ├── advanced/                   # Advanced features
│   │   ├── generics.md
│   │   └── enums.md
│   ├── stdlib/                     # Standard library guides
│   │   ├── overview.md
│   │   ├── io.md
│   │   ├── math.md
│   │   ├── list.md
│   │   ├── str.md
│   │   ├── bool.md
│   │   ├── int.md
│   │   ├── float.md
│   │   ├── fs.md
│   │   ├── time.md
│   │   └── env.md
│   └── api/                        # API reference
│       ├── overview.md
│       └── io.md
└── blog/                   # Blog posts (optional)
```

## Quick Start

### View Static Documentation

Open `index.html` in your browser:

```bash
# Linux
xdg-open index.html

# macOS
open index.html

# Windows
start index.html
```

### Using Docusaurus (Optional)

If you want to use the full Docusaurus build:

```bash
# Install dependencies
npm install

# Start development server
npm start

# Build for production
npm run build

# Serve production build
npm run serve
```

## Documentation Content

### Introduction
- Welcome to ForgeLang
- Installation instructions
- Quick start guide
- Why ForgeLang?

### Language Guide
- Syntax basics
- Types and type system
- Variables and scope
- Operators
- Control flow (if, match, for, while)
- Functions and closures
- Modules and imports

### Advanced Features
- Generics
- Enums with associated data
- Interfaces
- Pattern matching
- Error handling with Result/Option

### Standard Library
- `std.io` - Input/Output
- `std.math` - Mathematics
- `std.list` - List operations
- `std.str` - String manipulation
- `std.bool` - Boolean operations
- `std.int` - Integer operations
- `std.float` - Float operations
- `std.fs` - Filesystem
- `std.time` - Time utilities
- `std.env` - Environment

## Design Principles

The documentation follows Material Design 3 principles:

1. **Clean and Modern** - Fresh, contemporary design
2. **Accessible** - Works for everyone
3. **Responsive** - Looks great on all devices
4. **Dark Mode** - Easy on the eyes
5. **Fast** - Quick to load and navigate

## Contributing

To contribute to the documentation:

1. Fork the repository
2. Make your changes in the `docs/` folder
3. Test locally
4. Submit a pull request

## License

Documentation is released under the same license as ForgeLang (MIT).
