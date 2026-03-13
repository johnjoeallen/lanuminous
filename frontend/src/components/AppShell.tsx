import { PropsWithChildren } from "react";
import { AppSection, SiteViewModel } from "../types/site";
import { SidebarNav } from "./SidebarNav";
import { TopBar } from "./TopBar";

interface AppShellProps extends PropsWithChildren {
  sections: Array<{ id: AppSection; label: string }>;
  activeSection: AppSection;
  onSelectSection: (section: AppSection) => void;
  site: SiteViewModel;
  dataSource: "api" | "mock";
  loading: boolean;
  error: string | null;
}

export function AppShell({
  children,
  sections,
  activeSection,
  onSelectSection,
  site,
  dataSource,
  loading,
  error
}: AppShellProps) {
  return (
    <main className="app-shell">
      <div className="app-backdrop" />
      <div className="app-layout">
        <SidebarNav
          sections={sections}
          activeSection={activeSection}
          onSelectSection={onSelectSection}
          site={site}
          dataSource={dataSource}
        />
        <div className="app-main">
          <TopBar site={site} dataSource={dataSource} loading={loading} error={error} />
          <div className="page-stack">{children}</div>
        </div>
      </div>
    </main>
  );
}
