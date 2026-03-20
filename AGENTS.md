# Project Guide

## Overview

funpou is a minimal Rust CLI tool for quick one-line memos with automatic timestamps and optional Obsidian integration.

## Tech Stack

- **Language:** Rust (edition 2024)
- **CLI:** clap 4 (derive)
- **Storage:** JSONL (`~/.local/share/funpou/memos.jsonl`)
- **Config:** TOML (`~/.config/funpou/config.toml`)
- **Key crates:** chrono, serde, serde_json, toml, dirs, anyhow

## Module Structure

```
src/
  main.rs          Entry point (CLI parse → command dispatch)
  cli.rs           clap derive definitions
  config.rs        Config struct, TOML deserialization, defaults
  memo.rs          Memo struct, timestamp ID generation, serde
  storage.rs       JSONL read/write (append / read_all)
  obsidian.rs      Template path resolution, heading-based insertion
  commands.rs      Module re-exports (Rust 2024 style)
  commands/
    add.rs         add command
    list.rs        list command
    config.rs      config command
```

## Development

```sh
cargo test       # Run all tests
cargo clippy     # Lint
cargo fmt        # Format
```

TDD workflow: write tests first → make them pass → refactor.

## Design Decisions

- **No TUI crate** — scrollable list is achieved via piping to `less` or `fzf`
- **JSONL** — append-only, one JSON object per line for fast I/O
- **Obsidian integration is opt-in** — disabled by default, enabled via `config.toml`
- **Timestamp ID** (`YYYYMMDDhhmmss`) — simple, no external ID crate needed
- **Obsidian-compatible date syntax** in template paths (`YYYY`, `MM`, `DD`) converted internally to chrono strftime
