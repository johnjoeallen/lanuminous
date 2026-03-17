import { SectionPanel } from "../components/SectionPanel";
import { SiteViewModel } from "../types/site";
import { humanizeScopedName } from "../utils/display";

interface NetworksPageProps {
  site: SiteViewModel;
}

export function NetworksPage({ site }: NetworksPageProps) {
  const formatInterfaceRole = (role: string) => role.replaceAll("_", " ").toUpperCase();
  const displayName = (value: string) => humanizeScopedName(site.name, value);
  const hasWanInterface = site.interfaces.some((iface) => iface.role === "wan");
  const hasInternalNetworks = site.interfaces.some(
    (iface) => iface.role !== "wan" && iface.networkRefs.length > 0
  );
  const logicalNicNames = site.interfaces.map((iface) => iface.logicalName).join(", ");

  return (
    <>
      <SectionPanel
        title="Gateway baseline"
        subtitle="General network settings implied by the current gateway intent."
      >
        <div className="summary-grid">
          <article className="summary-card">
            <span>IP forwarding</span>
            <strong>{hasWanInterface && hasInternalNetworks ? "Enabled" : "Not set"}</strong>
            <p>
              {hasWanInterface && hasInternalNetworks
                ? "Required for routed traffic between WAN and managed internal networks."
                : "This view has not inferred routed gateway behavior yet."}
            </p>
          </article>
          <article className="summary-card">
            <span>Logical NICs</span>
            <strong>{logicalNicNames}</strong>
            <p>Stable interface names used for routing, policy, and generated config.</p>
          </article>
        </div>
      </SectionPanel>

      <SectionPanel
        title="Network inventory"
        subtitle="The minimum network definitions needed to route traffic and explain what each network is for."
      >
        <div className="table-card">
          <table className="data-table">
            <thead>
              <tr>
                <th>Name</th>
                <th>Description</th>
                <th>CIDR</th>
                <th>VLAN</th>
                <th>Interface</th>
              </tr>
            </thead>
            <tbody>
              {site.networks.map((network) => (
                <tr key={network.name}>
                  <td>{displayName(network.name)}</td>
                  <td>{network.description}</td>
                  <td>{network.cidr}</td>
                  <td>{network.vlan ? displayName(network.name) : "n/a"}</td>
                  <td>{displayName(network.interface)}</td>
                </tr>
              ))}
            </tbody>
          </table>
        </div>
      </SectionPanel>

      <SectionPanel
        title="Physical network segments"
        subtitle="The gateway's physical interfaces and the logical names used to manage them."
      >
        <div className="summary-grid">
          {site.interfaces.map((iface) => (
            <article key={iface.name} className="summary-card">
              <span>{formatInterfaceRole(iface.role)}</span>
              <strong>{formatInterfaceRole(iface.role)}</strong>
              <p>Logical: {iface.logicalName}</p>
              <p>Physical: {iface.name}</p>
              <small>Addresses: {iface.addresses.join(", ")}</small>
              <small>
                Networks: {iface.networkRefs.length ? iface.networkRefs.map(displayName).join(", ") : "none"}
              </small>
            </article>
          ))}
        </div>
      </SectionPanel>
    </>
  );
}
