use axum::{extract::State, routing::get, Json, Router};
use serde::{Deserialize, Serialize};
use serde_json::{json, Value};

use crate::domain::SiteConfig;

#[derive(Debug, Clone)]
pub struct ApiState {
    pub site: SiteConfig,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct SiteApiView {
    pub name: String,
    pub description: String,
    pub reverse_proxy_provider: String,
    pub networks: Vec<NetworkCard>,
    pub interfaces: Vec<InterfaceCard>,
    pub firewall_policies: Vec<FirewallPolicyCard>,
    pub port_forwards: Vec<PortForwardCard>,
    pub ssids: Vec<SsidCard>,
    pub access_points: Vec<AccessPointCard>,
    pub reverse_proxies: Vec<ReverseProxyCard>,
    pub artifacts: Vec<ArtifactCard>,
    pub deployments: Vec<DeploymentCard>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct NetworkCard {
    pub name: String,
    pub cidr: String,
    pub zone: String,
    pub vlan: Option<u16>,
    pub interface: String,
    pub purpose: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct InterfaceCard {
    pub name: String,
    pub role: String,
    pub addresses: Vec<String>,
    pub network_refs: Vec<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct FirewallPolicyCard {
    pub name: String,
    pub source_zone: String,
    pub destination_zone: String,
    pub action: String,
    pub summary: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct PortForwardCard {
    pub name: String,
    pub protocol: String,
    pub external_port: u16,
    pub destination_host: String,
    pub destination_port: u16,
    pub source_zone: String,
    pub summary: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct SsidCard {
    pub name: String,
    pub vlan: u16,
    pub zone: String,
    pub groups: Vec<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct AccessPointCard {
    pub name: String,
    pub management_ip: String,
    pub group: String,
    pub uplink_port: String,
    pub ssids: Vec<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ReverseProxyCard {
    pub name: String,
    pub provider: String,
    pub server_names: Vec<String>,
    pub listen_port: u16,
    pub backend_host: String,
    pub backend_port: u16,
    pub backend_scheme: String,
    pub tls_mode: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ArtifactCard {
    pub logical_name: String,
    pub target_path: String,
    pub renderer: String,
    pub change_state: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct DeploymentCard {
    pub id: String,
    pub timestamp: String,
    pub status: String,
    pub summary: String,
}

impl SiteApiView {
    pub fn from_site(site: &SiteConfig) -> Self {
        let networks = site
            .networks
            .iter()
            .map(|network| {
                let interface = site
                    .interfaces
                    .iter()
                    .find(|interface| {
                        interface
                            .network_refs
                            .iter()
                            .any(|name| name == &network.name)
                    })
                    .map(|interface| interface.name.clone())
                    .or_else(|| {
                        network.vlan.as_ref().and_then(|vlan| {
                            vlan.parent_interface
                                .as_ref()
                                .map(|parent| format!("{parent}.{}", vlan.id))
                        })
                    })
                    .unwrap_or_else(|| "unassigned".to_string());

                NetworkCard {
                    name: network.name.clone(),
                    cidr: network.cidr.clone(),
                    zone: network.zone.clone(),
                    vlan: network.vlan.as_ref().map(|vlan| vlan.id),
                    interface,
                    purpose: network
                        .dns_domain
                        .clone()
                        .unwrap_or_else(|| "Managed network".to_string()),
                }
            })
            .collect();

        let interfaces = site
            .interfaces
            .iter()
            .map(|interface| InterfaceCard {
                name: interface.name.clone(),
                role: format!("{:?}", interface.role).to_lowercase(),
                addresses: interface.addresses.clone(),
                network_refs: interface.network_refs.clone(),
            })
            .collect();

        let firewall_policies = site
            .firewall
            .policies
            .iter()
            .map(|policy| FirewallPolicyCard {
                name: policy.name.clone(),
                source_zone: policy.source_zone.clone(),
                destination_zone: policy.destination_zone.clone(),
                action: format!("{:?}", policy.action).to_lowercase(),
                summary: policy
                    .description
                    .clone()
                    .unwrap_or_else(|| "Managed policy".to_string()),
            })
            .collect();

        let port_forwards = site
            .port_forwards
            .rules
            .iter()
            .map(|rule| PortForwardCard {
                name: rule.name.clone(),
                protocol: format!("{:?}", rule.protocol).to_lowercase(),
                external_port: rule.external_port,
                destination_host: rule.destination_host.clone(),
                destination_port: rule.destination_port,
                source_zone: rule.source_zone.clone(),
                summary: rule
                    .description
                    .clone()
                    .unwrap_or_else(|| "Managed port forward".to_string()),
            })
            .collect();

        let ssids = site
            .wifi
            .ssids
            .iter()
            .map(|ssid| SsidCard {
                name: ssid.name.clone(),
                vlan: ssid.vlan,
                zone: ssid.zone.clone(),
                groups: ssid.broadcast_groups.clone(),
            })
            .collect();

        let access_points = site
            .wifi
            .access_points
            .iter()
            .map(|ap| AccessPointCard {
                name: ap.name.clone(),
                management_ip: ap.management_ip.clone(),
                group: ap.group.clone().unwrap_or_else(|| "ungrouped".to_string()),
                uplink_port: match &ap.uplink.switch_name {
                    Some(switch_name) => format!("{switch_name} {}", ap.uplink.port),
                    None => ap.uplink.port.clone(),
                },
                ssids: ap.ssids.clone(),
            })
            .collect();

        let reverse_proxies = site
            .reverse_proxies
            .sites
            .iter()
            .map(|proxy| ReverseProxyCard {
                name: proxy.name.clone(),
                provider: format!("{:?}", site.reverse_proxies.provider).to_lowercase(),
                server_names: proxy.server_names.clone(),
                listen_port: proxy.listen_port,
                backend_host: proxy.backend.host_ref.clone(),
                backend_port: proxy.backend.port,
                backend_scheme: format!("{:?}", proxy.backend.scheme).to_lowercase(),
                tls_mode: format!("{:?}", proxy.tls_mode).to_lowercase(),
            })
            .collect();

        let artifacts = site
            .services
            .iter()
            .flat_map(|service| {
                service.managed_paths.iter().map(move |path| ArtifactCard {
                    logical_name: path.logical_name.clone(),
                    target_path: path.path.clone(),
                    renderer: service.name.clone(),
                    change_state: "planned".to_string(),
                })
            })
            .collect();

        Self {
            name: site.metadata.name.clone(),
            description: site
                .metadata
                .description
                .clone()
                .unwrap_or_else(|| "Lantricate managed site".to_string()),
            reverse_proxy_provider: format!("{:?}", site.reverse_proxies.provider).to_lowercase(),
            networks,
            interfaces,
            firewall_policies,
            port_forwards,
            ssids,
            access_points,
            reverse_proxies,
            artifacts,
            deployments: vec![DeploymentCard {
                id: "stage1-preview".to_string(),
                timestamp: chrono::Utc::now().to_rfc3339(),
                status: "planned".to_string(),
                summary: "Preview deployment state from the shared canonical model.".to_string(),
            }],
        }
    }
}

pub fn build_router(site: SiteConfig) -> Router {
    Router::new()
        .route("/healthz", get(healthz))
        .route("/api/site", get(site_summary))
        .with_state(ApiState { site })
}

async fn healthz() -> Json<Value> {
    Json(json!({
        "status": "ok",
        "service": "lantricate-api",
    }))
}

async fn site_summary(State(state): State<ApiState>) -> Json<SiteApiView> {
    Json(SiteApiView::from_site(&state.site))
}
