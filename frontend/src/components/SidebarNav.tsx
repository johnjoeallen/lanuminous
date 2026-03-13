import { AppSection, SiteViewModel } from "../types/site";

interface SidebarNavProps {
  sections: Array<{ id: AppSection; label: string }>;
  activeSection: AppSection;
  onSelectSection: (section: AppSection) => void;
  site: SiteViewModel;
  dataSource: "api" | "mock";
}

export function SidebarNav({
  sections,
  activeSection,
  onSelectSection,
  site,
  dataSource
}: SidebarNavProps) {
  return (
    <aside className="sidebar">
      <div className="sidebar-brand">
        <p className="sidebar-kicker">Lantricate</p>
        <h1>{site.name}</h1>
        <p className="sidebar-copy">
          Canonical network intent for a managed home-lab gateway and Wi-Fi edge.
        </p>
      </div>

      <nav className="sidebar-nav" aria-label="Primary">
        {sections.map((section) => (
          <button
            key={section.id}
            type="button"
            className={section.id === activeSection ? "nav-link is-active" : "nav-link"}
            onClick={() => onSelectSection(section.id)}
          >
            <span>{section.label}</span>
          </button>
        ))}
      </nav>

      <div className="sidebar-foot">
        <div className="status-chip">
          <span className="status-dot" />
          Validation clean
        </div>
        <div className="status-chip">
          Data source: {dataSource}
        </div>
        <p className="sidebar-meta">
          Managed outputs: {site.artifacts.length} artifacts across dnsmasq, nftables,
          and Linux networking.
        </p>
      </div>
    </aside>
  );
}
