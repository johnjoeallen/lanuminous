export type AppSection =
  | "overview"
  | "networks"
  | "firewall"
  | "ingress"
  | "wifi"
  | "artifacts"
  | "deployments";

export interface SiteViewModel {
  name: string;
  description: string;
  reverseProxyProvider: string;
  wifiExposeAllSsidsOnAllAps: boolean;
  networks: NetworkCard[];
  interfaces: InterfaceCard[];
  firewallPolicies: FirewallPolicyCard[];
  portForwards: PortForwardCard[];
  ssids: SsidCard[];
  accessPoints: AccessPointCard[];
  reverseProxies: ReverseProxyCard[];
  artifacts: ArtifactCard[];
  deployments: DeploymentCard[];
}

export interface StagingResult {
  stageDir: string;
  generatedAt: string;
  artifactCount: number;
  artifacts: StagedArtifact[];
}

export interface StagedArtifact {
  logicalName: string;
  stagePath: string;
  targetPath: string;
  checksum: string;
  contents: string;
}

export interface NetworkCard {
  name: string;
  cidr: string;
  zone: string;
  vlan: number | null;
  vlanLabel: string | null;
  interface: string;
  purpose: string;
}

export interface InterfaceCard {
  logicalName: string;
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

export interface PortForwardCard {
  name: string;
  protocol: string;
  externalPort: number;
  destinationHost: string;
  destinationPort: number;
  sourceZone: string;
  summary: string;
}

export interface SsidCard {
  name: string;
  vlan: number;
  vlanLabel: string;
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

export interface ReverseProxyCard {
  name: string;
  provider: string;
  serverNames: string[];
  listenPort: number;
  backendHost: string;
  backendPort: number;
  backendScheme: string;
  tlsMode: string;
}

export interface ArtifactCard {
  logicalName: string;
  targetPath: string;
  renderer: string;
  changeState: "planned" | "staged" | "synced";
}

export interface DeploymentCard {
  id: string;
  timestamp: string;
  status: "planned" | "applied" | "rolled_back";
  summary: string;
}
