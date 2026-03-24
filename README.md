# funpou

A minimal CLI tool for quick one-line memos with automatic timestamps.

## Features

- One-command memo capture with auto-timestamp
- Scrollable list view (pipe to `fzf` or `less`)
- JSONL storage for fast read/write
- Optional [Obsidian](https://obsidian.md/) vault integration
- Configurable timestamp format

## Quick Start

```sh
# Save a memo
funpou add fix the login bug

# List all memos
funpou list

# List in oldest-first order
funpou list -r

# Search with fzf
funpou list | fzf

# Show last 10 memos
funpou list -n 10
```

`funpou list` shows newest memos first by default. Use `-r` / `--reverse` for oldest first.

## Installation

```sh
cargo install --path .
```

## Configuration

Config file: `~/.config/funpou/config.toml` (optional — works with zero config)

```toml
timestamp_format = "%Y-%m-%d %H:%M"

[obsidian]
enabled = true
vault_path = "/path/to/vault"
template_path = "daily/{{date:YYYY}}/{{date:YYYY-MM}}.md"
target_heading = "## Memos"
entry_format = "- {{timestamp}}: {{body}}"
```

See [docs/configuration.md](docs/configuration.md) for details.

## Documentation

- [Configuration](docs/configuration.md) — All config options and Obsidian setup
- [Usage](docs/usage.md) — Commands, flags, and shell integration

## License

MIT
