use anyhow::Result;

use crate::{
    domain::{RenderedArtifact, SiteConfig},
    render::Renderer,
    util::hashing::sha256_string,
};

pub struct NetworkingRenderer;

impl Renderer for NetworkingRenderer {
    fn name(&self) -> &'static str {
        "networking"
    }

    fn render(&self, site: &SiteConfig) -> Result<Vec<RenderedArtifact>> {
        let mut lines = vec![site.metadata.managed_prefix.clone()];

        for interface in &site.interfaces {
            lines.push("[Match]".to_string());
            lines.push(format!("Name={}", interface.name));
            lines.push("[Network]".to_string());
            for address in &interface.addresses {
                lines.push(format!("Address={address}"));
            }
            if !interface.network_refs.is_empty() {
                lines.push(format!("# Networks={}", interface.network_refs.join(",")));
            }
            lines.push(String::new());
        }

        let contents = lines.join("\n") + "\n";
        Ok(vec![RenderedArtifact {
            renderer: self.name().to_string(),
            logical_name: "networking_main".to_string(),
            target_path: "/etc/systemd/network/90-lanuminous.network".to_string(),
            checksum: sha256_string(&contents),
            contents,
        }])
    }
}
