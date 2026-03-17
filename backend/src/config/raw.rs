use serde::Deserialize;

use crate::domain::{
    ApBackend, ApController, HostRole, InterfaceKind, InterfaceRole, PolicyAction,
};

#[derive(Debug, Clone, Deserialize)]
pub struct SiteFile {
    pub metadata: RawMetadata,
}

#[derive(Debug, Clone, Deserialize)]
pub struct RawMetadata {
    pub name: String,
    pub description: Option<String>,
    pub managed_prefix: Option<String>,
}

#[derive(Debug, Clone, Deserialize)]
pub struct InterfacesFile {
    pub interfaces: Vec<RawInterface>,
}

#[derive(Debug, Clone, Deserialize)]
pub struct RawInterface {
    pub logical_name: Option<String>,
    pub name: String,
    pub role: InterfaceRole,
    pub kind: InterfaceKind,
    #[serde(default)]
    pub addresses: Vec<String>,
    #[serde(default)]
    pub network_refs: Vec<String>,
    #[serde(default)]
    pub vlan_tags: Vec<u16>,
}

#[derive(Debug, Clone, Deserialize)]
pub struct RawNetworkFile {
    pub name: String,
    pub cidr: String,
    pub zone: Option<String>,
    pub description: Option<String>,
    pub dns_domain: Option<String>,
    pub vlan: Option<u16>,
    pub parent_interface: Option<String>,
    pub dhcp: Option<RawDhcpPool>,
    #[serde(default)]
    pub routes: Vec<RawRoute>,
}

#[derive(Debug, Clone, Deserialize)]
pub struct RawRoute {
    pub destination: String,
    pub via: String,
    pub metric: Option<u32>,
}

#[derive(Debug, Clone, Deserialize)]
pub struct RawDhcpPool {
    pub start: String,
    pub end: String,
    pub lease_time: String,
}

#[derive(Debug, Clone, Deserialize)]
pub struct RawHostFile {
    pub name: String,
    pub role: HostRole,
    pub network: Option<String>,
    pub management_ip: Option<String>,
    #[serde(default)]
    pub interfaces: Vec<RawHostInterface>,
    #[serde(default)]
    pub reservations: Vec<RawReservation>,
    pub wifi: Option<RawHostWifiIntent>,
}

#[derive(Debug, Clone, Deserialize)]
pub struct RawHostInterface {
    pub name: String,
    pub network: Option<String>,
    pub address: Option<String>,
}

#[derive(Debug, Clone, Deserialize)]
pub struct RawReservation {
    pub hostname: String,
    pub ip: String,
    pub mac: String,
}

#[derive(Debug, Clone, Deserialize)]
pub struct RawHostWifiIntent {
    pub ap_group: Option<String>,
    #[serde(default)]
    pub ssids: Vec<String>,
}

#[derive(Debug, Clone, Deserialize)]
pub struct RawSsidsFile {
    #[serde(default)]
    pub controller: Option<ApController>,
    #[serde(default)]
    pub expose_all_ssids_on_all_aps: bool,
    pub ssids: Vec<RawSsid>,
}

#[derive(Debug, Clone, Deserialize)]
pub struct RawSsid {
    pub name: String,
    pub vlan: u16,
    pub zone: String,
    #[serde(default)]
    pub broadcast_groups: Vec<String>,
}

#[derive(Debug, Clone, Deserialize)]
pub struct RawApsFile {
    pub aps: Vec<RawAp>,
}

#[derive(Debug, Clone, Deserialize)]
pub struct RawAp {
    pub name: String,
    pub management_ip: String,
    pub group: Option<String>,
    pub backend: Option<ApBackend>,
    pub uplink: RawUplink,
    #[serde(default)]
    pub ssids: Vec<String>,
}

#[derive(Debug, Clone, Deserialize)]
pub struct RawGroupsFile {
    pub groups: Vec<RawApGroup>,
}

#[derive(Debug, Clone, Deserialize)]
pub struct RawApGroup {
    pub name: String,
    pub description: Option<String>,
    #[serde(default)]
    pub ssids: Vec<String>,
    #[serde(default)]
    pub ap_names: Vec<String>,
}

#[derive(Debug, Clone, Deserialize)]
pub struct RawUplink {
    pub switch_name: Option<String>,
    pub port: String,
    pub native_vlan: Option<u16>,
    #[serde(default)]
    pub tagged_vlans: Vec<u16>,
    #[serde(default)]
    pub expected_networks: Vec<String>,
}

#[derive(Debug, Clone, Deserialize)]
pub struct RawZonesFile {
    pub zones: Vec<RawZone>,
}

#[derive(Debug, Clone, Deserialize)]
pub struct RawZone {
    pub name: String,
    #[serde(default)]
    pub networks: Vec<String>,
    pub description: Option<String>,
}

#[derive(Debug, Clone, Deserialize)]
pub struct RawPoliciesFile {
    pub policies: Vec<RawPolicy>,
}

#[derive(Debug, Clone, Deserialize)]
pub struct RawPolicy {
    pub name: String,
    pub action: PolicyAction,
    pub source_zone: String,
    pub destination_zone: String,
    #[serde(default)]
    pub allowed_services: Vec<String>,
    #[serde(default)]
    pub destination_hosts: Vec<String>,
    pub description: Option<String>,
}

#[derive(Debug, Clone, Deserialize)]
pub struct RawDnsFile {
    pub domain: String,
    #[serde(default)]
    pub upstream_servers: Vec<String>,
    #[serde(default)]
    pub static_records: Vec<RawDnsRecord>,
}

#[derive(Debug, Clone, Deserialize)]
pub struct RawDnsRecord {
    pub name: String,
    pub address: String,
}

#[derive(Debug, Clone, Deserialize)]
pub struct RawDhcpFile {
    pub default_lease_time: String,
}

#[derive(Debug, Clone, Deserialize)]
pub struct RawServiceFile {
    pub name: String,
    pub enabled: bool,
    pub reload_command: Option<String>,
}

#[derive(Debug, Clone, Deserialize, Default)]
pub struct RawPortForwardsFile {
    #[serde(default)]
    pub rules: Vec<RawPortForwardRule>,
}

#[derive(Debug, Clone, Deserialize)]
pub struct RawPortForwardRule {
    pub name: String,
    pub protocol: crate::domain::PortProtocol,
    pub external_port: u16,
    pub destination_host: String,
    pub destination_port: u16,
    pub source_zone: String,
    pub description: Option<String>,
}

#[derive(Debug, Clone, Deserialize, Default)]
pub struct RawReverseProxyFile {
    #[serde(default)]
    pub provider: crate::domain::ReverseProxyProvider,
    #[serde(default)]
    pub sites: Vec<RawReverseProxySite>,
}

#[derive(Debug, Clone, Deserialize)]
pub struct RawReverseProxySite {
    pub name: String,
    pub server_names: Vec<String>,
    pub listen_port: u16,
    pub backend: RawProxyBackend,
    pub tls_mode: crate::domain::ProxyTlsMode,
}

#[derive(Debug, Clone, Deserialize)]
pub struct RawProxyBackend {
    pub host_ref: String,
    pub port: u16,
    pub scheme: crate::domain::ProxyScheme,
}
