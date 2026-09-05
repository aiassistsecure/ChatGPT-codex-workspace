# Contributing

Work on a focused branch and open a pull request with the problem, changes, validation and limitations. Keep each prototype independently buildable.

New prototypes need a README explaining what works, a local AGENTS.md, an implementation plan, checks, and an entry in projects.json. Share code only after multiple projects need the same behavior.

Run the catalog validator and project checks before review. Record unavailable toolchains honestly. Check in generated application lockfiles once resolved by the real package manager; never hand-author a Cargo.lock. The disk-explainer lockfile is committed and CI enforces it with --locked.

Do not claim published packages, benchmark results or working features based solely on a scaffold. Release preparation and registry publishing are separate changes.
