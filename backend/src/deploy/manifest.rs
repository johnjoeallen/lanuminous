use chrono::{DateTime, Utc};
use schemars::JsonSchema;
use serde::{Deserialize, Serialize};
use uuid::Uuid;

#[derive(Debug, Clone, Serialize, Deserialize, JsonSchema, PartialEq, Eq)]
pub struct StagedArtifact {
    pub logical_name: String,
    pub stage_path: String,
    pub target_path: String,
    pub checksum: String,
}

#[derive(Debug, Clone, Serialize, Deserialize, JsonSchema, PartialEq, Eq)]
pub struct DeploymentManifestRecord {
    pub deployment_id: Uuid,
    pub created_at: DateTime<Utc>,
    pub artifacts: Vec<StagedArtifact>,
    pub services_reloaded: Vec<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize, JsonSchema, PartialEq, Eq)]
pub struct BackupManifestRecord {
    pub deployment_id: Uuid,
    pub created_at: DateTime<Utc>,
    pub backup_paths: Vec<BackupPathRecord>,
}

#[derive(Debug, Clone, Serialize, Deserialize, JsonSchema, PartialEq, Eq)]
pub struct BackupPathRecord {
    pub original_path: String,
    pub backup_path: String,
    pub checksum: String,
}
