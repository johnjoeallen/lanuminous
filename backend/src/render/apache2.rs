use anyhow::Result;

use crate::{
    domain::{ProxyScheme, ProxyTlsMode, RenderedArtifact, ReverseProxyProvider, SiteConfig},
    render::Renderer,
    util::hashing::sha256_string,
};

pub struct Apache2Renderer;

impl Renderer for Apache2Renderer {
    fn name(&self) -> &'static str {
        "apache2"
    }

    fn render(&self, site: &SiteConfig) -> Result<Vec<RenderedArtifact>> {
        if site.reverse_proxies.sites.is_empty()
            || !matches!(site.reverse_proxies.provider, ReverseProxyProvider::Apache2)
        {
            return Ok(Vec::new());
        }

        let mut lines = vec![
            site.metadata.managed_prefix.clone(),
            "ProxyPreserveHost On".to_string(),
            "ProxyRequests Off".to_string(),
            String::new(),
        ];

        for proxy in &site.reverse_proxies.sites {
            let backend_scheme = match proxy.backend.scheme {
                ProxyScheme::Http => "http",
                ProxyScheme::Https => "https",
            };
            let backend_host = resolve_host_address(site, &proxy.backend.host_ref);
            let tls_comment = match proxy.tls_mode {
                ProxyTlsMode::TerminateAtProxy => "# TLS terminates at the proxy",
                ProxyTlsMode::PassThrough => "# TLS pass-through modeled for future refinement",
            };

            lines.push(format!("<VirtualHost *:{}>", proxy.listen_port));
            lines.push(format!("  ServerName {}", proxy.server_names[0]));
            for alias in proxy.server_names.iter().skip(1) {
                lines.push(format!("  ServerAlias {alias}"));
            }
            lines.push(format!("  {tls_comment}"));
            lines.push(format!(
                "  ProxyPass / {backend_scheme}://{}:{}/",
                backend_host, proxy.backend.port
            ));
            lines.push(format!(
                "  ProxyPassReverse / {backend_scheme}://{}:{}/",
                backend_host, proxy.backend.port
            ));
            lines.push("</VirtualHost>".to_string());
            lines.push(String::new());
        }

        let contents = lines.join("\n");
        Ok(vec![RenderedArtifact {
            renderer: self.name().to_string(),
            logical_name: "reverse_proxy_main".to_string(),
            target_path: "/etc/apache2/sites-available/lantricate-proxies.conf".to_string(),
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
