//! Domain foundation for the disk explainer. Scanning is not implemented yet.

use serde::Serialize;

#[derive(Debug, Serialize)]
pub struct Capabilities {
    pub schema_version: u32,
    pub version: &'static str,
    pub stage: &'static str,
    pub storage_engine: &'static str,
    pub implemented_commands: Vec<&'static str>,
    pub planned_commands: Vec<&'static str>,
    pub cleanup_enabled: bool,
}

pub fn capabilities() -> Capabilities {
    Capabilities {
        schema_version: 1,
        version: env!("CARGO_PKG_VERSION"),
        stage: "scaffold",
        storage_engine: "nedb-engine 2.8.5",
        implemented_commands: vec!["about"],
        planned_commands: vec!["scan", "explain", "diff", "plan", "apply"],
        cleanup_enabled: false,
    }
}
