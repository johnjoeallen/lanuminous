use std::{
    env, fs,
    path::{Path, PathBuf},
};

use anyhow::{bail, Context, Result};
use schemars::JsonSchema;
use serde::{Deserialize, Serialize};
use walkdir::WalkDir;

use crate::util::hashing::sha256_bytes;

#[derive(Debug, Clone, Serialize, Deserialize, JsonSchema, PartialEq, Eq)]
pub struct AgentIdentity {
    pub node_name: String,
    pub operating_system: String,
    pub architecture: String,
    pub lanuminous_version: String,
}

#[derive(Debug, Clone, Serialize, Deserialize, JsonSchema, PartialEq, Eq)]
pub struct AgentCapabilities {
    pub state_root: String,
    pub staging_root: String,
    pub backup_root: String,
    pub manifest_root: String,
    pub managed_services: Vec<String>,
    pub can_apply_managed_files: bool,
    pub can_reload_services: bool,
}

#[derive(Debug, Clone, Serialize, Deserialize, JsonSchema, PartialEq, Eq)]
pub struct HostAgentDescriptor {
    pub identity: AgentIdentity,
    pub capabilities: AgentCapabilities,
}

#[derive(Debug, Clone, Serialize, Deserialize, JsonSchema, PartialEq, Eq)]
pub struct StageInspection {
    pub stage_dir: String,
    pub artifact_count: usize,
    pub artifacts: Vec<StageInspectionArtifact>,
}

#[derive(Debug, Clone, Serialize, Deserialize, JsonSchema, PartialEq, Eq)]
pub struct StageInspectionArtifact {
    pub relative_path: String,
    pub full_path: String,
    pub checksum: String,
    pub size_bytes: u64,
}

#[derive(Debug, Default)]
pub struct HostAgentService;

impl HostAgentService {
    pub fn describe(&self, state_root: impl AsRef<Path>) -> HostAgentDescriptor {
        let state_root = state_root.as_ref();

        HostAgentDescriptor {
            identity: AgentIdentity {
                node_name: detect_node_name(),
                operating_system: env::consts::OS.to_string(),
                architecture: env::consts::ARCH.to_string(),
                lanuminous_version: env!("CARGO_PKG_VERSION").to_string(),
            },
            capabilities: AgentCapabilities {
                state_root: state_root.display().to_string(),
                staging_root: state_root.join("staging").display().to_string(),
                backup_root: state_root.join("backups").display().to_string(),
                manifest_root: state_root.join("manifests").display().to_string(),
                managed_services: vec![
                    "dnsmasq".to_string(),
                    "nftables".to_string(),
                    "systemd-networkd".to_string(),
                    "apache2".to_string(),
                    "nginx".to_string(),
                    "caddy".to_string(),
                    "traefik".to_string(),
                    "haproxy".to_string(),
                ],
                can_apply_managed_files: true,
                can_reload_services: true,
            },
        }
    }

    pub fn inspect_stage_dir(&self, stage_dir: impl AsRef<Path>) -> Result<StageInspection> {
        let stage_dir = stage_dir.as_ref();
        if !stage_dir.exists() {
            bail!("stage directory does not exist: {}", stage_dir.display());
        }
        if !stage_dir.is_dir() {
            bail!("stage path is not a directory: {}", stage_dir.display());
        }

        let mut artifacts = Vec::new();
        for entry in WalkDir::new(stage_dir)
            .min_depth(1)
            .into_iter()
            .filter_map(Result::ok)
            .filter(|entry| entry.file_type().is_file())
        {
            let full_path = entry.path().to_path_buf();
            let relative_path = full_path
                .strip_prefix(stage_dir)
                .unwrap_or(&full_path)
                .display()
                .to_string();
            let bytes = fs::read(&full_path).with_context(|| {
                format!("failed to read staged artifact {}", full_path.display())
            })?;
            artifacts.push(StageInspectionArtifact {
                relative_path,
                full_path: full_path.display().to_string(),
                checksum: sha256_bytes(&bytes),
                size_bytes: bytes.len() as u64,
            });
        }

        artifacts.sort_by(|left, right| left.relative_path.cmp(&right.relative_path));

        Ok(StageInspection {
            stage_dir: stage_dir.display().to_string(),
            artifact_count: artifacts.len(),
            artifacts,
        })
    }
}

fn detect_node_name() -> String {
    if let Ok(hostname) = env::var("HOSTNAME") {
        let trimmed = hostname.trim();
        if !trimmed.is_empty() {
            return trimmed.to_string();
        }
    }

    read_hostname_file("/etc/hostname")
        .or_else(|| read_hostname_file("/proc/sys/kernel/hostname"))
        .unwrap_or_else(|| "unknown-host".to_string())
}

fn read_hostname_file(path: impl Into<PathBuf>) -> Option<String> {
    let path = path.into();
    fs::read_to_string(path).ok().and_then(|contents| {
        let trimmed = contents.trim();
        (!trimmed.is_empty()).then(|| trimmed.to_string())
    })
}
