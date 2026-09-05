# Disk explainer

A local disk-usage explainer for people and agents, with NEDB-backed observation history. Working name; registry names remain provisional.

## What exists

- Independent Rust workspace with core and CLI crates.
- A capability command with human and JSON output.
- Shared serialized capability model and NEDB in-memory/reopen smoke tests.
- Full product specification, implementation gates and distribution contracts.

Scanning, ownership detectors, history comparison, cleanup and registry launchers are not implemented. `about` says so explicitly; other commands exit with an error.

```sh
cargo run -p disk-explainer-cli -- about
cargo run -p disk-explainer-cli -- about --json
cargo test --workspace
```

NEDB is a real pinned core dependency, not a substitute database. The persistence tests are a starting point, not a durability certification. See [implementation plan](docs/implementation-plan.md) and [full specification](docs/spec.md).

## Layout

- crates/core: shared data and future scanner/detector/storage APIs.
- crates/cli: terminal entry point.
- packages/python and packages/npm: future distribution contract.
- docs: specification, milestones and storage design.

## Checks

```sh
cargo fmt --all --check
cargo clippy --workspace --all-targets -- -D warnings
cargo test --workspace
```

Cargo.lock is committed after real dependency resolution. CI uses --locked. Publication is disabled in both crate manifests.
