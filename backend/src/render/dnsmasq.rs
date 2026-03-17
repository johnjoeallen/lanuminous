use anyhow::Result;

use crate::{
    domain::{RenderedArtifact, SiteConfig},
    render::Renderer,
    util::hashing::sha256_string,
};

pub struct DnsmasqRenderer;

impl Renderer for DnsmasqRenderer {
    fn name(&self) -> &'static str {
        "dnsmasq"
    }

    fn render(&self, site: &SiteConfig) -> Result<Vec<RenderedArtifact>> {
        let mut lines = vec![site.metadata.managed_prefix.clone()];

        if let Some(dns) = &site.dns {
            lines.push(format!("domain={}", dns.domain));
            for upstream in &dns.upstream_servers {
                lines.push(format!("server={upstream}"));
            }
            for record in &dns.static_records {
                lines.push(format!("address=/{}/{}", record.name, record.address));
            }
        }

        if let Some(dhcp) = &site.dhcp {
            for pool in &dhcp.pools {
                lines.push(format!(
                    "dhcp-range=set:{}, {}, {}, {}",
                    pool.network, pool.start, pool.end, pool.lease_time
                ));
            }
            for reservation in &dhcp.reservations {
                lines.push(format!(
                    "dhcp-host={}, {}, {}",
                    reservation.mac, reservation.hostname, reservation.ip
                ));
            }
        }

        let contents = lines.join("\n") + "\n";
        Ok(vec![RenderedArtifact {
            renderer: self.name().to_string(),
            logical_name: "dnsmasq_main".to_string(),
            target_path: "/etc/dnsmasq.d/lanuminous.conf".to_string(),
            checksum: sha256_string(&contents),
            contents,
        }])
    }
}
