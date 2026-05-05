---
name: refactor-generator-file-writing
description: Workflow command scaffold for refactor-generator-file-writing in zackstrap.
allowed_tools: ["Bash", "Read", "Write", "Grep", "Glob"]
---

# /refactor-generator-file-writing

Use this workflow when working on **refactor-generator-file-writing** in `zackstrap`.

## Goal

Refactor how generators handle file writing, especially dry_run and force logic, by centralizing logic and removing redundant code.

## Common Files

- `src/generators/common.rs`
- `src/generators/mod.rs`
- `src/commands.rs`
- `src/generators/bash.rs`
- `src/generators/basic.rs`
- `src/generators/go.rs`

## Suggested Sequence

1. Understand the current state and failure mode before editing.
2. Make the smallest coherent change that satisfies the workflow goal.
3. Run the most relevant verification for touched files.
4. Summarize what changed and what still needs review.

## Typical Commit Signals

- Update core generator logic to add or refactor dry_run/force handling (e.g., add struct fields, new emit_file method)
- Remove old dry_run_* methods from all generators
- Update all generator modules to use the new unified method
- Update command handling to use new generator options
- Update or add relevant tests to cover new logic

## Notes

- Treat this as a scaffold, not a hard-coded script.
- Update the command if the workflow evolves materially.