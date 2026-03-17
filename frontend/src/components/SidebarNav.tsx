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
        <p className="sidebar-kicker">Lanuminous</p>
        <p className="sidebar-copy">
          Simple, centralized management for secure networks.
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
