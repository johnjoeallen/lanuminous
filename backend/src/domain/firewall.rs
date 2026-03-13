use schemars::JsonSchema;
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize, JsonSchema, PartialEq, Eq, Default)]
pub struct FirewallConfig {
    pub zones: Vec<ZoneDef>,
    pub policies: Vec<PolicyRule>,
}

#[derive(Debug, Clone, Serialize, Deserialize, JsonSchema, PartialEq, Eq)]
pub struct ZoneDef {
    pub name: String,
    pub networks: Vec<String>,
    pub description: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize, JsonSchema, PartialEq, Eq)]
pub struct PolicyRule {
    pub name: String,
    pub action: PolicyAction,
    pub source_zone: String,
    pub destination_zone: String,
    pub allowed_services: Vec<String>,
    pub destination_hosts: Vec<String>,
    pub description: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize, JsonSchema, PartialEq, Eq)]
#[serde(rename_all = "snake_case")]
pub enum PolicyAction {
    Accept,
    Reject,
    Drop,
}
