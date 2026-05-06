# Project Guide

## Overview

funpou is a Rust CLI for quick one-line memos with automatic timestamps and optional Obsidian integration.

## Tech Stack

- Rust (edition 2024), clap 4 (derive)
- Storage: JSONL (`~/.local/share/funpou/memos.jsonl`)
- Config: TOML (`~/.config/funpou/config.toml`)
- Key crates: chrono, serde, serde_json, toml, dirs, anyhow

## Development

```sh
cargo test    # Run all tests
cargo clippy  # Lint
cargo fmt     # Format
cargo bench --bench storage  # Run storage benchmarks
```

TDD workflow: write tests first → make them pass → refactor.

See [.claude/rules/testing.md](.claude/rules/testing.md) for test policy.

## Design Decisions

- Obsidian integration is opt-in — enabled automatically when `vault_path` is set in `config.toml`
- Timestamp ID (`YYYYMMDDhhmmss`) — no external ID crate needed
- Config format strings use chrono strftime directly (`%Y-%m-%d %H:%M`); `entry_format` additionally accepts `{body}` as a placeholder for the memo text
- No TUI crate — pipe to `less` or `fzf` for scrolling
