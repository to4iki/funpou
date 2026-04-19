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
```

TDD workflow: write tests first → make them pass → refactor.

## Design Decisions

- Obsidian integration is opt-in — disabled by default, enabled via `config.toml`
- Timestamp ID (`YYYYMMDDhhmmss`) — no external ID crate needed
- Obsidian template paths use `YYYY`/`MM`/`DD` syntax, converted internally to chrono strftime
- No TUI crate — pipe to `less` or `fzf` for scrolling
