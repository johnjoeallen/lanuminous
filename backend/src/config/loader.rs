use std::{
    fs,
    path::{Path, PathBuf},
};

use thiserror::Error;
use walkdir::WalkDir;

use crate::config::raw::{
    InterfacesFile, RawApsFile, RawDhcpFile, RawDnsFile, RawGroupsFile, RawHostFile,
    RawNetworkFile, RawPoliciesFile, RawServiceFile, RawSsidsFile, RawZonesFile, SiteFile,
};

#[derive(Debug, Clone)]
pub struct ConfigBundle {
    pub site: SiteFile,
    pub interfaces: InterfacesFile,
    pub networks: Vec<RawNetworkFile>,
    pub hosts: Vec<RawHostFile>,
    pub wifi_ssids: RawSsidsFile,
    pub wifi_aps: RawApsFile,
    pub wifi_groups: RawGroupsFile,
    pub firewall_zones: RawZonesFile,
    pub firewall_policies: RawPoliciesFile,
    pub dns: RawDnsFile,
    pub dhcp: RawDhcpFile,
    pub nftables_service: Option<RawServiceFile>,
}

#[derive(Debug, Error)]
pub enum ConfigError {
    #[error("missing required config file: {0}")]
    MissingFile(String),
    #[error("failed to read {path}: {source}")]
    ReadFailed {
        path: PathBuf,
        #[source]
        source: std::io::Error,
    },
    #[error("failed to parse YAML {path}: {source}")]
    ParseFailed {
        path: PathBuf,
        #[source]
        source: serde_yaml::Error,
    },
}

pub fn load_site_from_path(root: impl AsRef<Path>) -> Result<ConfigBundle, ConfigError> {
    let root = root.as_ref();

    let site = read_yaml::<SiteFile>(&root.join("site.yaml"))?;
    let interfaces = read_yaml::<InterfacesFile>(&root.join("interfaces.yaml"))?;
    let networks = load_yaml_dir::<RawNetworkFile>(&root.join("networks"))?;
    let hosts = load_yaml_dir::<RawHostFile>(&root.join("hosts"))?;
    let wifi_ssids = read_yaml::<RawSsidsFile>(&root.join("wifi/ssids.yaml"))?;
    let wifi_aps = read_yaml::<RawApsFile>(&root.join("wifi/aps.yaml"))?;
    let wifi_groups = read_yaml::<RawGroupsFile>(&root.join("wifi/groups.yaml"))?;
    let firewall_zones = read_yaml::<RawZonesFile>(&root.join("firewall/zones.yaml"))?;
    let firewall_policies = read_yaml::<RawPoliciesFile>(&root.join("firewall/policies.yaml"))?;
    let dns = read_yaml::<RawDnsFile>(&root.join("services/dns.yaml"))?;
    let dhcp = read_yaml::<RawDhcpFile>(&root.join("services/dhcp.yaml"))?;

    let nftables_path = root.join("services/nftables.yaml");
    let nftables_service = if nftables_path.exists() {
        Some(read_yaml::<RawServiceFile>(&nftables_path)?)
    } else {
        None
    };

    Ok(ConfigBundle {
        site,
        interfaces,
        networks,
        hosts,
        wifi_ssids,
        wifi_aps,
        wifi_groups,
        firewall_zones,
        firewall_policies,
        dns,
        dhcp,
        nftables_service,
    })
}

fn read_yaml<T>(path: &Path) -> Result<T, ConfigError>
where
    T: serde::de::DeserializeOwned,
{
    if !path.exists() {
        return Err(ConfigError::MissingFile(path.display().to_string()));
    }

    let contents = fs::read_to_string(path).map_err(|source| ConfigError::ReadFailed {
        path: path.to_path_buf(),
        source,
    })?;

    serde_yaml::from_str(&contents).map_err(|source| ConfigError::ParseFailed {
        path: path.to_path_buf(),
        source,
    })
}

fn load_yaml_dir<T>(path: &Path) -> Result<Vec<T>, ConfigError>
where
    T: serde::de::DeserializeOwned,
{
    if !path.exists() {
        return Err(ConfigError::MissingFile(path.display().to_string()));
    }

    let mut items = Vec::new();

    for entry in WalkDir::new(path)
        .min_depth(1)
        .max_depth(1)
        .sort_by_file_name()
        .into_iter()
        .filter_map(Result::ok)
        .filter(|entry| entry.file_type().is_file())
    {
        let file_path = entry.into_path();
        if matches!(
            file_path.extension().and_then(|ext| ext.to_str()),
            Some("yaml" | "yml")
        ) {
            items.push(read_yaml(&file_path)?);
        }
    }

    Ok(items)
}
