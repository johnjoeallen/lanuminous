import { SectionPanel } from "../components/SectionPanel";
import { SiteViewModel } from "../types/site";

interface IngressPageProps {
  site: SiteViewModel;
}

export function IngressPage({ site }: IngressPageProps) {
  return (
    <>
      <SectionPanel
        title="Ingress summary"
        subtitle="Everything that accepts traffic from outside and routes it to internal services."
      >
        <div className="summary-grid">
          <article className="summary-card">
            <span>Port forwards</span>
            <strong>{site.portForwards.length}</strong>
          </article>
          <article className="summary-card">
            <span>Reverse proxies</span>
            <strong>{site.reverseProxies.length}</strong>
          </article>
          <article className="summary-card">
            <span>Proxy provider</span>
            <strong>{site.reverseProxyProvider}</strong>
          </article>
        </div>
      </SectionPanel>

      <SectionPanel
        title="Port forwarding"
        subtitle="External-to-internal service exposure modeled independently from nftables syntax."
      >
        <div className="table-card">
          <table className="data-table">
            <thead>
              <tr>
                <th>Name</th>
                <th>Source zone</th>
                <th>Protocol</th>
                <th>External</th>
                <th>Destination</th>
                <th>Summary</th>
              </tr>
            </thead>
            <tbody>
              {site.portForwards.map((rule) => (
                <tr key={rule.name}>
                  <td>{rule.name}</td>
                  <td>{rule.sourceZone}</td>
                  <td>{rule.protocol}</td>
                  <td>{rule.externalPort}</td>
                  <td>
                    {rule.destinationHost}:{rule.destinationPort}
                  </td>
                  <td>{rule.summary}</td>
                </tr>
              ))}
            </tbody>
          </table>
        </div>
      </SectionPanel>

      <SectionPanel
        title={`Reverse proxies via ${site.reverseProxyProvider}`}
        subtitle="Provider-selectable reverse proxy intent rendered into managed proxy configuration."
      >
        <div className="summary-grid">
          {site.reverseProxies.map((proxy) => (
            <article key={proxy.name} className="summary-card">
              <span>
                {proxy.provider} on {proxy.listenPort}
              </span>
              <strong>{proxy.name}</strong>
              <p>Hosts: {proxy.serverNames.join(", ")}</p>
              <small>
                Backend: {proxy.backendScheme}://{proxy.backendHost}:{proxy.backendPort}
              </small>
              <small>TLS: {proxy.tlsMode}</small>
            </article>
          ))}
        </div>
      </SectionPanel>
    </>
  );
}
