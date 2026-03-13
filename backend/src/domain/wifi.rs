use schemars::JsonSchema;
use serde::{Deserialize, Serialize};

use crate::domain::UplinkDef;

#[derive(Debug, Clone, Serialize, Deserialize, JsonSchema, PartialEq, Eq)]
pub struct WifiConfig {
    pub controller: ApController,
    pub ssids: Vec<SsidDef>,
    pub access_points: Vec<AccessPointDef>,
    pub groups: Vec<AccessPointGroupDef>,
}

impl Default for WifiConfig {
    fn default() -> Self {
        Self {
            controller: ApController::Manual,
            ssids: Vec::new(),
            access_points: Vec::new(),
            groups: Vec::new(),
        }
    }
}

#[derive(Debug, Clone, Serialize, Deserialize, JsonSchema, PartialEq, Eq)]
#[serde(rename_all = "snake_case")]
pub enum ApController {
    Manual,
    Unifi,
    Omada,
    OpenWrt,
}

#[derive(Debug, Clone, Serialize, Deserialize, JsonSchema, PartialEq, Eq)]
pub struct SsidDef {
    pub name: String,
    pub vlan: u16,
    pub zone: String,
    pub broadcast_groups: Vec<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize, JsonSchema, PartialEq, Eq)]
pub struct AccessPointDef {
    pub name: String,
    pub management_ip: String,
    pub group: Option<String>,
    pub backend: ApBackend,
    pub uplink: UplinkDef,
    pub ssids: Vec<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize, JsonSchema, PartialEq, Eq)]
#[serde(rename_all = "snake_case")]
pub enum ApBackend {
    Manual,
    Unifi,
    Omada,
    OpenWrt,
}

#[derive(Debug, Clone, Serialize, Deserialize, JsonSchema, PartialEq, Eq)]
pub struct AccessPointGroupDef {
    pub name: String,
    pub ssids: Vec<String>,
    pub ap_names: Vec<String>,
    pub description: Option<String>,
}
