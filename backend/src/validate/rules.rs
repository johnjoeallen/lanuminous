use std::collections::HashSet;

use crate::{
    domain::SiteConfig,
    validate::{IssueSeverity, ValidationIssue},
};

pub fn validate_site(site: &SiteConfig) -> Vec<ValidationIssue> {
    let mut issues = Vec::new();

    if site.metadata.name.trim().is_empty() {
        issues.push(error("metadata.name", "site name must not be empty"));
    }

    issues.extend(check_unique(
        site.networks.iter().map(|network| network.name.as_str()),
        "networks",
        "network names must be unique",
    ));
    issues.extend(check_unique(
        site.hosts.iter().map(|host| host.name.as_str()),
        "hosts",
        "host names must be unique",
    ));

    let network_names = site
        .networks
        .iter()
        .map(|network| network.name.clone())
        .collect::<HashSet<_>>();
    let zone_names = site
        .firewall
        .zones
        .iter()
        .map(|zone| zone.name.clone())
        .collect::<HashSet<_>>();
    let group_names = site
        .wifi
        .groups
        .iter()
        .map(|group| group.name.clone())
        .collect::<HashSet<_>>();
    let host_names = site
        .hosts
        .iter()
        .map(|host| host.name.clone())
        .collect::<HashSet<_>>();

    for interface in &site.interfaces {
        for network in &interface.network_refs {
            if !network_names.contains(network) {
                issues.push(error(
                    &format!("interfaces.{}", interface.name),
                    &format!("unknown network reference `{network}`"),
                ));
            }
        }
    }

    for host in &site.hosts {
        if let Some(network) = &host.network {
            if !network_names.contains(network) {
                issues.push(error(
                    &format!("hosts.{}", host.name),
                    &format!("unknown host network `{network}`"),
                ));
            }
        }

        for iface in &host.interfaces {
            if let Some(network) = &iface.network {
                if !network_names.contains(network) {
                    issues.push(error(
                        &format!("hosts.{}.interfaces.{}", host.name, iface.name),
                        &format!("unknown host interface network `{network}`"),
                    ));
                }
            }
        }
    }

    for policy in &site.firewall.policies {
        if !zone_names.contains(&policy.source_zone) {
            issues.push(error(
                &format!("firewall.policies.{}", policy.name),
                &format!("unknown source zone `{}`", policy.source_zone),
            ));
        }
        if !zone_names.contains(&policy.destination_zone) {
            issues.push(error(
                &format!("firewall.policies.{}", policy.name),
                &format!("unknown destination zone `{}`", policy.destination_zone),
            ));
        }
    }

    for ssid in &site.wifi.ssids {
        if !zone_names.contains(&ssid.zone) {
            issues.push(error(
                &format!("wifi.ssids.{}", ssid.name),
                &format!("unknown firewall zone `{}`", ssid.zone),
            ));
        }
        for group in &ssid.broadcast_groups {
            if !group_names.contains(group) {
                issues.push(error(
                    &format!("wifi.ssids.{}", ssid.name),
                    &format!("unknown AP group `{group}`"),
                ));
            }
        }
    }

    for ap in &site.wifi.access_points {
        if let Some(group) = &ap.group {
            if !group_names.contains(group) {
                issues.push(error(
                    &format!("wifi.access_points.{}", ap.name),
                    &format!("unknown AP group `{group}`"),
                ));
            }
        }
    }

    for rule in &site.port_forwards.rules {
        if !zone_names.contains(&rule.source_zone) {
            issues.push(error(
                &format!("port_forwards.{}", rule.name),
                &format!("unknown source zone `{}`", rule.source_zone),
            ));
        }
        if !host_names.contains(&rule.destination_host) {
            issues.push(error(
                &format!("port_forwards.{}", rule.name),
                &format!("unknown destination host `{}`", rule.destination_host),
            ));
        }
    }

    let mut server_names = HashSet::new();
    for proxy in &site.reverse_proxies.sites {
        if !host_names.contains(&proxy.backend.host_ref) {
            issues.push(error(
                &format!("reverse_proxies.{}", proxy.name),
                &format!("unknown backend host `{}`", proxy.backend.host_ref),
            ));
        }

        for server_name in &proxy.server_names {
            if !server_names.insert(server_name.clone()) {
                issues.push(error(
                    &format!("reverse_proxies.{}", proxy.name),
                    &format!("duplicate apache server name `{server_name}`"),
                ));
            }
        }
    }

    issues
}

fn check_unique<'a>(
    values: impl Iterator<Item = &'a str>,
    path: &str,
    message: &str,
) -> Vec<ValidationIssue> {
    let mut seen = HashSet::new();
    let mut issues = Vec::new();

    for value in values {
        if !seen.insert(value.to_string()) {
            issues.push(error(path, &format!("{message}: `{value}`")));
        }
    }

    issues
}

fn error(path: &str, message: &str) -> ValidationIssue {
    ValidationIssue {
        severity: IssueSeverity::Error,
        path: path.to_string(),
        message: message.to_string(),
    }
}
