# Zackstrap

[![CI/CD Pipeline](https://github.com/zackkitzmiller/zackstrap/actions/workflows/ci.yml/badge.svg?branch=main)](https://github.com/zackkitzmiller/zackstrap/actions/workflows/ci.yml)
[![Test Coverage](https://codecov.io/gh/zackkitzmiller/zackstrap/graph/badge.svg?token=LL69KNNRL0)](https://codecov.io/gh/zackkitzmiller/zackstrap)
[![Crates.io](https://img.shields.io/crates/v/zackstrap.svg)](https://crates.io/crates/zackstrap)
[![License: MIT](https://img.shields.io/badge/License-MIT-yellow.svg)](https://opensource.org/licenses/MIT)

A powerful Rust CLI tool to bootstrap project configuration files. Quickly generate common project configuration files like `.editorconfig`, `.prettierrc`, and Ruby-specific configurations.

## Features

- 🚀 **Fast**: Built in Rust for blazing-fast performance
- 🎯 **Smart**: Automatically detects project types and generates appropriate configs
- 🔒 **Safe**: Built-in safety checks and error handling
- 🎨 **Beautiful**: Colored output and progress indicators
- 🧪 **Tested**: Comprehensive test coverage
- 🔍 **Auto-detection**: Automatically detects Ruby projects and generates appropriate configs
- 🎯 **Interactive**: Guided setup with user prompts
- 🎨 **Templates**: Multiple configuration templates for different coding styles
- 👀 **Dry Run**: Preview what would be created without actually creating files

## Installation

### From Source

```bash
git clone https://github.com/yourusername/zackstrap.git
cd zackstrap
cargo build --release
cargo install --path .
```

### Using Cargo

```bash
cargo install zackstrap
```

## Usage

### Basic Project Configuration

Generate basic configuration files (`.editorconfig`, `.prettierrc`):

```bash
zackstrap basic
```

### Ruby Project Configuration

Generate Ruby-specific configuration files:

```bash
zackstrap ruby
```

This will create:

```md
- .editorconfig
- .prettierrc
- .ruby-version (set to 3.3.0)
- .node-version (set to 24)
- .rubocop.yml  (comprehensive configuration)
- package.json  (with prettier-plugin-ruby)
```

### Python Project Configuration

Generate Python-specific configuration files:

```bash
zackstrap python
```

This will create:

```bash
- .editorconfig
- .prettierrc
- .python-version (set to 3.12)
- pyproject.toml (with black, flake8, mypy)
- .flake8 configuration
- requirements-dev.txt
- justfile with Python development tasks
```

### Node.js Project Configuration

Generate Node.js-specific configuration files:

```bash
zackstrap node
```

This will create:

```bash
- .editorconfig
- .prettierrc
- .nvmrc (set to Node.js 20)
- .eslintrc.js (ESLint configuration)
- package.json (with development dependencies)
- justfile with Node.js development tasks
```

### Go Project Configuration

Generate Go-specific configuration files:

```bash
zackstrap go
```

This will create:

```bash
- .editorconfig
- .prettierrc
- go.mod (Go module file)
- .golangci.yml (golangci-lint configuration)
- .gitignore additions for Go
- justfile with Go development tasks
```

### Rust Project Configuration

Generate Rust-specific configuration files:

```bash
zackstrap rust
```

This will create:

```bash
- .editorconfig
- .prettierrc
- rustfmt.toml (Rust formatting configuration)
- .clippy.toml (Clippy linting configuration)
- .cargo/config.toml (Cargo configuration)
- justfile with Rust development tasks
```

### Force Overwrite

Use the `--force` flag to overwrite existing files:

```bash
zackstrap basic --force
zackstrap ruby --force
```

### Target Directory

Specify a different target directory:

```bash
zackstrap basic --target /path/to/project
zackstrap ruby --target /path/to/project
```

### List Available Configurations

See what configuration files are available:

```bash
zackstrap list
```

### Auto-Detection

Automatically detect project type and generate appropriate configs:

```bash
zackstrap auto
```

### Interactive Mode

Guided setup with user prompts:

```bash
zackstrap interactive
```

### Template System

Use different configuration templates:

```bash
# Basic projects with different styles
zackstrap basic --template google
zackstrap basic --template airbnb

# Ruby projects with framework-specific configs
zackstrap ruby --template rails
zackstrap ruby --template sinatra
zackstrap ruby --template gem

# Python projects with framework-specific configs
zackstrap python --template django
zackstrap python --template flask

# Node.js projects with framework-specific configs
zackstrap node --template express
zackstrap node --template react

# Go projects with project type configs
zackstrap go --template web
zackstrap go --template cli

# Rust projects with project type configs
zackstrap rust --template web
zackstrap rust --template cli
```

### Dry Run Mode

Preview what would be created without actually creating files:

```bash
# Note: --dry-run must come BEFORE the subcommand
zackstrap --dry-run basic
zackstrap --dry-run ruby --template rails
zackstrap --dry-run auto
```

## Generated Files

### Basic Projects

- `.editorconfig` - Multi-language editor configuration
- `.prettierrc` - Prettier formatting rules
- `justfile` - Project automation and development tasks

### Ruby Projects (includes basic +)

- `.ruby-version` - Ruby 3.3.0
- `.node-version` - Node.js 24
- `.rubocop.yml` - Comprehensive Ruby linting
- `package.json` - With `prettier-plugin-ruby`
- `justfile` - Ruby-specific automation tasks

### Python Projects (includes basic +)

- `.python-version` - Python 3.12
- `pyproject.toml` - Project configuration with black, flake8, mypy
- `.flake8` - Flake8 linting configuration
- `requirements-dev.txt` - Development dependencies
- `justfile` - Python-specific automation tasks

### Node.js Projects (includes basic +)

- `.nvmrc` - Node.js 20
- `.eslintrc.js` - ESLint configuration
- `package.json` - With development dependencies
- `justfile` - Node.js-specific automation tasks

### Go Projects (includes basic +)

- `go.mod` - Go module file
- `.golangci.yml` - golangci-lint configuration
- `.gitignore` - Go-specific ignore patterns
- `justfile` - Go-specific automation tasks

### Rust Projects (includes basic +)

- `rustfmt.toml` - Rust formatting configuration
- `.clippy.toml` - Clippy linting configuration
- `.cargo/config.toml` - Cargo configuration
- `justfile` - Rust-specific automation tasks

## Configuration Files

### .editorconfig

- Root settings for consistent coding style
- File-specific overrides for Ruby, JavaScript, TypeScript
- UTF-8 encoding, LF line endings
- 2-space indentation

### .prettierrc

- Semi-colons enabled
- Single quotes
- 2-space tab width
- 80 character print width
- ES5 trailing comma style

**Templates available:**

- **default**: Standard configuration
- **google**: Google style (double quotes, 80 char width)
- **airbnb**: Airbnb style (single quotes, 100 char width)

### .ruby-version

- Set to Ruby 3.3.0 (latest stable)

### .node-version

- Set to Node.js 24 (latest LTS)

### .rubocop.yml

- Comprehensive Ruby linting rules
- RSpec-specific configurations
- Performance and security checks
- Bundler integration

**Templates available:**

- **default**: Standard Ruby configuration
- **rails**: Rails-specific rules and exclusions
- **sinatra**: Lightweight Sinatra configuration
- **gem**: Gem development with stricter rules

### package.json

- Prettier with Ruby plugin
- Development dependencies only

**Templates available:**

- **default**: Basic Ruby project setup
- **rails**: Rails app with ESLint and additional tools
- **sinatra**: Sinatra app setup
- **gem**: Gem development with RSpec

### Python Configuration Files

#### .python-version

- Set to Python 3.12 (latest stable)

#### pyproject.toml

- Black formatting configuration (88 char width)
- Flake8 linting configuration
- MyPy type checking configuration
- Pytest configuration

**Templates available:**

- **default**: Standard Python configuration
- **django**: Django-specific settings and stubs
- **flask**: Flask app configuration

#### .flake8

- 88 character line length (compatible with Black)
- Extended ignore patterns for common issues

#### requirements-dev.txt

- Black (code formatter)
- Flake8 (linter)
- MyPy (type checker)
- Pytest (testing framework)

### Node.js Configuration Files

#### .nvmrc

- Set to Node.js 20 (LTS version)

#### .eslintrc.js

- ES2022 environment support
- Recommended ESLint rules
- Framework-specific configurations

**Templates available:**

- **default**: Standard Node.js configuration
- **express**: Express.js with console logging allowed
- **react**: React with JSX support and relaxed prop-types

#### package.json

- ESLint and Prettier development dependencies
- Framework-specific dependencies based on template

### Go Configuration Files

#### go.mod

- Go 1.21 module configuration
- Project module name

#### .golangci.yml

- golangci-lint configuration
- Common linters enabled (gofmt, golint, govet, errcheck)
- 5-minute timeout for large projects

#### .gitignore

- Go-specific ignore patterns
- Binary files, test files, and workspace files

### Rust Configuration Files

#### rustfmt.toml

- Rust 2021 edition
- 100 character line width

#### .clippy.toml

- Clippy linting configuration
- Customizable rules and settings

#### .cargo/config.toml

- Cargo build configuration
- Native CPU optimization flags

### justfile

- Project automation and development tasks
- Common development commands for all languages
- Language-specific commands and workflows
- Framework-specific commands and tasks

**Templates available:**

- **Basic**: Standard development tasks (build, test, format, lint)
- **Ruby**: Ruby development with bundler and tools
  - **Rails**: Rails-specific commands (server, console, routes, db)
  - **Sinatra**: Sinatra development commands
  - **Gem**: Gem development and release commands
- **Python**: Python development with virtual environments
  - **Django**: Django-specific commands (server, migrate, shell)
  - **Flask**: Flask development commands
- **Node.js**: Node.js development with npm/yarn
  - **Express**: Express.js development commands
  - **React**: React development and build commands
- **Go**: Go development with go commands
  - **Web**: Web application commands
  - **CLI**: Command-line application commands
- **Rust**: Rust development with cargo
  - **Web**: Web application commands
  - **CLI**: Command-line application commands

## Development

### Prerequisites

- Rust 1.70+
- Cargo

### Building

```bash
cargo build
```

### Testing

```bash
cargo test
```

### Running

```bash
cargo run -- basic
cargo run -- ruby
```

## Project Structure

```bash
zackstrap/
├── src/
│   ├── main.rs           # CLI entry point
│   ├── lib.rs               # Library exports
│   ├── config.rs         # Configuration structures
│   ├── error.rs           # Error handling
│   └── generators.rs  # File generation logic
├── tests/
│   └── integration_tests.rs
├── Cargo.toml
└── README.md
```

## Contributing

1. Fork the repository
2. Create a feature branch
3. Make your changes
4. Add tests
5. Run the test suite
6. Submit a pull request

## License

MIT License - see LICENSE file for details.

## Author

Zack Kitzmiller - [GitHub](https://github.com/yourusername)
