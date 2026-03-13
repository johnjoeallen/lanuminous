import { SectionPanel } from "../components/SectionPanel";
import { SiteViewModel } from "../types/site";

interface DeploymentsPageProps {
  site: SiteViewModel;
}

export function DeploymentsPage({ site }: DeploymentsPageProps) {
  return (
    <SectionPanel
      title="Deployment history"
      subtitle="Manifest and rollback surfaces that Stage 2 should wire to backend state."
    >
      <div className="timeline">
        {site.deployments.map((deployment) => (
          <article key={deployment.id} className="timeline-card">
            <span className="timeline-stamp">{deployment.timestamp}</span>
            <strong>{deployment.id}</strong>
            <p>{deployment.summary}</p>
            <div className="status-chip">{deployment.status}</div>
          </article>
        ))}
      </div>
    </SectionPanel>
  );
}

