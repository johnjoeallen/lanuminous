mod dnsmasq;
mod networking;
mod nftables;

use anyhow::Result;

use crate::domain::{RenderedArtifact, SiteConfig};

pub use dnsmasq::DnsmasqRenderer;
pub use networking::NetworkingRenderer;
pub use nftables::NftablesRenderer;

pub trait Renderer {
    fn name(&self) -> &'static str;
    fn render(&self, site: &SiteConfig) -> Result<Vec<RenderedArtifact>>;
}

pub fn render_all(site: &SiteConfig) -> Result<Vec<RenderedArtifact>> {
    let renderers: Vec<Box<dyn Renderer>> = vec![
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
