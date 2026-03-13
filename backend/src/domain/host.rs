use schemars::JsonSchema;
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize, JsonSchema, PartialEq, Eq)]
pub struct HostDef {
    pub name: String,
    pub role: HostRole,
    pub network: Option<String>,
    pub management_ip: Option<String>,
    pub interfaces: Vec<HostInterfaceDef>,
    pub reservations: Vec<ReservationDef>,
    pub wifi: Option<HostWifiIntent>,
}

#[derive(Debug, Clone, Serialize, Deserialize, JsonSchema, PartialEq, Eq)]
#[serde(rename_all = "snake_case")]
pub enum HostRole {
    Gateway,
    Server,
    AccessPoint,
    Client,
    Service,
}

#[derive(Debug, Clone, Serialize, Deserialize, JsonSchema, PartialEq, Eq)]
pub struct HostInterfaceDef {
    pub name: String,
    pub network: Option<String>,
    pub address: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize, JsonSchema, PartialEq, Eq)]
pub struct ReservationDef {
    pub hostname: String,
    pub ip: String,
    pub mac: String,
}

#[derive(Debug, Clone, Serialize, Deserialize, JsonSchema, PartialEq, Eq)]
pub struct HostWifiIntent {
    pub ap_group: Option<String>,
    pub ssids: Vec<String>,
}
