# funpou (分報)

[![Crates.io](https://img.shields.io/crates/v/funpou.svg)](https://crates.io/crates/funpou)

Quick one-line memos with automatic timestamps.

## Features

- One-command memo capture with auto-timestamp
- Scrollable list view (pipe to `fzf` or `less`)
- JSONL storage for fast read/write
- Optional [Obsidian](https://obsidian.md/) vault integration
- Configurable timestamp format

## Quick Start

```sh
# Save a memo
fnp add fix the login bug

# Or pipe text from stdin
echo "fix the login bug" | fnp add
pbpaste | fnp add

# List all memos
fnp list

# List in oldest-first order
fnp list -r

# Search with fzf
fnp list | fzf

# Show last 10 memos
fnp list -n 10

# Clear all memos (with confirmation)
fnp clear

# Clear all memos without confirmation
fnp clear --yes
```

`fnp list` shows newest memos first by default. Use `-r` / `--reverse` for oldest first.

## Install

**Homebrew (macOS):**

```bash
brew install to4iki/tap/funpou
```

**mise:**

```bash
mise use -g github:to4iki/funpou
```

**Cargo**

```bash
cargo install funpou
```

## Configuration

Config file: `~/.config/funpou/config.toml` (optional — works with zero config)

```toml
timestamp_format = "%Y-%m-%d %H:%M"

[obsidian]
vault_path = "/path/to/vault"
template_path = "daily/{{date:YYYY}}/{{date:YYYY-MM}}.md"
target_heading = "## Memos"
entry_format = "- {{timestamp}}: {{body}}"
```

## Documentation

- [Configuration](docs/configuration.md) — All config options and Obsidian setup
- [Usage](docs/usage.md) — Commands, flags, and shell integration

## License

MIT
