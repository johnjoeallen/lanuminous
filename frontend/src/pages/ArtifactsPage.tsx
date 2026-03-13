import { SectionPanel } from "../components/SectionPanel";
import { SiteViewModel } from "../types/site";

interface ArtifactsPageProps {
  site: SiteViewModel;
}

export function ArtifactsPage({ site }: ArtifactsPageProps) {
  return (
    <SectionPanel
      title="Managed artifacts"
      subtitle="Renderer outputs that would be staged before any future apply step."
    >
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
            {site.artifacts.map((artifact) => (
              <tr key={artifact.logicalName}>
                <td>{artifact.logicalName}</td>
                <td>{artifact.renderer}</td>
                <td>{artifact.targetPath}</td>
                <td>
                  <span className="state-pill">{artifact.changeState}</span>
                </td>
              </tr>
            ))}
          </tbody>
        </table>
      </div>
    </SectionPanel>
  );
}

