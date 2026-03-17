use anyhow::Result;

use crate::{
    domain::{PolicyAction, PortProtocol, RenderedArtifact, SiteConfig},
    render::Renderer,
    util::hashing::sha256_string,
};

pub struct NftablesRenderer;

impl Renderer for NftablesRenderer {
    fn name(&self) -> &'static str {
        "nftables"
    }

    fn render(&self, site: &SiteConfig) -> Result<Vec<RenderedArtifact>> {
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
            lines.push(format!(
                "    # {}",
                forward.description.as_deref().unwrap_or(&forward.name)
            ));
            lines.push(format!(
                "    iifname \"wan\" {protocol} dport {} dnat to {}:{}",
                forward.external_port, destination_host, forward.destination_port
            ));
        }

        lines.extend([
            "  }".to_string(),
            "  chain forward {".to_string(),
            "    type filter hook forward priority 0;".to_string(),
            "    policy drop;".to_string(),
        ]);

        for policy in &site.firewall.policies {
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
            lines.push(format!("    counter {verdict}"));
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
