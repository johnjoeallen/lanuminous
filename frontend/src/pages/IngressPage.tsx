import { SectionPanel } from "../components/SectionPanel";
import { SiteViewModel } from "../types/site";
import { humanizeScopedName } from "../utils/display";

interface IngressPageProps {
  site: SiteViewModel;
}

export function IngressPage({ site }: IngressPageProps) {
  const displayName = (value: string) => humanizeScopedName(site.name, value);
  const activePublications = site.remoteAccess.publications.filter((publication) => publication.enabled);
  const disabledPublications = site.remoteAccess.publications.filter(
    (publication) => !publication.enabled
  );
  const activeWanUpdates = site.remoteAccess.wanUpdates.filter((update) => update.enabled);

  return (
    <>
      <SectionPanel
        title="Remote access summary"
        subtitle="Ways people and devices can reach services on your network from outside."
      >
        <div className="summary-grid">
          <article className="summary-card">
            <span>Published services</span>
            <strong>{activePublications.length}</strong>
            <p>Services that are explicitly allowed remote access.</p>
          </article>
          <article className="summary-card">
            <span>WAN sync entries</span>
            <strong>{activeWanUpdates.length}</strong>
            <p>Hostnames that follow your current public IP address.</p>
          </article>
          <article className="summary-card">
            <span>Remote DNS providers</span>
            <strong>{site.remoteAccess.providers.length}</strong>
            <p>Backends available for publication and dynamic DNS updates.</p>
          </article>
        </div>
      </SectionPanel>

      <SectionPanel
        title="Published services"
        subtitle="Only services listed here are allowed to be named or reached remotely."
      >
        {site.remoteAccess.publications.length === 0 ? (
          <div className="empty-state">
            <p>No remote publication rules are configured.</p>
          </div>
        ) : (
          <div className="summary-grid">
            {site.remoteAccess.publications.map((publication) => (
              <article key={publication.service} className="summary-card">
                <span>{publication.enabled ? "Published" : "Private only"}</span>
                <strong>{publication.service}</strong>
                <p>
                  {publication.enabled && publication.externalName
                    ? publication.externalName
                    : "Not published remotely"}
                </p>
                <small>
                  Provider: {publication.provider ?? "none"} | Protocol: {publication.protocol.toUpperCase()} | Port:{" "}
                  {publication.targetPort || "n/a"}
                </small>
                <small>
                  Audience: {formatAudience(publication.audience)} | Mode:{" "}
                  {formatValue(publication.exposureMode)}
                </small>
                <small>
                  Target: {publication.targetAddress ?? "unresolved"}
                </small>
              </article>
            ))}
          </div>
        )}
      </SectionPanel>

      <SectionPanel
        title="WAN address sync"
        subtitle="Hostnames that are updated to follow your internet-facing address."
      >
        {site.remoteAccess.wanUpdates.length === 0 ? (
          <div className="empty-state">
            <p>No WAN address synchronization entries are configured.</p>
          </div>
        ) : (
          <div className="table-card">
            <table className="data-table">
              <thead>
                <tr>
                  <th>Name</th>
                  <th>Hostname</th>
                  <th>Provider</th>
                  <th>Audience</th>
                  <th>Status</th>
                </tr>
              </thead>
              <tbody>
                {site.remoteAccess.wanUpdates.map((update) => (
                  <tr key={update.name}>
                    <td>{update.name}</td>
                    <td>{update.hostname}</td>
                    <td>{update.provider}</td>
                    <td>{formatAudience(update.audience)}</td>
                    <td>{update.enabled ? "Enabled" : "Disabled"}</td>
                  </tr>
                ))}
              </tbody>
            </table>
          </div>
        )}
      </SectionPanel>

      <SectionPanel
        title="Remote DNS providers"
        subtitle="The publication backends Lanuminous can use for subdomains and dynamic DNS."
      >
        {site.remoteAccess.providers.length === 0 ? (
          <div className="empty-state">
            <p>No remote DNS providers are configured.</p>
          </div>
        ) : (
          <div className="summary-grid">
            {site.remoteAccess.providers.map((provider) => (
              <article key={provider.id} className="summary-card">
                <span>{formatProviderKind(provider.kind)}</span>
                <strong>{provider.id}</strong>
                <p>
                  {provider.credentialRef
                    ? `Uses secret reference ${provider.credentialRef}.`
                    : "No credential reference needed in the current example."}
                </p>
                <small>
                  Active services:{" "}
                  {
                    activePublications.filter((publication) => publication.provider === provider.id).length
                  }
                </small>
                <small>
                  WAN sync entries:{" "}
                  {activeWanUpdates.filter((update) => update.provider === provider.id).length}
                </small>
              </article>
            ))}
          </div>
        )}
      </SectionPanel>

      <SectionPanel
        title="Local edge rules"
        subtitle="Existing port forwarding and reverse-proxy rules that control access at your gateway."
      >
        <div className="summary-grid">
          <article className="summary-card">
            <span>Port forwards</span>
            <strong>{site.portForwards.length}</strong>
            <p>Direct inbound rules at the gateway.</p>
          </article>
          <article className="summary-card">
            <span>Reverse proxies</span>
            <strong>{site.reverseProxies.length}</strong>
            <p>Friendly web routing for services behind the gateway.</p>
          </article>
          <article className="summary-card">
            <span>Private services</span>
            <strong>{disabledPublications.length}</strong>
            <p>Configured services that remain private by policy.</p>
          </article>
        </div>
      </SectionPanel>

      {site.portForwards.length > 0 && (
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
      )}

      {site.reverseProxies.length > 0 && (
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
                <small>TLS: {formatValue(proxy.tlsMode)}</small>
              </article>
            ))}
          </div>
        </SectionPanel>
      )}
    </>
  );
}

function formatProviderKind(value: string): string {
  return value
    .replace(/_/g, " ")
    .replace(/\b\w/g, (character) => character.toUpperCase());
}

function formatAudience(value: string): string {
  return formatValue(value);
}

function formatValue(value: string): string {
  return value
    .replace(/_/g, " ")
    .replace(/\b\w/g, (character) => character.toUpperCase());
}
