use std::path::Path;

use anyhow::Result;

use crate::{
    config::{load_site_from_path, normalize_bundle},
    domain::{RenderedArtifact, SiteConfig},
    render::render_all,
    validate::{run_validation, ValidationReport},
};

#[derive(Debug, Default)]
pub struct SiteService;

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
}
