# Zackstrap Project Justfile
# Just is a command runner - https://github.com/casey/just

default:
  just info

# Development
rust-version:
  just info

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

test:
  cargo test

test-coverage:
  cargo install cargo-tarpaulin --version 0.32.8
  cargo tarpaulin --out Html --output-directory coverage

check:
  cargo check

fmt:
  cargo fmt --all

fmt-check:
  cargo fmt --all -- --check

clippy:
  cargo clippy --all-targets --all-features -- -D warnings

clean:
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
  cargo install cargo-get || echo "cargo-get already installed"
  cargo install cargo-set-version || echo "cargo-set-version already installed"
  cargo install cargo-audit || echo "cargo-audit already installed"
  cargo install cargo-outdated || echo "cargo-outdated already installed"
  cargo install cargo-watch || echo "cargo-watch already installed"
  cargo install cargo-tarpaulin --version 0.32.8 || echo "cargo-tarpaulin already installed"


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
  @if ! cargo get --version >/dev/null 2>&1; then \
    echo "cargo-get not found, installing tools..."; \
    just install-tools; \
  fi
  @VERSION=$$(cargo get --version 2>/dev/null || echo "unknown"); \
  git tag -a "v$$VERSION" -m "Release v$$VERSION"
  git push origin main
  git push origin "v$$VERSION"

release-minor:
  @echo "Creating minor release..."
  cargo set-version --bump minor
  git add Cargo.toml Cargo.lock
  git commit -m "Bump version for minor release"
  @if ! cargo get --version >/dev/null 2>&1; then \
    echo "cargo-get not found, installing tools..."; \
    just install-tools; \
  fi
  @VERSION=$$(cargo get --version 2>/dev/null || echo "unknown"); \
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

# Cache management
clear-cache:
    @echo "Clearing all caches..."
    @echo "Clearing cargo cache..."
    @rm -rf ~/.cargo/registry ~/.cargo/git target
    @echo "Clearing development tools cache..."
    @rm -rf ~/.cargo/bin
    @echo "Clearing just cache..."
    @rm -rf ~/.local/bin
    @echo "✅ All caches cleared!"

clear-cache-cargo:
    @echo "Clearing cargo cache..."
    @rm -rf ~/.cargo/registry ~/.cargo/git target
    @echo "✅ Cargo cache cleared!"

clear-cache-tools:
    @echo "Clearing development tools cache..."
    @rm -rf ~/.cargo/bin
    @echo "✅ Development tools cache cleared!"

clear-cache-just:
    @echo "Clearing just cache..."
    @rm -rf ~/.local/bin
    @echo "✅ Just cache cleared!"

cache-status:
    @echo "Cache Status"
    @echo "============"
    @echo "Cargo registry: $(du -sh ~/.cargo/registry 2>/dev/null || echo 'not found')"
    @echo "Cargo git: $(du -sh ~/.cargo/git 2>/dev/null || echo 'not found')"
    @echo "Target directory: $(du -sh target 2>/dev/null || echo 'not found')"
    @echo "Cargo bin: $(du -sh ~/.cargo/bin 2>/dev/null || echo 'not found')"
    @echo "Just bin: $(du -sh ~/.local/bin 2>/dev/null || echo 'not found')"
    @echo ""
    @echo "Development Tools:"
    @echo "cargo-get: $(cargo get --version 2>/dev/null || echo 'not installed')"
    @echo "cargo-set-version: $(cargo set-version --version 2>/dev/null || echo 'not installed')"
    @echo "cargo-audit: $(cargo audit --version 2>/dev/null || echo 'not installed')"
    @echo "cargo-outdated: $(cargo outdated --version 2>/dev/null || echo 'not installed')"
    @echo "cargo-watch: $(cargo watch --version 2>/dev/null || echo 'not installed')"
    @echo "cargo-tarpaulin: $(cargo tarpaulin --version 2>/dev/null || echo 'not installed')"

# Project info
info:
  @echo "Zackstrap Project Information"
  @echo "============================"
  @echo "Rust version: $(rustc --version)"
  @echo "Cargo version: $(cargo --version)"
  @echo "Just version: $(just --version)"
  @if ! cargo get --version >/dev/null 2>&1; then \
    echo "cargo-get not found, installing tools..."; \
    just install-tools; \
  fi
  @echo "Current version: $(cargo get --version 2>/dev/null || echo 'unknown')"
  @echo ""
  @echo "Development Tools Status:"
  @echo "========================"
  @echo "cargo-get: $(cargo get --version 2>/dev/null || echo 'not installed')"
  @echo "cargo-set-version: $(cargo set-version --version 2>/dev/null || echo 'not installed')"
  @echo "cargo-audit: $(cargo audit --version 2>/dev/null || echo 'not installed')"
  @echo "cargo-outdated: $(cargo outdated --version 2>/dev/null || echo 'not installed')"
  @echo "cargo-watch: $(cargo watch --version 2>/dev/null || echo 'not installed')"
  @echo "cargo-tarpaulin: $(cargo tarpaulin --version 2>/dev/null || echo 'not installed')"
