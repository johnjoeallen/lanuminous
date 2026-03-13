import { SectionPanel } from "../components/SectionPanel";
import { SiteViewModel } from "../types/site";

interface WifiPageProps {
  site: SiteViewModel;
}

export function WifiPage({ site }: WifiPageProps) {
  return (
    <>
      <SectionPanel
        title="SSID intent"
        subtitle="SSID to VLAN and zone mapping from the canonical Wi-Fi model."
      >
        <div className="summary-grid">
          {site.ssids.map((ssid) => (
            <article key={ssid.name} className="summary-card">
              <span>VLAN {ssid.vlan}</span>
              <strong>{ssid.name}</strong>
              <p>Zone: {ssid.zone}</p>
              <small>Groups: {ssid.groups.join(", ")}</small>
            </article>
          ))}
        </div>
      </SectionPanel>

      <SectionPanel
        title="Access point inventory"
        subtitle="Intended AP state even before vendor/controller integration exists."
      >
        <div className="summary-grid">
          {site.accessPoints.map((ap) => (
            <article key={ap.name} className="summary-card">
              <span>{ap.group}</span>
              <strong>{ap.name}</strong>
              <p>Mgmt: {ap.managementIp}</p>
              <small>Uplink: {ap.uplinkPort}</small>
              <small>SSIDs: {ap.ssids.join(", ")}</small>
            </article>
          ))}
        </div>
      </SectionPanel>
    </>
  );
}

