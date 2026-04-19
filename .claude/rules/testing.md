---
paths:
  - "src/**/*.{rust}"
---

# Testing Policy

## Core principle

Test value, not coverage. A test suite with fewer, well-chosen tests is better than one
with high coverage achieved through low-value assertions.

## What to test

- **Observable behavior through realistic entry points** — config loading, command execution,
  file I/O. These survive refactoring because they verify what the system does, not how.
- **Non-obvious invariants** — edge cases that are easy to get wrong and not obvious from
  reading the code (e.g., template path resolution with malformed placeholders).
- **Failure paths that matter** — behaviors that would silently corrupt data or produce wrong
  output if broken.

## What not to test

- **Trivial methods** — a 1-liner that delegates to a field or calls one stdlib function
  does not need its own test. If the behavior is already exercised through a higher-level
  test, the unit test adds no information.
- **Standard library / language behavior** — do not write tests to confirm that
  `str::trim()`, `Vec::len()`, or other stdlib primitives work correctly.
- **Internal implementation details** — avoid asserting on intermediate state or private
  logic that could change without affecting observable behavior. These cause false positives
  during refactoring (test fails even though the feature still works).
- **Backward compatibility for removed requirements** — if backward compat is not an
  explicit product requirement, do not add tests for it.

## Redundancy rule

Before adding a new test, check whether an existing test at a higher level already
exercises the same code path. Prefer the higher-level test. Delete the lower-level one
if both exist.

## On test naming

Name tests after the behavior being verified, not the method under test.
`load_full_obsidian_config_activates_when_vault_path_set` is better than
`is_enabled_returns_true`.
