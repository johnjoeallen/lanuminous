use std::collections::BTreeSet;

use anyhow::Result;

use crate::{
    domain::{InterfaceRole, PolicyAction, PortProtocol, RenderedArtifact, SiteConfig},
    render::Renderer,
    util::hashing::sha256_string,
};

pub struct NftablesRenderer;

impl Renderer for NftablesRenderer {
    fn name(&self) -> &'static str {
        "nftables"
    }

    fn render(&self, site: &SiteConfig) -> Result<Vec<RenderedArtifact>> {
        let wan_interfaces = wan_interface_names(site);
        let mut lines = vec![
            site.metadata.managed_prefix.clone(),
            "table inet lanuminous {".to_string(),
            "  chain prerouting {".to_string(),
            "    type nat hook prerouting priority dstnat;".to_string(),
        ];

        for forward in &site.port_forwards.rules {
            let protocol = match forward.protocol {
                PortProtocol::Tcp => "tcp",
                PortProtocol::Udp => "udp",
            };
            let destination_host = resolve_host_address(site, &forward.destination_host);
            let ingress_interfaces = zone_interface_names(site, &forward.source_zone);
            lines.push(format!(
                "    # {}",
                forward.description.as_deref().unwrap_or(&forward.name)
            ));
            for ingress_interface in ingress_interfaces {
                lines.push(format!(
                    "    iifname \"{ingress_interface}\" {protocol} dport {} dnat to {}:{}",
                    forward.external_port, destination_host, forward.destination_port
                ));
            }
        }

        lines.extend([
            "  }".to_string(),
            "  chain postrouting {".to_string(),
            "    type nat hook postrouting priority srcnat;".to_string(),
        ]);

        for wan_interface in &wan_interfaces {
            lines.push(format!("    oifname \"{wan_interface}\" masquerade"));
        }

        lines.extend([
            "  }".to_string(),
            "  chain forward {".to_string(),
            "    type filter hook forward priority 0;".to_string(),
            "    policy drop;".to_string(),
            "    ct state established,related accept".to_string(),
        ]);

        for policy in &site.firewall.policies {
            let source_interfaces = zone_interface_names(site, &policy.source_zone);
            let destination_interfaces = zone_interface_names(site, &policy.destination_zone);
            let verdict = match policy.action {
                PolicyAction::Accept => "accept",
                PolicyAction::Reject => "reject",
                PolicyAction::Drop => "drop",
            };
            let comment = policy.description.as_deref().unwrap_or(&policy.name);
            lines.push(format!(
                "    # {}: {} -> {}",
                comment, policy.source_zone, policy.destination_zone
            ));
            let service_matches = service_matches(&policy.allowed_services);
            let destination_hosts = policy_destination_hosts(site, &policy.destination_hosts);

            for source_interface in &source_interfaces {
                for destination_interface in &destination_interfaces {
                    if service_matches.is_empty() {
                        lines.push(build_forward_rule(
                            source_interface,
                            destination_interface,
                            None,
                            destination_hosts.as_deref(),
                            verdict,
                        ));
                    } else {
                        for service_match in &service_matches {
                            lines.push(build_forward_rule(
                                source_interface,
                                destination_interface,
                                Some(service_match),
                                destination_hosts.as_deref(),
                                verdict,
                            ));
                        }
                    }
                }
            }
        }

        lines.push("  }".to_string());
        lines.push("}".to_string());

        let contents = lines.join("\n") + "\n";
        Ok(vec![RenderedArtifact {
            renderer: self.name().to_string(),
            logical_name: "nftables_main".to_string(),
            target_path: "/etc/nftables.d/lanuminous.nft".to_string(),
            checksum: sha256_string(&contents),
            contents,
        }])
    }
}

fn resolve_host_address(site: &SiteConfig, host_ref: &str) -> String {
    site.hosts
        .iter()
        .find(|host| host.name == host_ref)
        .and_then(|host| host.management_ip.clone())
        .unwrap_or_else(|| host_ref.to_string())
}

fn wan_interface_names(site: &SiteConfig) -> Vec<String> {
    site.interfaces
        .iter()
        .filter(|interface| interface.role == InterfaceRole::Wan)
        .map(|interface| interface.name.clone())
        .collect()
}

fn zone_interface_names(site: &SiteConfig, zone_name: &str) -> Vec<String> {
    if zone_name == "wan" {
        return wan_interface_names(site);
    }

    let mut interface_names = BTreeSet::new();
    let zone_networks: BTreeSet<&str> = site
        .firewall
        .zones
        .iter()
        .find(|zone| zone.name == zone_name)
        .map(|zone| zone.networks.iter().map(String::as_str).collect())
        .unwrap_or_else(|| {
            site.networks
                .iter()
                .filter(|network| network.zone == zone_name)
                .map(|network| network.name.as_str())
                .collect()
        });

    for network_name in zone_networks {
        if let Some(interface_name) = network_interface_name(site, network_name) {
            interface_names.insert(interface_name);
        }
    }

    interface_names.into_iter().collect()
}

fn network_interface_name(site: &SiteConfig, network_name: &str) -> Option<String> {
    if let Some(interface) = site
        .interfaces
        .iter()
        .find(|interface| interface.network_refs.iter().any(|name| name == network_name))
    {
        if let Some(network) = site.networks.iter().find(|network| network.name == network_name) {
            if let Some(vlan) = &network.vlan {
                if let Some(parent) = &vlan.parent_interface {
                    return Some(format!("{parent}.{}", vlan.id));
                }
            }
        }

        return Some(interface.name.clone());
    }

    site.networks
        .iter()
        .find(|network| network.name == network_name)
        .and_then(|network| {
            network.vlan.as_ref().and_then(|vlan| {
                vlan.parent_interface
                    .as_ref()
                    .map(|parent| format!("{parent}.{}", vlan.id))
            })
        })
}

fn service_matches(allowed_services: &[String]) -> Vec<String> {
    let normalized: BTreeSet<&str> = allowed_services.iter().map(String::as_str).collect();
    if normalized.is_empty() || normalized.contains("any") {
        return Vec::new();
    }

    let mut matches = Vec::new();
    if normalized.contains("dns") {
        matches.push("udp dport 53".to_string());
        matches.push("tcp dport 53".to_string());
    }
    if normalized.contains("http") {
        matches.push("tcp dport 80".to_string());
    }
    if normalized.contains("https") {
        matches.push("tcp dport 443".to_string());
    }

    matches
}

fn policy_destination_hosts(site: &SiteConfig, host_refs: &[String]) -> Option<String> {
    let addresses: Vec<String> = host_refs
        .iter()
        .map(|host_ref| resolve_host_address(site, host_ref))
        .collect();

    match addresses.len() {
        0 => None,
        1 => Some(format!("ip daddr {}", addresses[0])),
        _ => Some(format!("ip daddr {{ {} }}", addresses.join(", "))),
    }
}

fn build_forward_rule(
    source_interface: &str,
    destination_interface: &str,
    service_match: Option<&str>,
    destination_hosts: Option<&str>,
    verdict: &str,
) -> String {
    let mut parts = vec![
        format!("iifname \"{source_interface}\""),
        format!("oifname \"{destination_interface}\""),
    ];

    if let Some(destination_hosts) = destination_hosts {
        parts.push(destination_hosts.to_string());
    }

    if let Some(service_match) = service_match {
        parts.push(service_match.to_string());
    }

    parts.push("counter".to_string());
    parts.push(verdict.to_string());
    format!("    {}", parts.join(" "))
}
