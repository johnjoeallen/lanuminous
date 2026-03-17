use std::path::PathBuf;

use lanuminous::{
    app::SiteService,
    domain::{PublicationAudience, PublicationTarget},
};

fn example_site() -> PathBuf {
    PathBuf::from(env!("CARGO_MANIFEST_DIR")).join("../examples/site")
}

#[test]
fn remote_access_plan_derives_publications_and_wan_updates() {
    let service = SiteService;
    let site = service
        .load_site(example_site())
        .expect("example config should load");

    let plan = service
        .plan_remote_access(&site)
        .expect("remote access plan should be derivable");

    assert_eq!(plan.publications.len(), 1);
    assert_eq!(plan.wan_updates.len(), 1);
    assert!(plan
        .publications
        .iter()
        .any(
            |publication| publication.external_name == "jellyfin.rivia-demo.lanuminous.net"
                && publication.target == PublicationTarget::Service("jellyfin".to_string())
        ));
    assert!(plan
        .dns_records
        .iter()
        .any(|record| record.name == "vpn.rivia.example.net"));
}

#[test]
fn remote_access_validation_rejects_duplicate_external_names() {
    let service = SiteService;
    let mut site = service
        .load_site(example_site())
        .expect("example config should load");

    site.remote_access
        .publications
        .push(site.remote_access.publications[0].clone());
    site.remote_access.publications[1].enabled = true;
    site.remote_access.publications[1].target = PublicationTarget::Service("gateway".to_string());
    site.remote_access.publications[1].provider = Some("lanuminous".to_string());
    site.remote_access.publications[1].publish_as = Some("jellyfin".to_string());
    site.remote_access.publications[1].target_port = 443;
    site.remote_access.publications[1].audience = PublicationAudience::AdminOnly;

    let report = service.validate_site(&site);

    assert!(report.issues.iter().any(|issue| issue
        .message
        .contains("duplicate external publication name")));
}

#[test]
fn remote_access_status_returns_provider_actions() {
    let service = SiteService;
    let site = service
        .load_site(example_site())
        .expect("example config should load");

    let status = service
        .remote_access_status(&site)
        .expect("remote access status should be available");

    assert!(status
        .iter()
        .any(|entry| entry.provider == "lanuminous" && entry.action == "apply_publications"));
    assert!(status
        .iter()
        .any(|entry| entry.provider == "joker" && entry.action == "sync_wan_address"));
}
