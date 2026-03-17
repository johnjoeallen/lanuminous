use std::{
    fs,
    path::{Path, PathBuf},
};

use anyhow::Result;
use chrono::{DateTime, Utc};
use uuid::Uuid;

use crate::{
    config::{load_site_from_path, normalize_bundle},
    domain::{RenderedArtifact, SiteConfig},
    render::render_all,
    validate::{run_validation, ValidationReport},
};

#[derive(Debug, Default)]
pub struct SiteService;

#[derive(Debug, Clone)]
pub struct StagingResult {
    pub stage_dir: PathBuf,
    pub generated_at: DateTime<Utc>,
    pub artifacts: Vec<StagedArtifactResult>,
}

#[derive(Debug, Clone)]
pub struct StagedArtifactResult {
    pub logical_name: String,
    pub stage_path: String,
    pub target_path: String,
    pub checksum: String,
    pub contents: String,
}

impl SiteService {
    pub fn load_site(&self, config_root: impl AsRef<Path>) -> Result<SiteConfig> {
        let bundle = load_site_from_path(config_root)?;
        Ok(normalize_bundle(bundle))
    }

    pub fn validate_site(&self, site: &SiteConfig) -> ValidationReport {
        run_validation(site)
    }

    pub fn render_site(&self, site: &SiteConfig) -> Result<Vec<RenderedArtifact>> {
        render_all(site)
    }

    pub fn stage_site(
        &self,
        site: &SiteConfig,
        stage_root: impl AsRef<Path>,
    ) -> Result<StagingResult> {
        let artifacts = self.render_site(site)?;
        let generated_at = Utc::now();
        let stage_dir = stage_root
            .as_ref()
            .join(format!("{}-{}", generated_at.format("%Y%m%dT%H%M%SZ"), Uuid::new_v4()));
        fs::create_dir_all(&stage_dir)?;

        let mut staged_artifacts = Vec::new();
        for artifact in &artifacts {
            let stage_path = stage_dir.join(stage_filename(artifact));
            fs::write(&stage_path, &artifact.contents)?;
            staged_artifacts.push(StagedArtifactResult {
                logical_name: artifact.logical_name.clone(),
                stage_path: stage_path.display().to_string(),
                target_path: artifact.target_path.clone(),
                checksum: artifact.checksum.clone(),
                contents: artifact.contents.clone(),
            });
        }

        Ok(StagingResult {
            stage_dir,
            generated_at,
            artifacts: staged_artifacts,
        })
    }
}

fn stage_filename(artifact: &RenderedArtifact) -> String {
    let extension = Path::new(&artifact.target_path)
        .extension()
        .and_then(|ext| ext.to_str())
        .filter(|ext| !ext.is_empty());

    match extension {
        Some(extension) => format!("{}.{}", artifact.logical_name, extension),
        None => artifact.logical_name.clone(),
    }
}
