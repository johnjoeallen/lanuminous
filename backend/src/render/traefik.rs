use anyhow::Result;

use crate::{
    domain::{RenderedArtifact, ReverseProxyProvider, SiteConfig},
    render::Renderer,
    util::hashing::sha256_string,
};

pub struct TraefikRenderer;

impl Renderer for TraefikRenderer {
    fn name(&self) -> &'static str {
        "traefik"
    }

    fn render(&self, site: &SiteConfig) -> Result<Vec<RenderedArtifact>> {
        if site.reverse_proxies.sites.is_empty()
            || !matches!(site.reverse_proxies.provider, ReverseProxyProvider::Traefik)
        {
            return Ok(Vec::new());
        }

        let mut lines = vec![
            format!("{} ", site.metadata.managed_prefix)
                .trim_end()
                .to_string(),
            "http:".to_string(),
            "  routers:".to_string(),
        ];

        for proxy in &site.reverse_proxies.sites {
            lines.push(format!("    {}:", proxy.name));
            lines.push(format!(
                "      rule: \"Host(`{}`)\"",
                proxy.server_names.join("`,`")
            ));
            lines.push(format!("      service: {}-svc", proxy.name));
        }

        lines.push("  services:".to_string());
        for proxy in &site.reverse_proxies.sites {
            let backend_host = resolve_host_address(site, &proxy.backend.host_ref);
            lines.push(format!("    {}-svc:", proxy.name));
            lines.push("      loadBalancer:".to_string());
            lines.push("        servers:".to_string());
            lines.push(format!(
                "          - url: \"http://{}:{}\"",
                backend_host, proxy.backend.port
            ));
        }

        let contents = lines.join("\n");
        Ok(vec![RenderedArtifact {
            renderer: self.name().to_string(),
            logical_name: "reverse_proxy_main".to_string(),
            target_path: "/etc/traefik/dynamic/lantricate.yml".to_string(),
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
