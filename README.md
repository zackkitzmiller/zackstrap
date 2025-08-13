# Zackstrap

A powerful Rust CLI tool to bootstrap project configuration files. Quickly generate common project configuration files like `.editorconfig`, `.prettierrc`, and Ruby-specific configurations.

## Features

- 🚀 **Fast**: Built in Rust for blazing-fast performance
- 🎯 **Smart**: Automatically detects project types and generates appropriate configs
- 🔒 **Safe**: Built-in safety checks and error handling
- 🎨 **Beautiful**: Colored output and progress indicators
- 🧪 **Tested**: Comprehensive test coverage

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

```bash
- .editorconfig
- .prettierrc
- .ruby-version (set to 3.3.0)
- .node-version (set to 24)
- .rubocop.yml  (comprehensive configuration)
- package.json  (with prettier-plugin-ruby)
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

### .ruby-version

- Set to Ruby 3.3.0 (latest stable)

### .node-version

- Set to Node.js 24 (latest LTS)

### .rubocop.yml

- Comprehensive Ruby linting rules
- RSpec-specific configurations
- Performance and security checks
- Bundler integration

### package.json

- Prettier with Ruby plugin
- Development dependencies only

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
