# Disk explainer instructions

Read docs/spec.md and docs/implementation-plan.md first. Build the smallest demonstrable milestone, starting with the NEDB storage contract.

- NEDB is authoritative local persistence; no SQLite fallback or hidden parallel database.
- Preserve the distinction between observations, ownership inference and causation.
- Unknown size/reclaimability must remain unknown. Never represent it as zero.
- Human and JSON interfaces share domain results; agents receive no implicit deletion authorization.
- No cleanup implementation before the storage and filesystem safety gates in the spec.
- Do not execute project-controlled scripts to identify artifact ownership.
- Pin/test NEDB API behavior; batch writes are not an assumed atomic transaction.
- Test affected behavior with cargo test; format and run clippy. Commit the resolved Cargo.lock once generated.
