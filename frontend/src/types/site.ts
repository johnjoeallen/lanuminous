export type AppSection =
  | "overview"
  | "networks"
  | "firewall"
  | "wifi"
  | "artifacts"
  | "deployments";

export interface SiteViewModel {
  name: string;
  description: string;
  networks: NetworkCard[];
  interfaces: InterfaceCard[];
  firewallPolicies: FirewallPolicyCard[];
  ssids: SsidCard[];
  accessPoints: AccessPointCard[];
  artifacts: ArtifactCard[];
  deployments: DeploymentCard[];
}

export interface NetworkCard {
  name: string;
  cidr: string;
  zone: string;
  vlan: number | null;
  interface: string;
  purpose: string;
}

export interface InterfaceCard {
  name: string;
  role: string;
  addresses: string[];
  networkRefs: string[];
}

export interface FirewallPolicyCard {
  name: string;
  sourceZone: string;
  destinationZone: string;
  action: string;
  summary: string;
}

export interface SsidCard {
  name: string;
  vlan: number;
  zone: string;
  groups: string[];
}

export interface AccessPointCard {
  name: string;
  managementIp: string;
  group: string;
  uplinkPort: string;
  ssids: string[];
}

export interface ArtifactCard {
  logicalName: string;
  targetPath: string;
  renderer: string;
  changeState: "changed" | "unchanged" | "planned";
}

export interface DeploymentCard {
  id: string;
  timestamp: string;
  status: "planned" | "applied" | "rolled_back";
  summary: string;
}
