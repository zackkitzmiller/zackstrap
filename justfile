# Zackstrap Project Justfile
# Just is a command runner - https://github.com/casey/just

# Default target
default:
    @just --list

# Build the project
build:
    cargo build

# Build release version
release:
    cargo build --release

# Check for compilation errors
check:
    cargo check

# Run tests
test:
    cargo test

# Run tests with output
test-verbose:
    cargo test -- --nocapture

# Run integration tests only
test-integration:
    cargo test --test integration_tests

# Run clippy linter
clippy:
    cargo clippy

# Run clippy with all warnings
clippy-strict:
    cargo clippy -- -D warnings

# Format code
fmt:
    cargo fmt

# Check formatting
fmt-check:
    cargo fmt -- --check

# Clean build artifacts
clean:
    cargo clean

# Install the binary globally
install:
    cargo install --path .

# Uninstall the binary
uninstall:
    cargo uninstall zackstrap

# Run the CLI with basic command
run-basic:
    cargo run -- basic

# Run the CLI with Ruby command
run-ruby:
    cargo run -- ruby

# Run the CLI with auto-detection
run-auto:
    cargo run -- auto

# Run the CLI with interactive mode
run-interactive:
    cargo run -- interactive

# Run the CLI with dry run
run-dry-run:
    cargo run -- --dry-run basic

# Show help
help:
    cargo run -- --help

# Show available commands
list:
    cargo run -- list

# Test all features
test-all: check test clippy fmt-check

# Prepare for release
prepare-release: test-all release

# Run with specific template
run-template:
    cargo run -- ruby --template rails

# Run with target directory
run-target:
    cargo run -- basic --target /tmp/test-project

# Generate documentation
doc:
    cargo doc --open

# Check for security vulnerabilities
audit:
    cargo audit

# Update dependencies
update:
    cargo update

# Show outdated dependencies
outdated:
    cargo outdated

# Run benchmarks (if any)
bench:
    cargo bench

# Install development tools
install-tools:
    cargo install cargo-audit
    cargo install cargo-outdated
    cargo install cargo-watch

# Watch for changes and run tests
watch:
    cargo watch -x check -x test

# Run with cargo watch
dev:
    cargo watch -x run -- basic

# Show project info
info:
    @echo "Zackstrap - Project Configuration Bootstrap Tool"
    @echo "Version: 0.1.0"
    @echo "Rust version: $(rustc --version)"
    @echo "Cargo version: $(cargo --version)"
    @echo "Just version: $(just --version)"
