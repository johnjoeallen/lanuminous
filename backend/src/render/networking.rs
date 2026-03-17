use anyhow::Result;

use crate::{
    domain::{InterfaceDef, RenderedArtifact, SiteConfig},
    render::Renderer,
    util::hashing::sha256_string,
};

pub struct NetworkingRenderer;

impl Renderer for NetworkingRenderer {
    fn name(&self) -> &'static str {
        "networking"
    }

    fn render(&self, site: &SiteConfig) -> Result<Vec<RenderedArtifact>> {
        let mut artifacts = Vec::new();

        for interface in &site.interfaces {
            let link_contents = vec![
                site.metadata.managed_prefix.clone(),
                "[Match]".to_string(),
                format!("OriginalName={}", interface.name),
                "[Link]".to_string(),
                format!("Name={}", interface.logical_name),
                String::new(),
            ]
            .join("\n");
            artifacts.push(RenderedArtifact {
                renderer: self.name().to_string(),
                logical_name: format!("networking_link_{}", interface.logical_name),
                target_path: format!("/etc/systemd/network/10-{}.link", interface.logical_name),
                checksum: sha256_string(&(link_contents.clone() + "\n")),
                contents: link_contents + "\n",
            });

            let mut network_lines = vec![
                site.metadata.managed_prefix.clone(),
                "[Match]".to_string(),
                format!("Name={}", interface.logical_name),
                "[Network]".to_string(),
            ];

            for address in physical_interface_addresses(site, interface) {
                network_lines.push(format!("Address={address}"));
            }

            for network_name in vlan_network_names(site, interface) {
                network_lines.push(format!("VLAN={network_name}"));
            }

            if !interface.network_refs.is_empty() {
                network_lines.push(format!("# Networks={}", interface.network_refs.join(",")));
            }
            network_lines.push(String::new());

            let network_contents = network_lines.join("\n") + "\n";
            artifacts.push(RenderedArtifact {
                renderer: self.name().to_string(),
                logical_name: format!("networking_main_{}", interface.logical_name),
                target_path: format!("/etc/systemd/network/20-{}.network", interface.logical_name),
                checksum: sha256_string(&network_contents),
                contents: network_contents,
            });
        }

        for network in site.networks.iter().filter(|network| network.vlan.is_some()) {
            let netdev_contents = vec![
                site.metadata.managed_prefix.clone(),
                "[NetDev]".to_string(),
                format!("Name={}", network.name),
                "Kind=vlan".to_string(),
                "[VLAN]".to_string(),
                format!("Id={}", network.vlan.as_ref().expect("vlan checked").id),
                String::new(),
            ]
            .join("\n")
                + "\n";
            artifacts.push(RenderedArtifact {
                renderer: self.name().to_string(),
                logical_name: format!("networking_vlan_netdev_{}", network.name),
                target_path: format!("/etc/systemd/network/30-{}.netdev", network.name),
                checksum: sha256_string(&netdev_contents),
                contents: netdev_contents,
            });

            let vlan_network_contents = vec![
                site.metadata.managed_prefix.clone(),
                "[Match]".to_string(),
                format!("Name={}", network.name),
                "[Network]".to_string(),
                format!("Address={}", default_gateway_address(&network.cidr)),
                String::new(),
            ]
            .join("\n")
                + "\n";
            artifacts.push(RenderedArtifact {
                renderer: self.name().to_string(),
                logical_name: format!("networking_vlan_network_{}", network.name),
                target_path: format!("/etc/systemd/network/40-{}.network", network.name),
                checksum: sha256_string(&vlan_network_contents),
                contents: vlan_network_contents,
            });
        }

        Ok(artifacts)
    }
}

fn physical_interface_addresses(site: &SiteConfig, interface: &InterfaceDef) -> Vec<String> {
    let vlan_backed_networks: Vec<&str> = site
        .networks
        .iter()
        .filter(|network| network.vlan.is_some())
        .map(|network| network.name.as_str())
        .collect();

    if interface
        .network_refs
        .iter()
        .all(|network_ref| vlan_backed_networks.contains(&network_ref.as_str()))
    {
        return interface
            .addresses
            .iter()
            .filter(|address| address.as_str() == "dhcp")
            .cloned()
            .collect();
    }

    interface.addresses.clone()
}

fn vlan_network_names(site: &SiteConfig, interface: &InterfaceDef) -> Vec<String> {
    interface
        .network_refs
        .iter()
        .filter(|network_ref| {
            site.networks
                .iter()
                .any(|network| network.name == **network_ref && network.vlan.is_some())
        })
        .cloned()
        .collect()
}

fn default_gateway_address(cidr: &str) -> String {
    let (network_ip, prefix) = cidr
        .split_once('/')
        .expect("cidr should include prefix length");
    let mut octets: Vec<u8> = network_ip
        .split('.')
        .map(|octet| octet.parse::<u8>().expect("ipv4 octet should parse"))
        .collect();
    if let Some(last) = octets.last_mut() {
        *last = last.saturating_add(1);
    }
    format!(
        "{}.{}.{}.{}/{}",
        octets[0], octets[1], octets[2], octets[3], prefix
    )
}
