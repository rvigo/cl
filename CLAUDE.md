# CLAUDE.md

This file provides guidance to Claude Code (claude.ai/code) when working with code in this repository.

## Guidelines

- Never bypass or suppress errors (e.g. no `unwrap`, `#[allow(...)]`, `--no-verify`, ignoring `Result`s) without asking the user first.
- Ask the user before taking any action that goes beyond what was explicitly requested.

## Commands

```bash
# Build
cargo build --release

# Test
cargo test --workspace

# Run a single test
cargo test --workspace <test_name>

# Lint (CI enforces -D warnings)
cargo clippy -- -D warnings

# Format check
cargo fmt --all -- --check

# Format fix
cargo fmt --all

# Benchmarks
cargo bench
```

## Architecture

`cl` is a command organization and execution tool. It stores commands (with aliases, namespaces, and named parameters) in a TOML file and provides two interfaces: a CLI and a TUI.

### Workspace layout

- **cl-core** — domain types and logic (no I/O concerns)
- **cl-cli** — `clap`-based CLI; defaults to launching the TUI when called with no subcommand
- **cl-new-gui** — `ratatui`-based TUI

### Core domain (cl-core)

`Command<'cmd>` is the central type: `alias`, `namespace`, `command` string, optional `description` and `tags`. Named parameters in commands use `#{name}` syntax and are substituted at execution time.

`Commands<'cmd>` wraps a `HashMap<Namespace, Vec<Command>>` and exposes `add`, `edit`, `remove`, `find(alias, namespace)`.

`Config` trait abstracts configuration; `DefaultConfig` reads/writes TOML at `~/.config/cl/config.toml`. Command data is stored at `~/.config/cl/commands.toml`.

Extension traits (`CommandVecExt`, `CommandMapExt`) add sorting, filtering, and mapping helpers to standard collections.

### CLI subcommands (cl-cli)

| Subcommand | Alias | Purpose |
|---|---|---|
| `exec` | `x` | Find and execute a stored command, substituting `#{param}` via `--param=value` flags |
| `share` | `s` | Import/export commands |
| `config` | — | Manage settings |
| `add` | — | Add a command without the TUI |
| `misc` | — | Hidden utilities |

### TUI (cl-new-gui)

Uses an actor model with Tokio channels:

- **`StateActor`** owns app state; receives `StateEvent` messages from the UI
- **`UiActor`** renders with ratatui and emits `StateEvent` on key input

UI is stack-based with a `Layer` trait:
- `MainScreenLayer` — namespace tabs + command list
- `EditScreenLayer` — form for editing command fields
- `QuickSearchLayer` — fuzzy search via `nucleo-matcher`
- `PopupLayer` — error/confirmation dialogs

`SignalHandler` intercepts SIGTERM/Ctrl-C and coordinates graceful shutdown via a `tokio::sync::broadcast` channel.

### Benchmarks

`benches/core.rs` uses Criterion with sample data at `benches/data/sample.toml`. Benchmarks cover init, find, update, add, and remove operations on `Commands`.
