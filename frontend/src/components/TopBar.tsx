import { SiteViewModel } from "../types/site";

interface TopBarProps {
  site: SiteViewModel;
  dataSource: "api" | "mock";
  loading: boolean;
  error: string | null;
}

export function TopBar({ site, dataSource, loading, error }: TopBarProps) {
  return (
    <header className="topbar">
      <div>
        <p className="eyebrow">Site status</p>
        <h2>{site.description}</h2>
        <p className="topbar-copy">
          {loading
            ? "Loading site data from the backend."
            : error
              ? `Backend unavailable, using local mock data. ${error}`
              : `Connected to the ${dataSource} site model endpoint.`}
        </p>
      </div>
      <div className="topbar-metrics">
        <article className="metric-pill">
          <span>Networks</span>
          <strong>{site.networks.length}</strong>
        </article>
        <article className="metric-pill">
          <span>Policies</span>
          <strong>{site.firewallPolicies.length}</strong>
        </article>
        <article className="metric-pill">
          <span>Access points</span>
          <strong>{site.accessPoints.length}</strong>
        </article>
      </div>
    </header>
  );
}
