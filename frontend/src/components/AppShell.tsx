import { PropsWithChildren } from "react";
import { AppSection, SiteViewModel, StagingResult } from "../types/site";
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
  staging: {
    loading: boolean;
    error: string | null;
    result: StagingResult | null;
  };
  onGenerateStaging: () => void;
}

export function AppShell({
  children,
  sections,
  activeSection,
  onSelectSection,
  site,
  dataSource,
  loading,
  error,
  staging,
  onGenerateStaging
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
          <TopBar
            site={site}
            dataSource={dataSource}
            loading={loading}
            error={error}
            staging={staging}
            onGenerateStaging={onGenerateStaging}
          />
          <div className="page-stack">{children}</div>
        </div>
      </div>
    </main>
  );
}
