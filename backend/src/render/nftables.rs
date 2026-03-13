use anyhow::Result;

use crate::{
    domain::{PolicyAction, RenderedArtifact, SiteConfig},
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
            "table inet lantricate {".to_string(),
            "  chain forward {".to_string(),
            "    type filter hook forward priority 0;".to_string(),
            "    policy drop;".to_string(),
        ];

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
            target_path: "/etc/nftables.d/lantricate.nft".to_string(),
            checksum: sha256_string(&contents),
            contents,
        }])
    }
}
