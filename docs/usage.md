# Usage

## Commands

### `funpou add <TEXT...>`

Save a one-line memo with an automatic timestamp.

```sh
funpou add fix the login bug
funpou add "refactor auth module"
```

Multiple words are joined with spaces. Quotes are optional.

Output (printed to stderr):

```
2026-03-20 14:05: fix the login bug
```

### `funpou list`

Print all saved memos to stdout.

```sh
funpou list
```

#### Flags

| Flag | Description |
|------|-------------|
| `-n, --limit <N>` | Show only the last N memos |
| `-r, --reverse` | Reverse order (oldest last) |
| `--json` | Output raw JSONL for scripting |

Examples:

```sh
# Last 20 memos
funpou list -n 20

# Reverse chronological order
funpou list -r

# Raw JSON output
funpou list --json
```

### `funpou config`

Show the resolved configuration.

```sh
funpou config          # Print full config as TOML
funpou config --path   # Print config file path only
```

## Shell Integration

### Aliases

```sh
alias f="funpou add"
alias fl="funpou list | fzf"
alias fll="funpou list -n 20"
```

### fzf

Pipe the list output to `fzf` for interactive search:

```sh
funpou list | fzf
```

### JSON processing with jq

```sh
# Extract only memo bodies
funpou list --json | jq -r '.body'

# Filter by date
funpou list --json | jq 'select(.created_at | startswith("2026-03"))'
```

## Data Storage

Memos are stored as JSONL (one JSON object per line):

- **Path:** `~/.local/share/funpou/memos.jsonl`

Each line:

```json
{"id":"20260320140532","body":"fix the login bug","created_at":"2026-03-20T14:05:32+09:00"}
```

| Field | Description |
|-------|-------------|
| `id` | Timestamp ID (`YYYYMMDDhhmmss`) |
| `body` | Memo text |
| `created_at` | RFC 3339 timestamp with timezone |
