# Workspace instructions

- Read the target project's README, AGENTS.md and docs before changing it.
- Keep prototypes under prototypes/<slug>; each has an independent workspace and lockfile.
- Update projects.json and the root project table when adding a project.
- Keep shared scripts small and dependency-light. Do not introduce a root runtime that couples every prototype.
- Distinguish implemented behavior, design targets and unresolved gates in docs and responses.
- Run python3 scripts/check_workspace.py for repository changes and the affected project's checks.
- Preserve the repository license. Do not add secrets, local databases, build products or downloaded model files.
- Keep registry publishing separate from validation. Prototype package names are provisional until checked.
