use anyhow::Result;

use crate::{
    domain::{RenderedArtifact, ReverseProxyProvider, SiteConfig},
    render::Renderer,
    util::hashing::sha256_string,
};

pub struct HaproxyRenderer;

impl Renderer for HaproxyRenderer {
    fn name(&self) -> &'static str {
        "haproxy"
    }

    fn render(&self, site: &SiteConfig) -> Result<Vec<RenderedArtifact>> {
        if site.reverse_proxies.sites.is_empty()
            || !matches!(site.reverse_proxies.provider, ReverseProxyProvider::Haproxy)
        {
            return Ok(Vec::new());
        }

        let mut lines = vec![site.metadata.managed_prefix.clone(), String::new()];
        for proxy in &site.reverse_proxies.sites {
            let backend_host = resolve_host_address(site, &proxy.backend.host_ref);
            lines.push(format!("frontend {}_front", proxy.name));
            lines.push(format!("    bind *:{}", proxy.listen_port));
            lines.push(format!("    default_backend {}_back", proxy.name));
            lines.push(String::new());
            lines.push(format!("backend {}_back", proxy.name));
            lines.push(format!(
                "    server {} {}:{} check",
                proxy.backend.host_ref, backend_host, proxy.backend.port
            ));
            lines.push(String::new());
        }

        let contents = lines.join("\n");
        Ok(vec![RenderedArtifact {
            renderer: self.name().to_string(),
            logical_name: "reverse_proxy_main".to_string(),
            target_path: "/etc/haproxy/lanuminous.cfg".to_string(),
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
