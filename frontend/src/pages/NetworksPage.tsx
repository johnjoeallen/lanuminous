import { SectionPanel } from "../components/SectionPanel";
import { SiteViewModel } from "../types/site";

interface NetworksPageProps {
  site: SiteViewModel;
}

export function NetworksPage({ site }: NetworksPageProps) {
  return (
    <>
      <SectionPanel
        title="Network inventory"
        subtitle="Canonical networks, VLAN assignments, and interface bindings."
      >
        <div className="table-card">
          <table className="data-table">
            <thead>
              <tr>
                <th>Name</th>
                <th>CIDR</th>
                <th>Zone</th>
                <th>VLAN</th>
                <th>Interface</th>
                <th>Purpose</th>
              </tr>
            </thead>
            <tbody>
              {site.networks.map((network) => (
                <tr key={network.name}>
                  <td>{network.name}</td>
                  <td>{network.cidr}</td>
                  <td>{network.zone}</td>
                  <td>
                    {network.vlanLabel
                      ? `${network.vlanLabel} (${network.vlan})`
                      : "native"}
                  </td>
                  <td>{network.interface}</td>
                  <td>{network.purpose}</td>
                </tr>
              ))}
            </tbody>
          </table>
        </div>
      </SectionPanel>

      <SectionPanel
        title="Gateway interfaces"
        subtitle="Physical and logical entry points associated with network intent."
      >
        <div className="summary-grid">
          {site.interfaces.map((iface) => (
            <article key={iface.name} className="summary-card">
              <span>{iface.role}</span>
              <strong>{iface.name}</strong>
              <p>{iface.addresses.join(", ")}</p>
              <small>Networks: {iface.networkRefs.length ? iface.networkRefs.join(", ") : "none"}</small>
            </article>
          ))}
        </div>
      </SectionPanel>
    </>
  );
}
