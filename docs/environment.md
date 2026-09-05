# Codex environment setup

Select aiassistsecure/ChatGPT-codex-workspace in the Codex environment UI, choose the working branch, and provision Rust stable (rustfmt and clippy), Python 3.11+, Git and, for future distribution work, Node.js LTS.

Dependency setup for the first project:

```sh
cargo fetch --manifest-path prototypes/disk-explainer/Cargo.toml
python3 scripts/check_workspace.py
```

Validation:

```sh
cd prototypes/disk-explainer
cargo fmt --all --check
cargo clippy --workspace --all-targets -- -D warnings
cargo test --workspace
```

Dependency setup needs access to crates.io, its download endpoints, and the Rust toolchain hosts. Run package resolution during the environment setup phase if task execution has restricted network access. Do not put access tokens in scripts or committed files.

This file describes setup; it does not create or change a hosted Codex environment. GitHub connector access and a shell's Git/toolchain/network access are separate capabilities.
