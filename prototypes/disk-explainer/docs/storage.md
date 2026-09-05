# NEDB integration baseline

Dependency: nedb-engine 2.8.5, pinned for the initial API spike. See https://docs.rs/nedb-engine/2.8.5/nedb_engine/db/struct.Db.html.

Scans, observations, findings, plans and cleanup events will live in NEDB. The scaffold only exercises put/get and flush/reopen against the actual engine; it does not expose a production storage adapter yet.

M0 must resolve startup readiness, single-process ownership, persistence error visibility, incomplete-batch handling, bounded history reads and compaction effects. Store explicit completion records; do not assume put_batch is a transaction. Keep observed ownership separate from external causation. No cleanup before durable intent can be verified.
