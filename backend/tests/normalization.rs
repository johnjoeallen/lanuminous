use std::path::PathBuf;

use lanuminous::config::{load_site_from_path, normalize_bundle};

fn example_site() -> PathBuf {
    PathBuf::from(env!("CARGO_MANIFEST_DIR")).join("../examples/site")
}

#[test]
fn normalizes_bundle_into_canonical_model() {
    let bundle = load_site_from_path(example_site()).expect("example config should load");
    let site = normalize_bundle(bundle);

    assert_eq!(site.metadata.name, "Rivia");
    assert_eq!(site.networks.len(), 5);
    assert_eq!(site.firewall.policies.len(), 7);
    assert_eq!(site.wifi.ssids.len(), 4);
    assert_eq!(site.port_forwards.rules.len(), 1);
    assert_eq!(site.reverse_proxies.sites.len(), 1);
    assert_eq!(site.remote_access.providers.len(), 2);
    assert_eq!(site.remote_access.publications.len(), 2);
    assert_eq!(site.remote_access.wan_updates.len(), 1);
    assert!(site
        .services
        .iter()
        .any(|service| service.name == "dnsmasq"));
}
