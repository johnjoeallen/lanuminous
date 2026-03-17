use std::{fs, path::PathBuf};

use lanuminous::agent::HostAgentService;

fn temp_stage_dir() -> tempfile::TempDir {
    tempfile::tempdir().expect("temporary directory should be created")
}

#[test]
fn agent_inspects_staged_files() {
    let stage_dir = temp_stage_dir();
    let nested_dir = stage_dir.path().join("nested");
    fs::create_dir_all(&nested_dir).expect("nested stage directory should be created");
    fs::write(stage_dir.path().join("dnsmasq_main.conf"), "port=53\n")
        .expect("dnsmasq artifact should be written");
    fs::write(nested_dir.join("nftables_main.nft"), "table inet lanuminous {}\n")
        .expect("nftables artifact should be written");

    let inspection = HostAgentService
        .inspect_stage_dir(stage_dir.path())
        .expect("stage directory should be inspected");

    assert_eq!(inspection.artifact_count, 2);
    assert!(inspection
        .artifacts
        .iter()
        .any(|artifact| artifact.relative_path == "dnsmasq_main.conf"));
    assert!(inspection
        .artifacts
        .iter()
        .any(|artifact| artifact.relative_path == PathBuf::from("nested/nftables_main.nft").display().to_string()));
}
