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

When enabled, each memo is also appended to a file in your Obsidian vault under a specific heading.

### `[obsidian]` section

| Key | Type | Default | Description |
|-----|------|---------|-------------|
| `enabled` | bool | `false` | Enable Obsidian integration |
| `vault_path` | string | `""` | Path to your Obsidian vault root (`~/` is expanded to the home directory) |
| `template_path` | string | `"daily/{{date:YYYY}}/{{date:YYYY-MM}}.md"` | File path template (relative to vault) |
| `target_heading` | string | `"## Memos"` | Markdown heading to insert under |
| `entry_format` | string | `"- {{timestamp}}: {{body}}"` | Format for each memo line |

### Full example

```toml
timestamp_format = "%Y-%m-%d %H:%M"

[obsidian]
enabled = true
vault_path = "~/ObsidianVault"
template_path = "daily/{{date:YYYY}}/{{date:YYYY-MM}}.md"
target_heading = "## Memos"
entry_format = "- {{timestamp}}: {{body}}"
```

### Template Path

The `template_path` supports Obsidian-compatible date placeholders:

| Token | Resolves to | Example |
|-------|-------------|---------|
| `YYYY` | 4-digit year | `2026` |
| `MM` | 2-digit month | `03` |
| `DD` | 2-digit day | `20` |
| `HH` | 2-digit hour (24h) | `14` |
| `mm` | 2-digit minute | `05` |
| `ss` | 2-digit second | `32` |

Wrap tokens in `{{date:...}}`:

```
daily/{{date:YYYY}}/{{date:YYYY-MM}}.md   → daily/2026/2026-03.md
notes/{{date:YYYY-MM-DD}}.md              → notes/2026-03-20.md
```

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
