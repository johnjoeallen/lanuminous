import { SiteViewModel, StagingResult } from "../types/site";

interface TopBarProps {
  site: SiteViewModel;
  dataSource: "api" | "mock";
  loading: boolean;
  error: string | null;
  staging: {
    loading: boolean;
    error: string | null;
    result: StagingResult | null;
  };
  onGenerateStaging: () => void;
}

export function TopBar({
  site,
  dataSource,
  loading,
  error,
  staging,
  onGenerateStaging
}: TopBarProps) {
  return (
    <header className="topbar">
      <div>
        <p className="eyebrow">Site status</p>
        <h2>{`Welcome to ${site.name}`}</h2>
        <p className="topbar-copy">
          {loading
            ? "Loading site data from the backend."
            : error
              ? `Backend unavailable, using local mock data. ${error}`
              : `Connected to the ${dataSource} site model endpoint.`}
        </p>
        {staging.result ? (
          <p className="topbar-copy">
            Staged {staging.result.artifactCount} artifacts into {staging.result.stageDir}
          </p>
        ) : null}
        {staging.error ? <p className="topbar-copy">{staging.error}</p> : null}
      </div>
      <div className="topbar-tools">
        <button
          type="button"
          className="primary-button"
          onClick={onGenerateStaging}
          disabled={staging.loading}
        >
          {staging.loading ? "Generating staging..." : "Generate Staging Files"}
        </button>
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
      </div>
    </header>
  );
}
