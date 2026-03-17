use std::path::PathBuf;

use lanuminous::config::load_site_from_path;

fn example_site() -> PathBuf {
    PathBuf::from(env!("CARGO_MANIFEST_DIR")).join("../examples/site")
}

#[test]
fn loads_example_site_bundle() {
    let bundle = load_site_from_path(example_site()).expect("example config should load");
    assert_eq!(bundle.networks.len(), 4);
    assert_eq!(bundle.hosts.len(), 10);
    assert_eq!(bundle.wifi_aps.aps.len(), 8);
}
