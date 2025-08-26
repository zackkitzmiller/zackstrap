use crate::error::ZackstrapError;
use super::common::FileGenerator;

impl super::ConfigGenerator {
    pub async fn generate_rust(&self, force: bool) -> Result<(), ZackstrapError> {
        self.generate_rust_with_template(force, "default").await
    }

    pub async fn generate_rust_with_template(
        &self,
        force: bool,
        template: &str,
    ) -> Result<(), ZackstrapError> {
        // Generate basic configs first
        self.generate_basic_with_template(force, false, template).await?;

        // Generate Rust-specific configs
        self.generate_rustfmt_config(force).await?;
        self.generate_clippy_config(force).await?;
        self.generate_cargo_config(force).await?;

        // Overwrite the basic justfile with Rust-specific one
        self.generate_rust_justfile(true, template).await?;

        Ok(())
    }

    pub async fn dry_run_rust_with_template(&self, template: &str) -> Result<(), ZackstrapError> {
        println!("  📝 Would generate .editorconfig");
        println!("  🎨 Would generate .prettierrc (template: {})", template);
        println!("  🔧 Would generate justfile");
        println!("  🦀 Would generate rustfmt.toml");
        println!("  🔍 Would generate .clippy.toml");
        println!("  📦 Would generate .cargo/config.toml");
        println!("  🔧 Would generate Rust justfile (template: {})", template);
        Ok(())
    }

    async fn generate_rustfmt_config(&self, force: bool) -> Result<(), ZackstrapError> {
        let content = r#"# Rustfmt configuration
edition = "2021"
max_width = 100
tab_spaces = 4
newline_style = "Unix"
use_small_heuristics = "Default"
"#;
        self.write_file_if_not_exists("rustfmt.toml", content, force, false).await
    }

    async fn generate_clippy_config(&self, force: bool) -> Result<(), ZackstrapError> {
        let content = r#"# Clippy configuration
# Allow some lints that are too strict for development
allow = [
    "clippy::too_many_arguments",
    "clippy::needless_pass_by_value",
    "clippy::missing_errors_doc",
    "clippy::missing_panics_doc",
]

# Deny some important lints
deny = [
    "clippy::unwrap_used",
    "clippy::expect_used",
    "clippy::panic",
]

# Set some specific configurations
[clippy::all]
"#;
        self.write_file_if_not_exists(".clippy.toml", content, force, false).await
    }

    async fn generate_cargo_config(&self, force: bool) -> Result<(), ZackstrapError> {
        let content = r#"[build]
# Set the target directory
target = "target"

[target.x86_64-unknown-linux-gnu]
rustflags = ["-C", "target-cpu=native"]

[profile.dev]
# Development profile settings
opt-level = 0
debug = true
split-debuginfo = "unpacked"

[profile.release]
# Release profile settings
opt-level = 3
debug = false
lto = true
codegen-units = 1

[profile.test]
# Test profile settings
opt-level = 0
debug = true

[profile.bench]
# Benchmark profile settings
opt-level = 3
debug = false
lto = true
codegen-units = 1
"#;
        self.write_file_if_not_exists(".cargo/config.toml", content, force, false).await
    }

    async fn generate_rust_justfile(
        &self,
        force: bool,
        template: &str,
    ) -> Result<(), ZackstrapError> {
        let content = match template {
            "web" => r#"# Rust web project justfile
default:
    @echo "Available Rust web commands:"
    @just --list

# Run the web server
cargo-web:
    @echo "Running Rust web server..."
    @cargo run

# Build the web server
build:
    @echo "Building Rust web server..."
    @cargo build

# Build for release
build-release:
    @echo "Building Rust web server for release..."
    @cargo build --release

# Run tests
test:
    @echo "Running Rust tests..."
    @cargo test

# Run tests with coverage
test-coverage:
    @echo "Running Rust tests with coverage..."
    @cargo test --no-run
    @cargo llvm-cov --html

# Run linting
lint:
    @echo "Running Clippy..."
    @cargo clippy

# Format code
fmt:
    @echo "Formatting Rust code..."
    @cargo fmt

# Install dependencies
install:
    @echo "Installing Rust dependencies..."
    @cargo build
"#,
            "cli" => r#"# Rust CLI project justfile
default:
    @echo "Available Rust CLI commands:"
    @just --list

# Run the CLI tool
cargo-cli:
    @echo "Running Rust CLI tool..."
    @cargo run

# Build the CLI tool
build:
    @echo "Building Rust CLI tool..."
    @cargo build

# Build for release
build-release:
    @echo "Building Rust CLI tool for release..."
    @cargo build --release

# Install the CLI tool
install:
    @echo "Installing Rust CLI tool..."
    @cargo install --path .

# Run tests
test:
    @echo "Running Rust tests..."
    @cargo test

# Run tests with coverage
test-coverage:
    @echo "Running Rust tests with coverage..."
    @cargo test --no-run
    @cargo llvm-cov --html

# Run linting
lint:
    @echo "Running Clippy..."
    @cargo clippy

# Format code
fmt:
    @echo "Formatting Rust code..."
    @cargo fmt

# Install dependencies
install-deps:
    @echo "Installing Rust dependencies..."
    @cargo build
"#,
            _ => r#"# Rust project justfile
default:
    @echo "Available Rust commands:"
    @just --list

# Run the application
run:
    @echo "Running Rust application..."
    @cargo run

# Build the application
build:
    @echo "Building Rust application..."
    @cargo build

# Build for release
build-release:
    @echo "Building Rust application for release..."
    @cargo build --release

# Run tests
test:
    @echo "Running Rust tests..."
    @cargo test

# Run tests with coverage
test-coverage:
    @echo "Running Rust tests with coverage..."
    @cargo test --no-run
    @cargo llvm-cov --html

# Run linting
lint:
    @echo "Running Clippy..."
    @cargo clippy

# Format code
fmt:
    @echo "Formatting Rust code..."
    @cargo fmt

# Install dependencies
install:
    @echo "Installing Rust dependencies..."
    @cargo build
"#,
        };
        self.write_file_if_not_exists("justfile", content, force, false).await
    }
}
