use schemars::JsonSchema;
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize, JsonSchema, PartialEq, Eq, Default)]
pub struct RemoteAccessConfig {
    pub providers: Vec<RemoteDnsProviderDef>,
    pub publications: Vec<PublicationRule>,
    pub wan_updates: Vec<WanAddressUpdate>,
}

#[derive(Debug, Clone, Serialize, Deserialize, JsonSchema, PartialEq, Eq)]
pub struct RemoteDnsProviderDef {
    pub id: String,
    pub provider: RemoteDnsProviderKind,
    pub credential_ref: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize, JsonSchema, PartialEq, Eq)]
#[serde(rename_all = "snake_case", tag = "type")]
pub enum RemoteDnsProviderKind {
    ManagedSubdomain(ManagedSubdomainProviderDef),
    JokerDynDns(DynamicDnsProviderDef),
    GenericDynDns(DynamicDnsProviderDef),
    Manual(ManualProviderDef),
}

#[derive(Debug, Clone, Serialize, Deserialize, JsonSchema, PartialEq, Eq)]
pub struct ManagedSubdomainProviderDef {
    pub zone: String,
}

#[derive(Debug, Clone, Serialize, Deserialize, JsonSchema, PartialEq, Eq)]
pub struct DynamicDnsProviderDef {
    pub hostname: String,
    pub service: Option<String>,
    pub update_url: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize, JsonSchema, PartialEq, Eq, Default)]
pub struct ManualProviderDef {
    pub base_domain: Option<String>,
    pub note: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize, JsonSchema, PartialEq, Eq)]
pub struct PublicationRule {
    pub target: PublicationTarget,
    pub enabled: bool,
    pub provider: Option<String>,
    pub publish_as: Option<String>,
    pub protocol: PublicationProtocol,
    pub target_port: u16,
    pub audience: PublicationAudience,
    pub exposure_mode: ExposureMode,
}

#[derive(Debug, Clone, Serialize, Deserialize, JsonSchema, PartialEq, Eq)]
#[serde(rename_all = "snake_case")]
pub enum PublicationTarget {
    Service(String),
}

impl PublicationTarget {
    pub fn service_name(&self) -> &str {
        match self {
            Self::Service(service) => service,
        }
    }
}

#[derive(Debug, Clone, Serialize, Deserialize, JsonSchema, PartialEq, Eq)]
#[serde(rename_all = "snake_case")]
pub enum PublicationAudience {
    Private,
    Family,
    AdminOnly,
    Public,
}

#[derive(Debug, Clone, Serialize, Deserialize, JsonSchema, PartialEq, Eq)]
#[serde(rename_all = "snake_case")]
pub enum ExposureMode {
    Direct,
    Tunnel,
    Manual,
}

#[derive(Debug, Clone, Serialize, Deserialize, JsonSchema, PartialEq, Eq)]
#[serde(rename_all = "snake_case")]
pub enum PublicationProtocol {
    Http,
    Https,
    Tcp,
    Udp,
}

#[derive(Debug, Clone, Serialize, Deserialize, JsonSchema, PartialEq, Eq)]
pub struct WanAddressUpdate {
    pub name: String,
    pub enabled: bool,
    pub provider: String,
    pub hostname: String,
    pub audience: PublicationAudience,
}

#[derive(Debug, Clone, Serialize, Deserialize, JsonSchema, PartialEq, Eq)]
pub struct DesiredPublication {
    pub service: String,
    pub provider: String,
    pub external_name: String,
    pub protocol: PublicationProtocol,
    pub target_port: u16,
    pub audience: PublicationAudience,
    pub exposure_mode: ExposureMode,
    pub target: PublicationTarget,
    pub target_address: String,
}

#[derive(Debug, Clone, Serialize, Deserialize, JsonSchema, PartialEq, Eq)]
pub struct DesiredDnsRecord {
    pub provider: String,
    pub name: String,
    pub record_type: DesiredDnsRecordType,
    pub value_source: DesiredDnsValueSource,
    pub ttl: Option<u32>,
    pub purpose: DesiredDnsPurpose,
}

#[derive(Debug, Clone, Serialize, Deserialize, JsonSchema, PartialEq, Eq)]
#[serde(rename_all = "snake_case")]
pub enum DesiredDnsRecordType {
    A,
    Aaaa,
    Cname,
}

#[derive(Debug, Clone, Serialize, Deserialize, JsonSchema, PartialEq, Eq)]
#[serde(rename_all = "snake_case")]
pub enum DesiredDnsValueSource {
    WanAddress,
    Static(String),
}

#[derive(Debug, Clone, Serialize, Deserialize, JsonSchema, PartialEq, Eq)]
#[serde(rename_all = "snake_case")]
pub enum DesiredDnsPurpose {
    ServicePublication,
    WanSync,
}

#[derive(Debug, Clone, Serialize, Deserialize, JsonSchema, PartialEq, Eq, Default)]
pub struct PublicationPlan {
    pub publications: Vec<DesiredPublication>,
    pub dns_records: Vec<DesiredDnsRecord>,
    pub wan_updates: Vec<WanAddressUpdate>,
    pub warnings: Vec<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize, JsonSchema, PartialEq, Eq)]
pub struct PublicationResult {
    pub provider: String,
    pub action: String,
    pub status: PublicationStatus,
    pub message: String,
}

#[derive(Debug, Clone, Serialize, Deserialize, JsonSchema, PartialEq, Eq)]
#[serde(rename_all = "snake_case")]
pub enum PublicationStatus {
    Planned,
    Applied,
    Removed,
    NotImplemented,
    Failed,
}
