use crate::{
    deploy::StagedArtifact,
    domain::{ApplyPlan, ManagedPath, RenderedArtifact},
};

#[derive(Debug, Default)]
pub struct DeploymentPlanner;

impl DeploymentPlanner {
    pub fn plan_stage1(&self, artifacts: &[RenderedArtifact]) -> ApplyPlan {
        ApplyPlan {
            artifacts: artifacts.to_vec(),
            changed_paths: artifacts
                .iter()
                .map(|artifact| ManagedPath {
                    logical_name: artifact.logical_name.clone(),
                    path: artifact.target_path.clone(),
                    service: Some(artifact.renderer.clone()),
                })
                .collect(),
        }
    }

    pub fn stage_records(
        &self,
        stage_dir: &str,
        artifacts: &[RenderedArtifact],
    ) -> Vec<StagedArtifact> {
        artifacts
            .iter()
            .map(|artifact| StagedArtifact {
                logical_name: artifact.logical_name.clone(),
                stage_path: format!("{stage_dir}/{}", artifact.logical_name),
                target_path: artifact.target_path.clone(),
                checksum: artifact.checksum.clone(),
            })
            .collect()
    }
}
