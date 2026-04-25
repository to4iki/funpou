# Configuration

funpou works with zero configuration. All settings are optional.

## Config File

**Path:** `~/.config/funpou/config.toml`

Create this file manually when you want to customize behavior. To check the resolved config:

```sh
fnp config
fnp config --path
```

## Template Syntax

Date/time formatting uses [chrono strftime](https://docs.rs/chrono/latest/chrono/format/strftime/index.html) directly. Common specifiers: `%Y` (year), `%m` (month), `%d` (day), `%H` (hour), `%M` (minute), `%S` (second).

`entry_format` additionally supports a single `{body}` placeholder for the memo text. strftime is applied first, so any `%` in the memo body is preserved untouched.

## Options

### `timestamp_format`

strftime format string for the timestamp shown in `fnp add` / `fnp list` output.

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
| `template_path` | string | `"daily/%Y-%m-%d.md"` | File path relative to vault — strftime specifiers or a literal path |
| `target_heading` | string | `"## Memos"` | Markdown heading to insert under |
| `entry_format` | string | `"- %Y-%m-%d %H:%M: {body}"` | Format for each memo line; `{body}` is replaced with the memo text |

### Full example

```toml
timestamp_format = "%Y-%m-%d %H:%M"

[obsidian]
vault_path = "~/ObsidianVault"
template_path = "daily/%Y-%m-%d.md"
target_heading = "## Memos"
entry_format = "- %Y-%m-%d %H:%M: {body}"
```

### Template Path

`template_path` accepts strftime specifiers or a static path:

```toml
template_path = "daily/%Y-%m-%d.md"  # → daily/2026-03-20.md
template_path = "notes/times.md"     # static — always the same file
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

`entry_format` is rendered in two passes: strftime is applied first against the memo timestamp, then `{body}` is replaced with the memo text. This order means a `%` character inside the body is never reinterpreted as a strftime specifier. `{body}` is meaningful only here — elsewhere it stays as a literal `{body}` string.

### Error Handling

The JSONL file (source of truth) is always written first. If the Obsidian write fails (e.g., vault not mounted), a warning is printed to stderr but the memo is never lost.
