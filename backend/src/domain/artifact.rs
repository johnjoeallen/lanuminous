use chrono::{DateTime, Utc};
use schemars::JsonSchema;
use serde::{Deserialize, Serialize};
use uuid::Uuid;

use crate::domain::ManagedPath;

#[derive(Debug, Clone, Serialize, Deserialize, JsonSchema, PartialEq, Eq)]
pub struct RenderedArtifact {
    pub renderer: String,
    pub logical_name: String,
    pub target_path: String,
    pub contents: String,
    pub checksum: String,
}

#[derive(Debug, Clone, Serialize, Deserialize, JsonSchema, PartialEq, Eq, Default)]
pub struct ApplyPlan {
    pub artifacts: Vec<RenderedArtifact>,
    pub changed_paths: Vec<ManagedPath>,
}

#[derive(Debug, Clone, Serialize, Deserialize, JsonSchema, PartialEq, Eq)]
pub struct DeploymentManifest {
    pub deployment_id: Uuid,
    pub created_at: DateTime<Utc>,
    pub artifacts: Vec<ArtifactManifestEntry>,
    pub services_reloaded: Vec<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize, JsonSchema, PartialEq, Eq)]
pub struct ArtifactManifestEntry {
    pub logical_name: String,
    pub target_path: String,
    pub checksum: String,
}

#[derive(Debug, Clone, Serialize, Deserialize, JsonSchema, PartialEq, Eq)]
pub struct BackupManifest {
    pub deployment_id: Uuid,
    pub created_at: DateTime<Utc>,
    pub backups: Vec<BackupEntry>,
}

#[derive(Debug, Clone, Serialize, Deserialize, JsonSchema, PartialEq, Eq)]
pub struct BackupEntry {
    pub original_path: String,
    pub backup_path: String,
    pub checksum: String,
}
