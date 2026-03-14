import { SectionPanel } from "../components/SectionPanel";
import { SiteViewModel } from "../types/site";

interface FirewallPageProps {
  site: SiteViewModel;
}

export function FirewallPage({ site }: FirewallPageProps) {
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
                <span>{policy.sourceZone}</span>
                <strong>{policy.action}</strong>
                <span>{policy.destinationZone}</span>
              </div>
              <h4>{policy.name}</h4>
              <p>{policy.summary}</p>
            </article>
          ))}
        </div>
      </SectionPanel>

      <SectionPanel
        title="NAT and ingress edge"
        subtitle="Port forwards modeled as canonical ingress intent, rendered later into nftables."
      >
        <div className="summary-grid">
          {site.portForwards.map((rule) => (
            <article key={rule.name} className="summary-card">
              <span>{rule.sourceZone}</span>
              <strong>
                {rule.protocol}:{rule.externalPort} {"->"} {rule.destinationHost}:
                {rule.destinationPort}
              </strong>
              <p>{rule.summary}</p>
            </article>
          ))}
        </div>
      </SectionPanel>
    </>
  );
}
