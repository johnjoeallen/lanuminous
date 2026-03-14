use std::path::PathBuf;

use lantricate::{app::SiteService, validate::IssueSeverity};

fn example_site() -> PathBuf {
    PathBuf::from(env!("CARGO_MANIFEST_DIR")).join("../examples/site")
}

#[test]
fn example_site_validates_cleanly() {
    let service = SiteService;
    let site = service
        .load_site(example_site())
        .expect("example config should load");
    let report = service.validate_site(&site);

    assert!(report.is_valid());
    assert!(report.issues.is_empty());
}

#[test]
fn validation_detects_missing_network_reference() {
    let service = SiteService;
    let mut site = service
        .load_site(example_site())
        .expect("example config should load");
    site.hosts[0].network = Some("missing-network".to_string());

    let report = service.validate_site(&site);

    assert!(report
        .issues
        .iter()
        .any(|issue| issue.severity == IssueSeverity::Error
            && issue.message.contains("missing-network")));
}

#[test]
fn validation_detects_unknown_proxy_backend_host() {
    let service = SiteService;
    let mut site = service
        .load_site(example_site())
        .expect("example config should load");
    site.reverse_proxies.sites[0].backend.host_ref = "missing-host".to_string();

    let report = service.validate_site(&site);

    assert!(report
        .issues
        .iter()
        .any(|issue| issue.severity == IssueSeverity::Error
            && issue.message.contains("missing-host")));
}
