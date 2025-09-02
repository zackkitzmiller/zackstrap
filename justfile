# Zackstrap Project Justfile
# Just is a command runner - https://github.com/casey/just

default:
    just info

# Development
rust-version:
    rustc --version

cargo-build:
    cargo build

cargo-build-release:
    cargo build --release

release-build:
    @echo "Building release version..."
    @if ! cargo get version >/dev/null 2>&1; then \
        echo "cargo-get not found, installing tools..."; \
        just install-tools; \
    fi
    @echo "Building version: $(cargo get version 2>/dev/null || echo 'unknown')"
    cargo build --release
    @echo "Creating release directory..."
    mkdir -p dist
    @echo "Copying binary to dist/..."
    cp target/release/zackstrap dist/
    @echo "Creating zipfile..."
    cd dist && zip -r "zackstrap-$(cargo get version 2>/dev/null || echo 'unknown')-$(uname -s | tr '[:upper:]' '[:lower:]')-$(uname -m).zip" zackstrap
    @echo "Release build complete!"
    @echo "Binary: dist/zackstrap"
    @echo "Zipfile: dist/zackstrap-$(cargo get version 2>/dev/null || echo 'unknown')-$(uname -s | tr '[:upper:]' '[:lower:]')-$(uname -m).zip"

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
    cargo install cargo-get
    cargo install cargo-set-version
    cargo install cargo-audit
    cargo install cargo-outdated
    cargo install cargo-watch
    cargo install cargo-tarpaulin --version 0.32.8


check-deps:
    @echo "Checking dependencies..."
    cargo outdated || echo "Dependencies check completed (some may be outdated)"

check-deps-json:
    @echo "Checking dependencies and saving to JSON..."
    cargo outdated --format json > outdated-deps.json || echo "Dependencies check completed (some may be outdated)"
    @echo "Results saved to outdated-deps.json"

check-deps-table:
    @echo "Checking dependencies in table format..."
    cargo outdated --format list || echo "Dependencies check completed (some may be outdated)"

check-tools:
    @echo "Development Tools Status:"
    @echo "========================"
    @echo "cargo-get: $(cargo get --version 2>/dev/null || echo 'not installed')"
    @echo "cargo-set-version: $(cargo set-version --version 2>/dev/null || echo 'not installed')"
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
    @if ! cargo get version >/dev/null 2>&1; then \
        echo "cargo-get not found, installing tools..."; \
        just install-tools; \
    fi
    @VERSION=$$(cargo get version 2>/dev/null || echo "unknown"); \
    git tag -a "v$$VERSION" -m "Release v$$VERSION"
    git push origin main
    git push origin "v$$VERSION"

release-minor:
    @echo "Creating minor release..."
    cargo set-version --bump minor
    git add Cargo.toml Cargo.lock
    git commit -m "Bump version for minor release"
    @if ! cargo get version >/dev/null 2>&1; then \
        echo "cargo-get not found, installing tools..."; \
        just install-tools; \
    fi
    @VERSION=$$(cargo get version 2>/dev/null || echo "unknown"); \
    git tag -a "v$$VERSION" -m "Release v$$VERSION"
    git push origin main
    git push origin "v$$VERSION"

release-major:
    @echo "Creating major release..."
    cargo set-version --bump major
    git add Cargo.toml Cargo.lock
    git commit -m "Bump version for major release"
    @if ! cargo get version >/dev/null 2>&1; then \
        echo "cargo-get not found, installing tools..."; \
        just install-tools; \
    fi
    @VERSION=$$(cargo get version 2>/dev/null || echo "unknown"); \
    git tag -a "v$$VERSION" -m "Release v$$VERSION"
    git push origin main
    git push origin "v$$VERSION"

# Project info
info:
    @echo "Zackstrap Project Information"
    @echo "============================"
    @echo "Rust version: $(rustc --version)"
    @echo "Cargo version: $(cargo --version)"
    @echo "Just version: $(just --version)"
    @if ! cargo get version >/dev/null 2>&1; then \
        echo "cargo-get not found, installing tools..."; \
        just install-tools; \
    fi
    @echo "Current version: $(cargo get version 2>/dev/null || echo 'unknown')"
    @echo ""
    @echo "Development Tools Status:"
    @echo "========================"
    @echo "cargo-get: $(cargo get --version 2>/dev/null || echo 'not installed')"
    @echo "cargo-set-version: $(cargo set-version --version 2>/dev/null || echo 'not installed')"
    @echo "cargo-audit: $(cargo audit --version 2>/dev/null || echo 'not installed')"
    @echo "cargo-outdated: $(cargo outdated --version 2>/dev/null || echo 'not installed')"
    @echo "cargo-watch: $(cargo watch --version 2>/dev/null || echo 'not installed')"
    @echo "cargo-tarpaulin: $(cargo tarpaulin --version 2>/dev/null || echo 'not installed')"
