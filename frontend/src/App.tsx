import { useState } from "react";
import { generateStaging } from "./api/client";
import { AppShell } from "./components/AppShell";
import { useSiteData } from "./hooks/useSiteData";
import { ArtifactsPage } from "./pages/ArtifactsPage";
import { DeploymentsPage } from "./pages/DeploymentsPage";
import { FirewallPage } from "./pages/FirewallPage";
import { IngressPage } from "./pages/IngressPage";
import { NetworksPage } from "./pages/NetworksPage";
import { OverviewPage } from "./pages/OverviewPage";
import { WifiPage } from "./pages/WifiPage";
import { AppSection, StagingResult } from "./types/site";

const sections: Array<{ id: AppSection; label: string }> = [
  { id: "overview", label: "Overview" },
  { id: "networks", label: "Networks" },
  { id: "firewall", label: "Firewall" },
  { id: "ingress", label: "Remote Access" },
  { id: "wifi", label: "Wi-Fi" },
  { id: "artifacts", label: "Artifacts" },
  { id: "deployments", label: "Deployments" }
];

export default function App() {
  const [activeSection, setActiveSection] = useState<AppSection>("overview");
  const { site, source, loading, error } = useSiteData();
  const [staging, setStaging] = useState<{
    loading: boolean;
    error: string | null;
    result: StagingResult | null;
  }>({
    loading: false,
    error: null,
    result: null
  });

  async function handleGenerateStaging() {
    setStaging({
      loading: true,
      error: null,
      result: null
    });

    try {
      const result = (await generateStaging()) as StagingResult;
      setStaging({
        loading: false,
        error: null,
        result
      });
    } catch (stageError) {
      setStaging({
        loading: false,
        error: stageError instanceof Error ? stageError.message : "Failed to generate staging files",
        result: null
      });
    }
  }

  return (
    <AppShell
      sections={sections}
      activeSection={activeSection}
      onSelectSection={setActiveSection}
      site={site}
      dataSource={source}
      loading={loading}
      error={error}
      staging={staging}
      onGenerateStaging={handleGenerateStaging}
    >
      {activeSection === "overview" && <OverviewPage site={site} />}
      {activeSection === "networks" && <NetworksPage site={site} />}
      {activeSection === "firewall" && <FirewallPage site={site} />}
      {activeSection === "ingress" && <IngressPage site={site} />}
      {activeSection === "wifi" && <WifiPage site={site} />}
      {activeSection === "artifacts" && <ArtifactsPage site={site} staging={staging} />}
      {activeSection === "deployments" && <DeploymentsPage site={site} />}
    </AppShell>
  );
}
