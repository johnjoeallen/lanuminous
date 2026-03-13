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
    WifiSummary,
    Api,
}

#[derive(Debug, Clone, Serialize, Deserialize, JsonSchema, PartialEq, Eq)]
pub struct ManagedPath {
    pub logical_name: String,
    pub path: String,
    pub service: Option<String>,
}
