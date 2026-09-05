# Implementation milestones

| Gate | Deliverable | Proof |
| --- | --- | --- |
| Scaffold (this change) | Workspace, capability CLI, NEDB smoke tests, spec, CI | Catalog check; compile/tests in CI |
| M0 | Verified NEDB storage adapter and committed Cargo.lock | Write/kill/reopen boundaries, readiness, error propagation, bounded queries and retention behavior |
| M1 | Read-only scan with Rust/Node ownership evidence | Disposable fixtures and real directory output with correct unknowns |
| M2 | NEDB history and explanations | Growth/rename comparison across process restarts |
| M3 | Reviewed cleanup plans and receipts | Stale-plan, symlink-race and interrupted-cleanup tests |
| M4 | Cargo/PyPI/npm distribution | Clean installs with identical results on supported platforms |
| M5 | Real-user trial | Useful first run and repeated use compared with existing tools |

Next action: run and inspect the scaffold's NEDB smoke tests, then build M0. Do not treat these smoke tests as crash-durability coverage. Read the full spec's persistence protocol and retention caveats before implementing deletion.
