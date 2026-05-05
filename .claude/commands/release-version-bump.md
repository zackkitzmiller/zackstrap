---
name: release-version-bump
description: Workflow command scaffold for release-version-bump in zackstrap.
allowed_tools: ["Bash", "Read", "Write", "Grep", "Glob"]
---

# /release-version-bump

Use this workflow when working on **release-version-bump** in `zackstrap`.

## Goal

Prepare and execute a new release by updating version numbers and changelog, and merging main into feature branches if needed.

## Common Files

- `Cargo.toml`
- `Cargo.lock`
- `CHANGELOG.md`

## Suggested Sequence

1. Understand the current state and failure mode before editing.
2. Make the smallest coherent change that satisfies the workflow goal.
3. Run the most relevant verification for touched files.
4. Summarize what changed and what still needs review.

## Typical Commit Signals

- Update version numbers in Cargo.toml and Cargo.lock
- Update CHANGELOG.md with recent changes
- Merge main into feature branch if necessary

## Notes

- Treat this as a scaffold, not a hard-coded script.
- Update the command if the workflow evolves materially.