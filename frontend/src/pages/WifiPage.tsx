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
        subtitle="SSID and zone mapping from the canonical Wi-Fi model."
      >
        <h4 className="section-subheading">Virtual Lans (VLANS)</h4>
        <div className="summary-grid">
          {site.ssids.map((ssid) => (
            <article key={ssid.name} className="summary-card">
              <span>{ssid.vlanLabel}</span>
              <strong>{ssid.name}</strong>
              <p>Zone: {ssid.zone}</p>
              <small>Groups: {ssid.groups.join(", ")}</small>
            </article>
          ))}
        </div>
      </SectionPanel>

      <SectionPanel
        title="Access points (APs)"
        subtitle="Intended AP state even before vendor/controller integration exists."
        headerAction={
          <div className="toggle-with-help">
            <label className="toggle-field">
              <input
                type="checkbox"
                checked={exposeAllSsids}
                onChange={(event) => setExposeAllSsids(event.target.checked)}
              />
              <span>Unify SSIDs</span>
            </label>
            <div className="help-popover">
              <button
                type="button"
                className="help-button"
                aria-label="Explain Unify SSIDs"
              >
                ?
              </button>
              <div className="help-popover-card" role="note">
                Show the same Wi-Fi networks on every access point instead of assigning
                different networks to different rooms or devices.
              </div>
            </div>
          </div>
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
