# CLAUDE.md

This file provides guidance to Claude Code (claude.ai/code) when working with code in this repository.

## Project Overview

Zackstrap is a Rust CLI tool (published on crates.io) that bootstraps project
configuration files (.editorconfig, .prettierrc, .rubocop.yml, etc.) for
multiple language ecosystems: Basic, Ruby, Python, Node.js, Go, Rust, and
Bash. Each language supports templates (e.g., `ruby --template rails`,
`bash --template devops`) and optional git hook generation.

## Build & Development Commands

```bash
cargo build                    # Debug build
cargo build --release          # Release build
cargo test                     # Run all tests
cargo test -p zackstrap --test e2e_tests  # E2E tests only
cargo test <test_name>         # Run a single test by name
cargo fmt --all                # Format code
cargo fmt --all -- --check     # Check formatting
cargo clippy --all-targets --all-features -- -D warnings  # Lint
just ci-local                  # Full local CI (lint + test)
just pre-commit                # Quick pre-commit checks (fmt + clippy + check)
```

## Architecture

**CLI layer** (`src/main.rs`): Uses `clap` derive macros. The `Cli` struct holds global flags (`--force`, `--fail-on-exists`, `--dry-run`, `--hooks`, `--target`). Each language is a `Commands` enum variant with an optional `--template` arg.

**Command dispatch** (`src/commands.rs`): `CommandHandler` holds the resolved flags and delegates to `ConfigGenerator` for file generation and `GitHooksGenerator` for hook generation. All handler methods are async.

**Generators** (`src/generators/`): One file per language (`basic.rs`,
`ruby.rs`, `python.rs`, `node.rs`, `go.rs`, `rust.rs`, `bash.rs`), plus
`hooks.rs` for git hooks and `common.rs` for shared utilities. `mod.rs`
contains `ConfigGenerator` and the `ProjectType` enum used for auto-detection.
Auto-detection checks for marker files (e.g., `Gemfile` -> Ruby, `go.mod` ->
Go, `.shellcheckrc` / `main.sh` / `.bats` -> Bash).

**Config structs** (`src/config.rs`): `EditorConfig`, `PrettierConfig`, `PackageJson` with `Display` implementations that produce the actual file content.

**Errors** (`src/error.rs`): `ZackstrapError` via `thiserror` with variants for directory issues, file conflicts, serialization, and git state.

## Code Style

- Rust 2021 edition, `rustfmt.toml`: max_width=100, tab_spaces=4, Unix newlines
- Clippy configured with relaxed thresholds in `.clippy.toml` (e.g., cognitive-complexity=35, too-many-arguments=8)
- All clippy warnings treated as errors (`-D warnings`)

## Testing

Tests live in `tests/` as separate integration test files:
- `unit_tests.rs` - Config struct creation and formatting
- `generators_tests.rs` - File generation logic
- `integration_tests.rs` - Full CLI flow with `assert_cmd`
- `cli_tests.rs` - CLI argument parsing
- `e2e_tests.rs` - End-to-end scenarios
- `fail_on_exists_tests.rs` - `--fail-on-exists` flag behavior

Tests use `tempfile`/`assert_fs` for temporary directories and `assert_cmd` for running the binary.

## CI

GitHub Actions in `.github/workflows/`:
- `ci.yml`: Lint (rustfmt + clippy) then test across Ubuntu/macOS/Windows on stable + 1.89, with tarpaulin coverage on Ubuntu
- `release.yml`: Tag-triggered release with cross-platform binary builds

## Release

Version bumps via `just release-patch|minor|major` which uses `cargo-set-version`, commits, tags, and pushes.
