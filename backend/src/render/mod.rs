mod apache2;
mod caddy;
mod dnsmasq;
mod haproxy;
mod networking;
mod nftables;
mod nginx;
mod traefik;

use anyhow::Result;

use crate::domain::{RenderedArtifact, SiteConfig};

pub use apache2::Apache2Renderer;
pub use caddy::CaddyRenderer;
pub use dnsmasq::DnsmasqRenderer;
pub use haproxy::HaproxyRenderer;
pub use networking::NetworkingRenderer;
pub use nftables::NftablesRenderer;
pub use nginx::NginxRenderer;
pub use traefik::TraefikRenderer;

pub trait Renderer {
    fn name(&self) -> &'static str;
    fn render(&self, site: &SiteConfig) -> Result<Vec<RenderedArtifact>>;
}

pub fn render_all(site: &SiteConfig) -> Result<Vec<RenderedArtifact>> {
    let renderers: Vec<Box<dyn Renderer>> = vec![
        Box::new(Apache2Renderer),
        Box::new(NginxRenderer),
        Box::new(CaddyRenderer),
        Box::new(TraefikRenderer),
        Box::new(HaproxyRenderer),
        Box::new(DnsmasqRenderer),
        Box::new(NftablesRenderer),
        Box::new(NetworkingRenderer),
    ];

    let mut artifacts = Vec::new();
    for renderer in renderers {
        artifacts.extend(renderer.render(site)?);
    }
    Ok(artifacts)
}
