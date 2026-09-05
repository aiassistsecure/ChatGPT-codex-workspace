//! Initial API/reopen checks; these do not prove crash or power-loss durability.
use nedb_engine::Db;
use serde_json::json;

#[test]
fn memory_store_accepts_scan_record() -> anyhow::Result<()> {
    let db = Db::in_memory();
    db.put(
        "scans",
        "scan-1",
        json!({"status": "incomplete", "schema_version": 1}),
        vec![],
        None,
        None,
    )?;
    assert!(db.get("scans", "scan-1").is_some());
    assert!(db.get("scans", "absent").is_none());
    Ok(())
}

#[test]
fn explicitly_flushed_record_survives_reopen() -> anyhow::Result<()> {
    let directory = tempfile::tempdir()?;
    {
        let db = Db::open(directory.path(), None)?;
        db.put(
            "scans",
            "scan-1",
            json!({"status": "incomplete", "schema_version": 1}),
            vec![],
            None,
            None,
        )?;
        db.flush_all();
    }
    let reopened = Db::open(directory.path(), None)?;
    assert!(reopened.get("scans", "scan-1").is_some());
    Ok(())
}
