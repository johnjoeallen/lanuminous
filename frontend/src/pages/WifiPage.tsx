import { useEffect, useState } from "react";
import { SectionPanel } from "../components/SectionPanel";
import { SiteViewModel } from "../types/site";

interface WifiPageProps {
  site: SiteViewModel;
}

export function WifiPage({ site }: WifiPageProps) {
  const [exposeAllSsids, setExposeAllSsids] = useState(site.wifiExposeAllSsidsOnAllAps);

  useEffect(() => {
    setExposeAllSsids(site.wifiExposeAllSsidsOnAllAps);
  }, [site.wifiExposeAllSsidsOnAllAps]);

  return (
    <>
      <SectionPanel
        title="SSID intent"
        subtitle="SSID to VLAN and zone mapping from the canonical Wi-Fi model."
      >
        <div className="summary-grid">
          {site.ssids.map((ssid) => (
            <article key={ssid.name} className="summary-card">
              <span>{ssid.vlanLabel}</span>
              <strong>{ssid.name}</strong>
              <p>
                Zone: {ssid.zone} | Internal tag: {ssid.vlan}
              </p>
              <small>Groups: {ssid.groups.join(", ")}</small>
            </article>
          ))}
        </div>
      </SectionPanel>

      <SectionPanel
        title="Access point inventory"
        subtitle="Intended AP state even before vendor/controller integration exists."
        headerAction={
          <label className="toggle-field">
            <input
              type="checkbox"
              checked={exposeAllSsids}
              onChange={(event) => setExposeAllSsids(event.target.checked)}
            />
            <span>Expose all SSIDs on all APs</span>
          </label>
        }
      >
        <div className="summary-grid">
          {site.accessPoints.map((ap) => (
            <article key={ap.name} className="summary-card ap-card">
              <span>{ap.group}</span>
              <strong>{ap.name}</strong>
              <p>Mgmt: {ap.managementIp}</p>
              <div className="meta-list">
                <div className="meta-row">
                  <small className="meta-label">Uplink</small>
                  <small>{ap.uplinkPort}</small>
                </div>
                <div className="meta-row">
                  <small className="meta-label">SSIDs</small>
                  <small>
                    {(exposeAllSsids ? site.ssids.map((ssid) => ssid.name) : ap.ssids).join(", ")}
                  </small>
                </div>
              </div>
            </article>
          ))}
        </div>
      </SectionPanel>
    </>
  );
}
