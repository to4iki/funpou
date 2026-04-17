# Usage

## Commands

### `fnp add <TEXT...>`

Save a one-line memo with an automatic timestamp.

```sh
fnp add fix the login bug
fnp add "refactor auth module"
```

Multiple words are joined with spaces. Quotes are optional.

Output (printed to stderr):

```
2026-03-20 14:05: fix the login bug
```

### `fnp list`

Print all saved memos to stdout.

```sh
fnp list
```

Default order is newest first.

#### Flags

| Flag | Description |
|------|-------------|
| `-n, --limit <N>` | Show only the last N memos |
| `-r, --reverse` | Reverse order (oldest first) |
| `--json` | Output raw JSONL for scripting |

Examples:

```sh
# Last 20 memos
fnp list -n 20

# Oldest-first order
fnp list -r

# Raw JSON output
fnp list --json
```

### `fnp clear`

Delete all saved memos after confirmation.

```sh
fnp clear       # Prompts "Clear N memo(s)? [y/N]:"
fnp clear --yes # Skip confirmation (useful for scripts)
fnp clear -y    # Short form of --yes
```

Only an explicit `y` or `yes` (case-insensitive) proceeds with deletion. Any other input — including pressing Enter — cancels.

If there are no memos, the command exits immediately with `No memos to clear.`

### `fnp config`

Show the resolved configuration.

```sh
fnp config          # Print full config as TOML
fnp config --path   # Print config file path only
```

## Shell Integration

### Aliases

```sh
alias f="fnp add"
alias fl="fnp list | fzf"
alias fll="fnp list -n 20"
```

### fzf

Pipe the list output to `fzf` for interactive search:

```sh
fnp list | fzf
```

### JSON processing with jq

```sh
# Extract only memo bodies
fnp list --json | jq -r '.body'

# Filter by date
fnp list --json | jq 'select(.created_at | startswith("2026-03"))'
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
