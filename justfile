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

# CI/CD
ci-local:
    @echo "Running local CI checks..."
    cargo fmt --all -- --check
    cargo clippy --all-targets --all-features -- -D warnings
    cargo test --all-features
    cargo tarpaulin --out Xml --output-dir coverage

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
