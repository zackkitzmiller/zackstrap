```markdown
# zackstrap Development Patterns

> Auto-generated skill from repository analysis

## Overview
This skill provides guidance on contributing to the `zackstrap` Rust codebase. It covers coding conventions, file organization, and common development workflows, including generator refactoring and release management. By following these patterns, contributors ensure consistency, maintainability, and reliability across the project.

## Coding Conventions

- **File Naming:**  
  Use camelCase for file names.  
  _Example:_  
  ```
  src/generators/basic.rs
  src/commands.rs
  ```

- **Import Style:**  
  Use relative imports within the crate.  
  _Example:_  
  ```rust
  use super::common::{emit_file, GeneratorOptions};
  use crate::generators::bash::BashGenerator;
  ```

- **Export Style:**  
  Use named exports for modules and functions.  
  _Example:_  
  ```rust
  pub mod basic;
  pub fn emit_file(...) { ... }
  ```

- **Commit Messages:**  
  - Mixed types, often prefixed with `refactor` or `fix`
  - Aim for concise summaries (~49 characters)
  _Example:_  
  ```
  refactor: centralize file writing logic in generators
  fix: handle dry_run option in node generator
  ```

## Workflows

### Refactor Generator File Writing
**Trigger:** When improving or unifying how code generators handle file output, especially for `dry_run` and `force` behaviors.  
**Command:** `/refactor-generator-file-writing`

1. **Update core generator logic**  
   - Add or refactor `dry_run`/`force` handling (e.g., add struct fields, new `emit_file` method)  
   _Example:_  
   ```rust
   pub struct GeneratorOptions {
       pub dry_run: bool,
       pub force: bool,
   }

   pub fn emit_file(path: &str, content: &str, options: &GeneratorOptions) -> Result<(), Error> {
       if options.dry_run {
           println!("Dry run: would write to {}", path);
           return Ok(());
       }
       // ... actual file writing logic ...
   }
   ```

2. **Remove old `dry_run_*` methods from all generators**  
   - Delete redundant or legacy methods handling dry-run logic.

3. **Update all generator modules to use the new unified method**  
   - Refactor each generator (e.g., `bash.rs`, `go.rs`, etc.) to use `emit_file` and pass `GeneratorOptions`.

4. **Update command handling to use new generator options**  
   - Ensure commands construct and pass the correct options.

5. **Update or add relevant tests to cover new logic**  
   - Modify or add tests in `tests/fail_on_exists_tests.rs`, `tests/integration_tests.rs` to verify new behaviors.

**Files Involved:**  
- `src/generators/common.rs`
- `src/generators/mod.rs`
- `src/commands.rs`
- `src/generators/bash.rs`
- `src/generators/basic.rs`
- `src/generators/go.rs`
- `src/generators/node.rs`
- `src/generators/python.rs`
- `src/generators/ruby.rs`
- `src/generators/rust.rs`
- `tests/fail_on_exists_tests.rs`
- `tests/integration_tests.rs`

---

### Release Version Bump
**Trigger:** When preparing a new release or synchronizing release metadata.  
**Command:** `/release-bump`

1. **Update version numbers**  
   - Edit `Cargo.toml` and `Cargo.lock` to reflect the new version.
   _Example:_  
   ```toml
   [package]
   name = "zackstrap"
   version = "1.2.3"
   ```

2. **Update `CHANGELOG.md`**  
   - Add recent changes under the new version heading.

3. **Merge main into feature branch if necessary**  
   - Ensure release metadata is up to date across branches.

**Files Involved:**  
- `Cargo.toml`
- `Cargo.lock`
- `CHANGELOG.md`

---

## Testing Patterns

- **Test File Naming:**  
  Test files follow the `*.test.*` pattern (e.g., `integration_tests.rs`).
- **Framework:**  
  The specific test framework is not specified, but Rust's built-in test framework is likely used.
- **Test Example:**  
  ```rust
  #[test]
  fn test_emit_file_dry_run() {
      let opts = GeneratorOptions { dry_run: true, force: false };
      let result = emit_file("foo.txt", "bar", &opts);
      assert!(result.is_ok());
      // Check that file was not written
  }
  ```

## Commands

| Command                           | Purpose                                                        |
|------------------------------------|----------------------------------------------------------------|
| /refactor-generator-file-writing   | Refactor and unify generator file writing logic                |
| /release-bump                     | Prepare and execute a new release (version, changelog, merge)  |
```
