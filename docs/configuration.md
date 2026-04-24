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

All format strings (`timestamp_format`, `template_path`, `entry_format`) share a single template syntax:

- `{...}` — expanded when rendering
- Anything outside braces — copied verbatim
- Unclosed `{` — kept literal

Inside `{...}`:

| Token | Meaning |
|-------|---------|
| `YYYY` | 4-digit year |
| `MM`   | 2-digit month |
| `DD`   | 2-digit day |
| `HH`   | 2-digit hour (24h) |
| `mm`   | 2-digit minute |
| `ss`   | 2-digit second |
| `body` | Memo text (valid only in `entry_format`) |

Tokens may appear together with literal characters, e.g. `{YYYY-MM-DD-HH:mm}` → `2026-03-20-14:05`.

## Options

### `timestamp_format`

Template for the timestamp shown in `fnp add` / `fnp list` output.

- **Type:** String
- **Default:** `"{YYYY-MM-DD-HH:mm}"`

```toml
timestamp_format = "{YYYY-MM-DD HH:mm:ss}"
```

## Obsidian Integration

When `vault_path` is set, each memo is also appended to a file in your Obsidian vault under a specific heading.

### `[obsidian]` section

| Key | Type | Default | Description |
|-----|------|---------|-------------|
| `vault_path` | string | `""` | Path to your Obsidian vault root — setting this enables the integration (`~/` is expanded to the home directory) |
| `template_path` | string | `"daily/{YYYY-MM-DD}.md"` | File path relative to vault — supports date tokens or a literal path |
| `target_heading` | string | `"## Memos"` | Markdown heading to insert under |
| `entry_format` | string | `"- {YYYY-MM-DD-HH:mm}: {body}"` | Template for each memo line; `{body}` is the memo text |

### Full example

```toml
timestamp_format = "{YYYY-MM-DD-HH:mm}"

[obsidian]
vault_path = "~/ObsidianVault"
template_path = "daily/{YYYY-MM-DD}.md"
target_heading = "## Memos"
entry_format = "- {YYYY-MM-DD-HH:mm}: {body}"
```

### Template Path

`template_path` accepts date tokens or a static path:

```toml
template_path = "daily/{YYYY-MM-DD}.md"  # → daily/2026-03-20.md
template_path = "notes/times.md"         # static — always the same file
```

### Heading-Based Insertion

Memos are inserted just before the next heading of equal or higher level:

```markdown
## Memos
- 2026-03-20-14:00: existing memo
- 2026-03-20-14:05: new memo       ← inserted here

## Other Section
```

If the heading does not exist, it is appended to the end of the file. If the file does not exist, it is created along with any necessary directories.

### Entry Format

`entry_format` uses the same template syntax. `{body}` is replaced with the memo text; date tokens resolve against the memo timestamp. Unlike `timestamp_format`, `{body}` is only meaningful here — elsewhere it expands to an empty string.

### Error Handling

The JSONL file (source of truth) is always written first. If the Obsidian write fails (e.g., vault not mounted), a warning is printed to stderr but the memo is never lost.
