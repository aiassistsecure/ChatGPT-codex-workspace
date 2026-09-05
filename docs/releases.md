# Release conventions

Each prototype owns its version and release readiness. Use project-prefixed tags, changelogs, and explicit registry targets. Ordinary pull requests run validation only.

Disk explainer will publish a Rust core/CLI, Python platform wheels and npm platform packages around one Rust engine. Its crates are currently publish=false; the Python and npm directories document the contract without fake installable launchers.

Before enabling publication: finalize available names, confirm license metadata, commit a resolved lockfile, define platforms, build binaries once, verify checksums and clean installs, configure registry authentication, and review a project-specific release workflow. Track partial registry success because cross-registry publishing is not atomic. Never rebuild different bytes under an already published version.
