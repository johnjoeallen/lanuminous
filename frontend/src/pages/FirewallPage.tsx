import { SectionPanel } from "../components/SectionPanel";
import { SiteViewModel } from "../types/site";
import { humanizeScopedName } from "../utils/display";

interface FirewallPageProps {
  site: SiteViewModel;
}

export function FirewallPage({ site }: FirewallPageProps) {
  const displayName = (value: string) => humanizeScopedName(site.name, value);

  return (
    <>
      <SectionPanel
        title="Policy graph"
        subtitle="Intent-level firewall relationships before nftables rendering."
      >
        <div className="policy-list">
          {site.firewallPolicies.map((policy) => (
            <article key={policy.name} className="policy-card">
              <div className="policy-route">
                <span>{displayName(policy.sourceZone)}</span>
                <strong>{policy.action}</strong>
                <span>{displayName(policy.destinationZone)}</span>
              </div>
              <h4>{policy.name}</h4>
              <p>{policy.summary}</p>
            </article>
          ))}
        </div>
      </SectionPanel>
    </>
  );
}
