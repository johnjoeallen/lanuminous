import { useState } from "react";
import { AppShell } from "./components/AppShell";
import { useSiteData } from "./hooks/useSiteData";
import { ArtifactsPage } from "./pages/ArtifactsPage";
import { DeploymentsPage } from "./pages/DeploymentsPage";
import { FirewallPage } from "./pages/FirewallPage";
import { IngressPage } from "./pages/IngressPage";
import { NetworksPage } from "./pages/NetworksPage";
import { OverviewPage } from "./pages/OverviewPage";
import { WifiPage } from "./pages/WifiPage";
import { AppSection } from "./types/site";

const sections: Array<{ id: AppSection; label: string }> = [
  { id: "overview", label: "Overview" },
  { id: "networks", label: "Networks" },
  { id: "firewall", label: "Firewall" },
  { id: "ingress", label: "Ingress" },
  { id: "wifi", label: "Wi-Fi" },
  { id: "artifacts", label: "Artifacts" },
  { id: "deployments", label: "Deployments" }
];

export default function App() {
  const [activeSection, setActiveSection] = useState<AppSection>("overview");
  const { site, source, loading, error } = useSiteData();

  return (
    <AppShell
      sections={sections}
      activeSection={activeSection}
      onSelectSection={setActiveSection}
      site={site}
      dataSource={source}
      loading={loading}
      error={error}
    >
      {activeSection === "overview" && <OverviewPage site={site} />}
      {activeSection === "networks" && <NetworksPage site={site} />}
      {activeSection === "firewall" && <FirewallPage site={site} />}
      {activeSection === "ingress" && <IngressPage site={site} />}
      {activeSection === "wifi" && <WifiPage site={site} />}
      {activeSection === "artifacts" && <ArtifactsPage site={site} />}
      {activeSection === "deployments" && <DeploymentsPage site={site} />}
    </AppShell>
  );
}
