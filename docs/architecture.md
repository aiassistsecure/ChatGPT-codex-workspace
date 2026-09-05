# Monorepo architecture

`prototypes/<slug>/` is the unit of development and release. Rust projects own their Cargo workspace, Python projects their pyproject.toml, and web projects their package manager metadata. There is intentionally no root Cargo or npm workspace.

`projects.json` is a catalog, not a build system. `scripts/` contains repository checks. `.github/workflows/` contains CI with project-scoped jobs. `docs/` contains repository-wide decisions.

A mature prototype can remain here or graduate into its own repository without importing unrelated dependencies. Use project-scoped tags such as disk-explainer/v0.1.0. Keep package versions independent between prototypes.
