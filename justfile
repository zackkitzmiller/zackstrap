# Zackstrap Project Justfile
# Just is a command runner - https://github.com/casey/just

# Development
rust-version:
    rustc --version

cargo-build:
    cargo build

cargo-build-release:
    cargo build --release

cargo-test:
    cargo test

cargo-test-coverage:
    cargo install cargo-tarpaulin --version 0.32.8
    cargo tarpaulin --out Html

cargo-check:
    cargo check

cargo-fmt:
    cargo fmt --all

cargo-fmt-check:
    cargo fmt --all -- --check

cargo-clippy:
    cargo clippy --all-targets --all-features -- -D warnings

cargo-clean:
    cargo clean

# CI/CD Stages
# Note: These commands require 'just' to be installed in CI
# The CI workflows automatically install 'just' using taiki-e/install-action@just
ci-lint-format:
    @echo "Running lint and format checks..."
    cargo fmt --all -- --check
    cargo clippy --all-targets --all-features -- -D warnings

ci-test:
    @echo "Running tests and coverage..."
    cargo test --all-features
    cargo tarpaulin --out Xml --output-dir coverage

ci-local:
    @echo "Running full local CI pipeline..."
    just ci-lint-format
    just ci-test

# Quick checks
quick-check:
    @echo "Quick code check..."
    cargo check --all-targets --all-features

quick-fmt:
    @echo "Formatting code..."
    cargo fmt --all

quick-lint:
    @echo "Running clippy..."
    cargo clippy --all-targets --all-features -- -D warnings

# Pre-commit checks
pre-commit:
    @echo "Running pre-commit checks..."
    just quick-fmt
    just quick-lint
    just quick-check

# Development tools
install-tools:
    @echo "Installing development tools..."
    cargo install cargo-audit
    cargo install cargo-outdated
    cargo install cargo-watch
    cargo install cargo-tarpaulin --version 0.32.8

check-deps:
    @echo "Checking dependencies..."
    cargo outdated

check-tools:
    @echo "Development Tools Status:"
    @echo "========================"
    @echo "cargo-audit: $(cargo audit --version 2>/dev/null || echo 'not installed')"
    @echo "cargo-outdated: $(cargo outdated --version 2>/dev/null || echo 'not installed')"
    @echo "cargo-watch: $(cargo watch --version 2>/dev/null || echo 'not installed')"
    @echo "cargo-tarpaulin: $(cargo tarpaulin --version 2>/dev/null || echo 'not installed')"

# Release
release-patch:
    @echo "Creating patch release..."
    cargo set-version --bump patch
    git add Cargo.toml Cargo.lock
    git commit -m "Bump version for patch release"
    git tag -a "v$(cargo get version)" -m "Release v$(cargo get version)"
    git push origin main
    git push origin "v$(cargo get version)"

release-minor:
    @echo "Creating minor release..."
    cargo set-version --bump minor
    git add Cargo.toml Cargo.lock
    git commit -m "Bump version for minor release"
    git tag -a "v$(cargo get version)" -m "Release v$(cargo get version)"
    git push origin main
    git push origin "v$(cargo get version)"

release-major:
    @echo "Creating major release..."
    cargo set-version --bump major
    git add Cargo.toml Cargo.lock
    git commit -m "Bump version for major release"
    git tag -a "v$(cargo get version)" -m "Release v$(cargo get version)"
    git push origin main
    git push origin "v$(cargo get version)"

# Project info
info:
    @echo "Zackstrap Project Information"
    @echo "============================"
    @echo "Rust version: $(rustc --version)"
    @echo "Cargo version: $(cargo --version)"
    @echo "Just version: $(just --version)"
    @echo "Current version: $(cargo get version)"
    @echo ""
    @echo "Development Tools Status:"
    @echo "========================"
    @echo "cargo-audit: $(cargo audit --version 2>/dev/null || echo 'not installed')"
    @echo "cargo-outdated: $(cargo outdated --version 2>/dev/null || echo 'not installed')"
    @echo "cargo-watch: $(cargo watch --version 2>/dev/null || echo 'not installed')"
    @echo "cargo-tarpaulin: $(cargo tarpaulin --version 2>/dev/null || echo 'not installed')"
