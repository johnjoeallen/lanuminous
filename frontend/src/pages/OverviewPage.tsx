import { SectionPanel } from "../components/SectionPanel";
import { SiteViewModel } from "../types/site";

interface OverviewPageProps {
  site: SiteViewModel;
}

export function OverviewPage({ site }: OverviewPageProps) {
  const highlights = [
    { label: "Canonical model", value: "Rust domain types shared by CLI and API" },
    { label: "Config source", value: "Folder-based YAML with normalization" },
    { label: "Current stage", value: "Foundation with renderer and deploy scaffolding" }
  ];

  return (
    <>
      <section className="hero-panel">
        <p className="eyebrow">Overview</p>
        <h2>Manage your home network from one place.</h2>
        <p className="hero-copy">
          Lanuminous helps you set up and manage your gateway, Wi-Fi, and network
          rules without editing system files by hand.
        </p>
        <div className="hero-grid">
          {highlights.map((item) => (
            <article key={item.label} className="highlight-card">
              <span>{item.label}</span>
              <strong>{item.value}</strong>
            </article>
          ))}
        </div>
      </section>

      <SectionPanel
        title="Current topology summary"
        subtitle="A compact operational read of the intended site state."
      >
        <div className="summary-grid">
          <article className="summary-card">
            <span>Gateway uplinks</span>
            <strong>WAN + trusted LAN + Wi-Fi trunk</strong>
          </article>
          <article className="summary-card">
            <span>Wi-Fi segmentation</span>
            <strong>{site.ssids.length} Wi-Fi networks with separate policy and routing</strong>
          </article>
          <article className="summary-card">
            <span>Apply posture</span>
            <strong>Render and plan today, deploy pipeline next</strong>
          </article>
        </div>
      </SectionPanel>
    </>
  );
}
