use anyhow::Result;

use crate::{
    domain::{ProxyScheme, ProxyTlsMode, RenderedArtifact, ReverseProxyProvider, SiteConfig},
    render::Renderer,
    util::hashing::sha256_string,
};

pub struct NginxRenderer;

impl Renderer for NginxRenderer {
    fn name(&self) -> &'static str {
        "nginx"
    }

    fn render(&self, site: &SiteConfig) -> Result<Vec<RenderedArtifact>> {
        if site.reverse_proxies.sites.is_empty()
            || !matches!(site.reverse_proxies.provider, ReverseProxyProvider::Nginx)
        {
            return Ok(Vec::new());
        }

        let mut lines = vec![site.metadata.managed_prefix.clone(), String::new()];

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

            lines.push("server {".to_string());
            lines.push(format!("    listen {};", proxy.listen_port));
            lines.push(format!("    server_name {};", proxy.server_names.join(" ")));
            lines.push(format!("    {tls_comment}"));
            lines.push("    location / {".to_string());
            lines.push(format!(
                "        proxy_pass {backend_scheme}://{}:{};",
                backend_host, proxy.backend.port
            ));
            lines.push("        proxy_set_header Host $host;".to_string());
            lines.push(
                "        proxy_set_header X-Forwarded-For $proxy_add_x_forwarded_for;".to_string(),
            );
            lines.push("        proxy_set_header X-Forwarded-Proto $scheme;".to_string());
            lines.push("    }".to_string());
            lines.push("}".to_string());
            lines.push(String::new());
        }

        let contents = lines.join("\n");
        Ok(vec![RenderedArtifact {
            renderer: self.name().to_string(),
            logical_name: "reverse_proxy_main".to_string(),
            target_path: "/etc/nginx/conf.d/lanuminous-proxies.conf".to_string(),
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
