# Configuration

funpou works with zero configuration. All settings are optional.

## Config File

**Path:** `~/.config/funpou/config.toml`

Create this file manually when you want to customize behavior. To check the resolved config:

```sh
fnp config
fnp config --path
```

## Options

### `timestamp_format`

Display format for timestamps using [strftime](https://docs.rs/chrono/latest/chrono/format/strftime/index.html) syntax.

- **Type:** String
- **Default:** `"%Y-%m-%d %H:%M"`

```toml
timestamp_format = "%Y-%m-%d %H:%M:%S"
```

## Obsidian Integration

When `vault_path` is set, each memo is also appended to a file in your Obsidian vault under a specific heading.

### `[obsidian]` section

| Key | Type | Default | Description |
|-----|------|---------|-------------|
| `vault_path` | string | `""` | Path to your Obsidian vault root — setting this enables the integration (`~/` is expanded to the home directory) |
| `template_path` | string | `"daily/{{date:YYYY}}/{{date:YYYY-MM}}.md"` | File path relative to vault — supports `{{date:...}}` placeholders or a literal path |
| `target_heading` | string | `"## Memos"` | Markdown heading to insert under |
| `entry_format` | string | `"- {{timestamp}}: {{body}}"` | Format for each memo line |

### Full example

```toml
timestamp_format = "%Y-%m-%d %H:%M"

[obsidian]
vault_path = "~/ObsidianVault"
template_path = "daily/{{date:YYYY}}/{{date:YYYY-MM}}.md"
target_heading = "## Memos"
entry_format = "- {{timestamp}}: {{body}}"
```

### Template Path

`template_path` accepts `{{date:...}}` placeholders, or a static path with none:

```toml
template_path = "daily/{{date:YYYY-MM-DD}}.md"  # → daily/2026-03-20.md
template_path = "notes/times.md"                # static — always the same file
```

Supported tokens: `YYYY` `MM` `DD` `HH` `mm` `ss`.

### Heading-Based Insertion

Memos are inserted just before the next heading of equal or higher level:

```markdown
## Memos
- 2026-03-20 14:00: existing memo
- 2026-03-20 14:05: new memo       ← inserted here

## Other Section
```

If the heading does not exist, it is appended to the end of the file. If the file does not exist, it is created along with any necessary directories.

### Entry Format

The `entry_format` string supports these placeholders:

| Placeholder | Replaced with |
|-------------|---------------|
| `{{timestamp}}` | Formatted timestamp (using `timestamp_format`) |
| `{{body}}` | Memo text |

### Error Handling

The JSONL file (source of truth) is always written first. If the Obsidian write fails (e.g., vault not mounted), a warning is printed to stderr but the memo is never lost.
