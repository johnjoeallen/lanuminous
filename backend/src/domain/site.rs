use schemars::JsonSchema;
use serde::{Deserialize, Serialize};

use crate::domain::{
    DhcpConfig, DnsConfig, FirewallConfig, HostDef, InterfaceDef, Metadata, NetworkDef,
    PortForwardConfig, RemoteAccessConfig, ReverseProxyConfig, ServiceDef, SwitchDef, WifiConfig,
};

#[derive(Debug, Clone, Serialize, Deserialize, JsonSchema, PartialEq, Eq)]
pub struct SiteConfig {
    pub metadata: Metadata,
    pub interfaces: Vec<InterfaceDef>,
    pub networks: Vec<NetworkDef>,
    pub hosts: Vec<HostDef>,
    pub services: Vec<ServiceDef>,
    pub dns: Option<DnsConfig>,
    pub dhcp: Option<DhcpConfig>,
    pub port_forwards: PortForwardConfig,
    pub reverse_proxies: ReverseProxyConfig,
    pub remote_access: RemoteAccessConfig,
    pub firewall: FirewallConfig,
    pub wifi: WifiConfig,
    pub switches: Vec<SwitchDef>,
}
