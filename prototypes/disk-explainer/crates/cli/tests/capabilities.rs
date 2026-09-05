use std::process::Command;

#[test]
fn json_capabilities_do_not_claim_unimplemented_features() {
    let output = Command::new(env!("CARGO_BIN_EXE_diskwhy"))
        .args(["about", "--json"])
        .output()
        .expect("CLI starts");
    assert!(output.status.success());
    let value: serde_json::Value = serde_json::from_slice(&output.stdout).unwrap();
    assert_eq!(value["stage"], "scaffold");
    assert_eq!(value["cleanup_enabled"], false);
    assert_eq!(value["implemented_commands"], serde_json::json!(["about"]));
}

#[test]
fn unsupported_cleanup_fails_instead_of_pretending_success() {
    let output = Command::new(env!("CARGO_BIN_EXE_diskwhy"))
        .arg("apply")
        .output()
        .expect("CLI starts");
    assert!(!output.status.success());
    assert!(output.stdout.is_empty());
}
