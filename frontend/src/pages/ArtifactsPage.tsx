import { useMemo, useState } from "react";
import { SectionPanel } from "../components/SectionPanel";
import { SiteViewModel, StagingResult } from "../types/site";

interface ArtifactsPageProps {
  site: SiteViewModel;
  staging: {
    loading: boolean;
    error: string | null;
    result: StagingResult | null;
  };
}

export function ArtifactsPage({ site, staging }: ArtifactsPageProps) {
  const stagedArtifacts = staging.result?.artifacts ?? [];
  const stagedArtifactByTargetPath = new Map(
    stagedArtifacts.map((artifact) => [artifact.targetPath, artifact])
  );
  const [selectedLogicalName, setSelectedLogicalName] = useState<string | null>(null);

  const selectedArtifact = useMemo(() => {
    if (!site.artifacts.length) {
      return null;
    }

    return (
      site.artifacts.find((artifact) => artifact.logicalName === selectedLogicalName) ??
      site.artifacts[0]
    );
  }, [selectedLogicalName, site.artifacts]);

  return (
    <SectionPanel
      title="Managed artifacts"
      subtitle={
        staging.result
          ? `${staging.result.artifactCount} files are staged in ${staging.result.stageDir}. Click a managed target to inspect its staged contents.`
          : "Click a managed target to inspect it. Generate staging files to view the exact rendered output."
      }
    >
      <div className="artifact-explorer">
        <div className="table-card">
          <table className="data-table">
            <thead>
              <tr>
                <th>Logical name</th>
                <th>Renderer</th>
                <th>Target path</th>
                <th>State</th>
              </tr>
            </thead>
            <tbody>
              {site.artifacts.map((artifact) => {
                const state =
                  artifact.changeState === "synced"
                    ? "synced"
                    : stagedArtifactByTargetPath.has(artifact.targetPath)
                      ? "staged"
                      : "planned";

                return (
                  <tr
                    key={artifact.logicalName}
                    className={artifact.logicalName === selectedArtifact?.logicalName ? "artifact-row is-selected" : "artifact-row"}
                    onClick={() => setSelectedLogicalName(artifact.logicalName)}
                  >
                    <td className="artifact-name-cell">
                      <button type="button" className="artifact-link">
                        {artifact.logicalName}
                      </button>
                    </td>
                    <td>{artifact.renderer}</td>
                    <td>{artifact.targetPath}</td>
                    <td>
                      <span className={`state-pill is-${state}`}>{state}</span>
                    </td>
                  </tr>
                );
              })}
            </tbody>
          </table>
        </div>

        {selectedArtifact ? (
          <div className="artifact-preview-card">
            <div className="artifact-preview-meta">
              <strong>{selectedArtifact.logicalName}</strong>
              <span>{selectedArtifact.targetPath}</span>
              <span>Renderer: {selectedArtifact.renderer}</span>
            </div>
            {stagedArtifactByTargetPath.has(selectedArtifact.targetPath) ? (
              <pre className="artifact-code">
                <code>{stagedArtifactByTargetPath.get(selectedArtifact.targetPath)?.contents}</code>
              </pre>
            ) : (
              <div className="empty-state artifact-empty-state">
                <strong>No staged file for this target.</strong>
                <p>
                  This managed artifact is currently planned only. Generate staging files to
                  inspect the rendered contents here.
                </p>
              </div>
            )}
          </div>
        ) : (
          <div className="empty-state">
            <strong>No managed artifacts are available.</strong>
          </div>
        )}
      </div>
    </SectionPanel>
  );
}
