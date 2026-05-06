# Changelog

## [v0.1.6](https://github.com/to4iki/funpou/compare/v0.1.5...v0.1.6) - 2026-05-06
- Add Criterion benchmarks by @to4iki in https://github.com/to4iki/funpou/pull/38
- Fix Criterion benchmark temp file exhaustion by @to4iki in https://github.com/to4iki/funpou/pull/40
- Fix review findings for packaging and storage safety by @to4iki in https://github.com/to4iki/funpou/pull/41

## [v0.1.5](https://github.com/to4iki/funpou/compare/v0.1.4...v0.1.5) - 2026-04-25
- fix(tagpr): use postVersionCommand to sync Cargo.lock on release bump by @to4iki in https://github.com/to4iki/funpou/pull/32
- docs: improve Obsidian template path examples and clarity by @to4iki in https://github.com/to4iki/funpou/pull/33
- refactor(template): adopt chrono strftime + {body} for config templates by @to4iki in https://github.com/to4iki/funpou/pull/34
- docs: clarify config format strings and {body} placeholder in AGENTS.md by @to4iki in https://github.com/to4iki/funpou/pull/36
- chore: add cargo command permissions to Claude settings by @to4iki in https://github.com/to4iki/funpou/pull/37

## [v0.1.4](https://github.com/to4iki/funpou/compare/v0.1.3...v0.1.4) - 2026-04-24
- feat: add --today flag to list command by @to4iki in https://github.com/to4iki/funpou/pull/28
- refactor: remove --limit flag from list command by @to4iki in https://github.com/to4iki/funpou/pull/30

## [v0.1.3](https://github.com/to4iki/funpou/compare/v0.1.2...v0.1.3) - 2026-04-19
- ci(tagpr): fetch all remote branches so fallback detection works by @to4iki in https://github.com/to4iki/funpou/pull/20
- docs: simplify AGENTS.md by @to4iki in https://github.com/to4iki/funpou/pull/22
- fix(tagpr): update Cargo.lock package version on release bump by @to4iki in https://github.com/to4iki/funpou/pull/23
- refactor(obsidian): remove explicit enabled flag — vault_path presence drives activation by @to4iki in https://github.com/to4iki/funpou/pull/25
- docs: add Homebrew installation instructions to README by @to4iki in https://github.com/to4iki/funpou/pull/24

## [v0.1.2](https://github.com/to4iki/funpou/compare/v0.1.1...v0.1.2) - 2026-04-18
- ci: sync Cargo.lock in tagpr PRs and auto-publish on release by @to4iki in https://github.com/to4iki/funpou/pull/12
- feat(clear): add clear command to delete all memos with confirmation by @to4iki in https://github.com/to4iki/funpou/pull/14
- docs: add clear command usage and Japanese name to README by @to4iki in https://github.com/to4iki/funpou/pull/15
- ci: rename release assets to maltmill-compatible naming by @to4iki in https://github.com/to4iki/funpou/pull/16

## [v0.1.1](https://github.com/to4iki/funpou/compare/v0.1.0...v0.1.1) - 2026-04-17
- ci(tagpr): use GitHub App token to trigger downstream release workflow by @to4iki in https://github.com/to4iki/funpou/pull/9
- docs: add mise as alternative installation method by @to4iki in https://github.com/to4iki/funpou/pull/11

## [v0.1.0](https://github.com/to4iki/funpou/commits/v0.1.0) - 2026-04-16
- ci: add tagpr and release binary upload workflow by @to4iki in https://github.com/to4iki/funpou/pull/1
- fix(ci): grant issues:write permission to tagpr by @to4iki in https://github.com/to4iki/funpou/pull/2
- chore: update managed files by @to4iki in https://github.com/to4iki/funpou/pull/4
- chore: prepare for crates.io publishing and rename binary to fnp by @to4iki in https://github.com/to4iki/funpou/pull/5
- docs: update installation instruction to cargo install funpou by @to4iki in https://github.com/to4iki/funpou/pull/6
- feat: support XDG base directory and tilde expansion for config and storage paths by @to4iki in https://github.com/to4iki/funpou/pull/7
- test: improve test quality across core modules by @to4iki in https://github.com/to4iki/funpou/pull/8
