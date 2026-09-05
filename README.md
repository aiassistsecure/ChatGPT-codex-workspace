# ChatGPT Codex Workspace

A monorepo for prototypes built by aiassistsecure with Codex. Each project owns its runtime, dependencies, tests and release lifecycle.

## Projects

| Project | Stage | Purpose |
| --- | --- | --- |
| [Disk explainer](prototypes/disk-explainer/) | Scaffold | Explain disk usage, ownership and growth with local NEDB history |

The machine-readable catalog is [projects.json](projects.json).

## Start here

```sh
python3 scripts/check_workspace.py
cd prototypes/disk-explainer
cargo test --workspace
cargo run -p disk-explainer-cli -- about --json
```

Rust stable with rustfmt/clippy and Python 3.11+ are required for these commands. npm packaging will require Node.js when the distribution milestone starts.

- [Contributing](CONTRIBUTING.md)
- [Architecture](docs/architecture.md)
- [Environment setup](docs/environment.md)
- [Release conventions](docs/releases.md)

No packages are published by this scaffold. The repository's existing GPLv3 license is preserved; package metadata uses GPL-3.0-only pending any explicit owner change.
