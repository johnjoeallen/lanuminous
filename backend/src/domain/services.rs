use schemars::JsonSchema;
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize, JsonSchema, PartialEq, Eq)]
pub struct DnsConfig {
    pub domain: String,
    pub upstream_servers: Vec<String>,
    pub static_records: Vec<DnsRecord>,
}

#[derive(Debug, Clone, Serialize, Deserialize, JsonSchema, PartialEq, Eq)]
pub struct DnsRecord {
    pub name: String,
    pub address: String,
}

#[derive(Debug, Clone, Serialize, Deserialize, JsonSchema, PartialEq, Eq)]
pub struct DhcpConfig {
    pub default_lease_time: String,
    pub pools: Vec<crate::domain::DhcpPool>,
    pub reservations: Vec<crate::domain::ReservationDef>,
}

#[derive(Debug, Clone, Serialize, Deserialize, JsonSchema, PartialEq, Eq, Default)]
pub struct PortForwardConfig {
    pub rules: Vec<PortForwardRule>,
}

#[derive(Debug, Clone, Serialize, Deserialize, JsonSchema, PartialEq, Eq)]
pub struct PortForwardRule {
    pub name: String,
    pub protocol: PortProtocol,
    pub external_port: u16,
    pub destination_host: String,
    pub destination_port: u16,
    pub source_zone: String,
    pub description: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize, JsonSchema, PartialEq, Eq)]
#[serde(rename_all = "snake_case")]
pub enum PortProtocol {
    Tcp,
    Udp,
}

#[derive(Debug, Clone, Serialize, Deserialize, JsonSchema, PartialEq, Eq, Default)]
pub struct ReverseProxyConfig {
    pub provider: ReverseProxyProvider,
    pub sites: Vec<ReverseProxySite>,
}

#[derive(Debug, Clone, Serialize, Deserialize, JsonSchema, PartialEq, Eq)]
pub struct ReverseProxySite {
    pub name: String,
    pub server_names: Vec<String>,
    pub listen_port: u16,
    pub backend: ProxyBackend,
    pub tls_mode: ProxyTlsMode,
}

#[derive(Debug, Clone, Serialize, Deserialize, JsonSchema, PartialEq, Eq)]
#[serde(rename_all = "snake_case")]
pub enum ReverseProxyProvider {
    Apache2,
    Nginx,
    Caddy,
    Traefik,
    Haproxy,
}

impl Default for ReverseProxyProvider {
    fn default() -> Self {
        Self::Apache2
    }
}

#[derive(Debug, Clone, Serialize, Deserialize, JsonSchema, PartialEq, Eq)]
pub struct ProxyBackend {
    pub host_ref: String,
    pub port: u16,
    pub scheme: ProxyScheme,
}

#[derive(Debug, Clone, Serialize, Deserialize, JsonSchema, PartialEq, Eq)]
#[serde(rename_all = "snake_case")]
pub enum ProxyScheme {
    Http,
    Https,
}

#[derive(Debug, Clone, Serialize, Deserialize, JsonSchema, PartialEq, Eq)]
#[serde(rename_all = "snake_case")]
pub enum ProxyTlsMode {
    #[serde(alias = "terminate_at_apache")]
    TerminateAtProxy,
    PassThrough,
}

#[derive(Debug, Clone, Serialize, Deserialize, JsonSchema, PartialEq, Eq)]
pub struct ServiceDef {
    pub name: String,
    pub service_type: ServiceType,
    pub enabled: bool,
    pub reload_command: Option<String>,
    pub managed_paths: Vec<ManagedPath>,
}

#[derive(Debug, Clone, Serialize, Deserialize, JsonSchema, PartialEq, Eq)]
#[serde(rename_all = "snake_case")]
pub enum ServiceType {
    Dnsmasq,
    Nftables,
    Networking,
    Apache2,
    Nginx,
    Caddy,
    Traefik,
    Haproxy,
    WifiSummary,
    Api,
}

#[derive(Debug, Clone, Serialize, Deserialize, JsonSchema, PartialEq, Eq)]
pub struct ManagedPath {
    pub logical_name: String,
    pub path: String,
    pub service: Option<String>,
}
