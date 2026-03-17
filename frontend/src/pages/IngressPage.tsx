import { SectionPanel } from "../components/SectionPanel";
import { SiteViewModel } from "../types/site";
import { humanizeScopedName } from "../utils/display";

interface IngressPageProps {
  site: SiteViewModel;
}

export function IngressPage({ site }: IngressPageProps) {
  const displayName = (value: string) => humanizeScopedName(site.name, value);

  return (
    <>
      <SectionPanel
        title="Remote access summary"
        subtitle="Ways people and devices can reach services on your network from outside."
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
        subtitle="Direct access from outside your network to a service inside it."
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
                  <td>{displayName(rule.sourceZone)}</td>
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
        title={`Web access via ${site.reverseProxyProvider}`}
        subtitle="Friendly web access that routes incoming requests to services inside your network."
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
