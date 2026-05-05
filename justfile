# Zackstrap — project command runner
# https://github.com/casey/just

set dotenv-load := false

version := `cargo get package.version 2>/dev/null || echo 'unknown'`
os := `uname -s | tr '[:upper:]' '[:lower:]'`
arch := `uname -m`

# ─── Default ──────────────────────────────────────────────────────────

default:
    @just --list --unsorted

# ─── Development ──────────────────────────────────────────────────────

# Build debug binary
build:
    cargo build

# Build optimized release binary
build-release:
    cargo build --release

# Install zackstrap to ~/.cargo/bin
install:
    cargo install --path .

# Run the CLI with arguments (e.g. `just run -- ruby --template rails`)
run *ARGS:
    cargo run -- {{ARGS}}

# Watch for changes and rebuild on save
watch:
    cargo watch -x check -x 'test -- --nocapture'

# Check compilation without producing binaries
check:
    cargo check --all-targets --all-features

# ─── Testing ──────────────────────────────────────────────────────────

# Run all tests
test:
    cargo test

# Run a single test by name
test-one NAME:
    cargo test {{NAME}} -- --nocapture

# Run only unit tests
test-unit:
    cargo test -p zackstrap --test unit_tests

# Run only integration tests
test-integration:
    cargo test -p zackstrap --test integration_tests

# Run only CLI argument parsing tests
test-cli:
    cargo test -p zackstrap --test cli_tests

# Run only end-to-end tests
test-e2e:
    cargo test -p zackstrap --test e2e_tests

# Run only fail-on-exists tests
test-fail-on-exists:
    cargo test -p zackstrap --test fail_on_exists_tests

# Run only generator tests
test-generators:
    cargo test -p zackstrap --test generators_tests

# Run tests with stdout visible
test-verbose:
    cargo test -- --nocapture

# Generate HTML coverage report via tarpaulin
test-coverage:
    cargo tarpaulin --out Html --output-directory coverage
    @echo "Report: coverage/tarpaulin-report.html"

# Generate coverage and fail if below threshold
test-coverage-check THRESHOLD="80":
    cargo tarpaulin --fail-under {{THRESHOLD}}

# ─── Linting & Formatting ────────────────────────────────────────────

# Format all source files
fmt:
    cargo fmt --all

# Check formatting without modifying files
fmt-check:
    cargo fmt --all -- --check

# Run clippy with warnings-as-errors
clippy:
    cargo clippy --all-targets --all-features -- -D warnings

# Run clippy and auto-fix what it can
clippy-fix:
    cargo clippy --all-targets --all-features --fix --allow-dirty -- -D warnings

# Lint everything (format check + clippy)
lint: fmt-check clippy

# Fix everything (format + clippy auto-fix)
fix: fmt clippy-fix

# ─── Security & Dependencies ─────────────────────────────────────────

# Audit dependencies for known vulnerabilities
audit:
    cargo audit

# Show outdated dependencies
outdated:
    cargo outdated

# Show the full dependency tree
deps:
    cargo tree

# Show duplicate dependencies
deps-dupes:
    cargo tree -d

# Update all dependencies to latest compatible versions
deps-update:
    cargo update

# ─── Pre-commit & CI ─────────────────────────────────────────────────

# Quick pre-commit checks (format + clippy + check)
pre-commit: fmt clippy check

# Full local CI pipeline (lint + all tests)
ci-local: lint test

# CI lint stage (matches GitHub Actions)
ci-lint:
    @echo "CI: lint + format check"
    cargo fmt --all -- --check
    cargo clippy --all-targets --all-features -- -D warnings

# CI test stage (matches GitHub Actions)
ci-test:
    @echo "CI: tests + coverage"
    cargo test --all-features
    cargo tarpaulin --out Xml --output-dir coverage

# ─── Release ──────────────────────────────────────────────────────────

# Bump version, tag, push, and publish to crates.io
release LEVEL:
    #!/usr/bin/env bash
    set -euo pipefail
    if [[ "{{LEVEL}}" != "patch" && "{{LEVEL}}" != "minor" && "{{LEVEL}}" != "major" ]]; then
        echo "Usage: just release [patch|minor|major]"
        exit 1
    fi
    echo "Creating {{LEVEL}} release..."
    cargo set-version --bump {{LEVEL}}
    cargo check
    git add Cargo.toml Cargo.lock
    git commit -m "Bump version for {{LEVEL}} release"
    VERSION="$(cargo get package.version)"
    git tag -a "v${VERSION}" -m "Release v${VERSION}"
    echo "Pushing to origin..."
    git push origin main
    git push origin "v${VERSION}"
    echo "Publishing to crates.io..."
    cargo publish
    echo "Released v${VERSION}"

# Build a distributable zipfile for the current platform
dist:
    #!/usr/bin/env bash
    set -euo pipefail
    echo "Building release binary..."
    cargo build --release
    mkdir -p dist
    cp target/release/zackstrap dist/
    ZIPNAME="zackstrap-{{version}}-{{os}}-{{arch}}.zip"
    cd dist && zip -r "$ZIPNAME" zackstrap
    echo "dist/$ZIPNAME"

# Dry-run a crates.io publish (validates packaging without uploading)
publish-dry-run:
    cargo publish --dry-run

# ─── Cleanup ──────────────────────────────────────────────────────────

# Remove build artifacts
clean:
    cargo clean

# Remove build artifacts and coverage reports
clean-all: clean
    rm -rf coverage dist

# ─── Project Info ─────────────────────────────────────────────────────

# Show project and toolchain versions
info:
    @echo "Zackstrap v{{version}}"
    @echo ""
    @echo "Toolchain"
    @echo "  rustc:  $(rustc --version)"
    @echo "  cargo:  $(cargo --version)"
    @echo "  just:   $(just --version)"
    @echo ""
    @echo "Dev Tools"
    @echo "  cargo-get:         $(cargo get --version 2>/dev/null || echo 'not installed')"
    @echo "  cargo-set-version: $(cargo set-version --version 2>/dev/null || echo 'not installed')"
    @echo "  cargo-audit:       $(cargo audit --version 2>/dev/null || echo 'not installed')"
    @echo "  cargo-outdated:    $(cargo outdated --version 2>/dev/null || echo 'not installed')"
    @echo "  cargo-watch:       $(cargo watch --version 2>/dev/null || echo 'not installed')"
    @echo "  cargo-tarpaulin:   $(cargo tarpaulin --version 2>/dev/null || echo 'not installed')"

# Show lines of code
loc:
    @echo "Source:"
    @find src -name '*.rs' | xargs wc -l | tail -1
    @echo "Tests:"
    @find tests -name '*.rs' | xargs wc -l | tail -1

# Install all development tools
install-tools:
    cargo install cargo-get cargo-set-version cargo-audit cargo-outdated cargo-watch
    cargo install cargo-tarpaulin --version 0.32.8

# Show disk usage of build artifacts and caches
cache-status:
    @echo "Disk Usage"
    @echo "  target/:         $(du -sh target 2>/dev/null | cut -f1 || echo 'n/a')"
    @echo "  coverage/:       $(du -sh coverage 2>/dev/null | cut -f1 || echo 'n/a')"
    @echo "  dist/:           $(du -sh dist 2>/dev/null | cut -f1 || echo 'n/a')"
    @echo "  ~/.cargo/registry: $(du -sh ~/.cargo/registry 2>/dev/null | cut -f1 || echo 'n/a')"

# ─── Dogfooding ───────────────────────────────────────────────────────

# Generate configs into a temp directory (test the CLI output)
try PROFILE="ruby" *ARGS="":
    #!/usr/bin/env bash
    set -euo pipefail
    DIR=$(mktemp -d)
    echo "Target: $DIR"
    cargo run -- --target "$DIR" {{PROFILE}} {{ARGS}}
    echo ""
    echo "Generated files:"
    ls -la "$DIR"
    echo ""
    echo "Cleaning up..."
    rm -rf "$DIR"

# Dry-run a profile to preview output
preview PROFILE="ruby" *ARGS="":
    cargo run -- --dry-run {{PROFILE}} {{ARGS}}
