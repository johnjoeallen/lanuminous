use schemars::JsonSchema;
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize, JsonSchema, PartialEq, Eq)]
pub struct InterfaceDef {
    pub logical_name: String,
    pub name: String,
    pub role: InterfaceRole,
    pub kind: InterfaceKind,
    pub addresses: Vec<String>,
    pub network_refs: Vec<String>,
    pub vlan_tags: Vec<u16>,
}

#[derive(Debug, Clone, Serialize, Deserialize, JsonSchema, PartialEq, Eq)]
#[serde(rename_all = "snake_case")]
pub enum InterfaceRole {
    Wan,
    Lan,
    WifiUplink,
    Trunk,
    Management,
}

#[derive(Debug, Clone, Serialize, Deserialize, JsonSchema, PartialEq, Eq)]
#[serde(rename_all = "snake_case")]
pub enum InterfaceKind {
    Physical,
    Bridge,
    Vlan,
    Bond,
}

#[derive(Debug, Clone, Serialize, Deserialize, JsonSchema, PartialEq, Eq)]
pub struct NetworkDef {
    pub name: String,
    pub cidr: String,
    pub zone: String,
    pub dns_domain: Option<String>,
    pub vlan: Option<VlanDef>,
    pub dhcp_pool: Option<DhcpPool>,
    pub routes: Vec<RouteDef>,
}

#[derive(Debug, Clone, Serialize, Deserialize, JsonSchema, PartialEq, Eq)]
pub struct VlanDef {
    pub id: u16,
    pub parent_interface: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize, JsonSchema, PartialEq, Eq)]
pub struct RouteDef {
    pub destination: String,
    pub via: String,
    pub metric: Option<u32>,
}

#[derive(Debug, Clone, Serialize, Deserialize, JsonSchema, PartialEq, Eq)]
pub struct SwitchDef {
    pub name: String,
    pub management_ip: Option<String>,
    pub uplinks: Vec<UplinkDef>,
}

#[derive(Debug, Clone, Serialize, Deserialize, JsonSchema, PartialEq, Eq)]
pub struct UplinkDef {
    pub switch_name: Option<String>,
    pub port: String,
    pub native_vlan: Option<u16>,
    pub tagged_vlans: Vec<u16>,
    pub expected_networks: Vec<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize, JsonSchema, PartialEq, Eq)]
pub struct DhcpPool {
    pub network: String,
    pub start: String,
    pub end: String,
    pub lease_time: String,
}
