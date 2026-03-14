import { SiteViewModel } from "../types/site";

export const mockSite: SiteViewModel = {
  name: "Lantricate Demo Site",
  description: "Intent model for a Linux gateway, segmented Wi-Fi, and staged artifact generation.",
  reverseProxyProvider: "nginx",
  networks: [
    {
      name: "lab",
      cidr: "10.0.0.0/24",
      zone: "lab",
      vlan: null,
      interface: "enp2s0",
      purpose: "Trusted lab and management network"
    },
    {
      name: "wifi",
      cidr: "10.0.10.0/24",
      zone: "wifi",
      vlan: 10,
      interface: "enp3s0.10",
      purpose: "Primary client Wi-Fi"
    },
    {
      name: "iot",
      cidr: "10.0.20.0/24",
      zone: "iot",
      vlan: 20,
      interface: "enp3s0.20",
      purpose: "Restricted IoT segment"
    },
    {
      name: "guest",
      cidr: "10.0.30.0/24",
      zone: "guest",
      vlan: 30,
      interface: "enp3s0.30",
      purpose: "Internet-only guest Wi-Fi"
    }
  ],
  interfaces: [
    { name: "enp1s0", role: "wan", addresses: ["dhcp"], networkRefs: [] },
    { name: "enp2s0", role: "lan", addresses: ["10.0.0.1/24"], networkRefs: ["lab"] },
    {
      name: "enp3s0",
      role: "wifi_uplink",
      addresses: ["10.0.10.1/24"],
      networkRefs: ["wifi", "iot", "guest"]
    }
  ],
  firewallPolicies: [
    {
      name: "lab-anywhere",
      sourceZone: "lab",
      destinationZone: "wan",
      action: "accept",
      summary: "Trusted lab clients can access anything upstream."
    },
    {
      name: "wifi-internet",
      sourceZone: "wifi",
      destinationZone: "wan",
      action: "accept",
      summary: "Wi-Fi clients can reach DNS, HTTP, and HTTPS."
    },
    {
      name: "iot-jellyfin",
      sourceZone: "iot",
      destinationZone: "lab",
      action: "accept",
      summary: "IoT devices can reach Jellyfin only."
    },
    {
      name: "guest-internet",
      sourceZone: "guest",
      destinationZone: "wan",
      action: "accept",
      summary: "Guest clients can access the internet only."
    }
  ],
  portForwards: [
    {
      name: "jellyfin-https",
      protocol: "tcp",
      externalPort: 8443,
      destinationHost: "jellyfin",
      destinationPort: 8096,
      sourceZone: "wan",
      summary: "Forward WAN 8443 to the internal Jellyfin service."
    }
  ],
  ssids: [
    { name: "HomeWiFi", vlan: 10, zone: "wifi", groups: ["indoor"] },
    { name: "IoTWiFi", vlan: 20, zone: "iot", groups: ["indoor"] },
    { name: "GuestWiFi", vlan: 30, zone: "guest", groups: ["indoor"] }
  ],
  accessPoints: [
    {
      name: "ap1",
      managementIp: "10.0.10.2",
      group: "indoor",
      uplinkPort: "core-sw1 ge-0/0/10",
      ssids: ["HomeWiFi", "IoTWiFi", "GuestWiFi"]
    },
    {
      name: "ap2",
      managementIp: "10.0.10.3",
      group: "indoor",
      uplinkPort: "core-sw1 ge-0/0/11",
      ssids: ["HomeWiFi", "GuestWiFi"]
    }
  ],
  reverseProxies: [
    {
      name: "jellyfin-proxy",
      provider: "nginx",
      serverNames: ["jellyfin.example.lan", "media.example.lan"],
      listenPort: 443,
      backendHost: "jellyfin",
      backendPort: 8096,
      backendScheme: "http",
      tlsMode: "terminate_at_proxy"
    }
  ],
  artifacts: [
    {
      logicalName: "reverse_proxy_main",
      targetPath: "/etc/nginx/conf.d/lantricate-proxies.conf",
      renderer: "nginx",
      changeState: "changed"
    },
    {
      logicalName: "dnsmasq_main",
      targetPath: "/etc/dnsmasq.d/lantricate.conf",
      renderer: "dnsmasq",
      changeState: "changed"
    },
    {
      logicalName: "nftables_main",
      targetPath: "/etc/nftables.d/lantricate.nft",
      renderer: "nftables",
      changeState: "changed"
    },
    {
      logicalName: "networking_main",
      targetPath: "/etc/systemd/network/90-lantricate.network",
      renderer: "networking",
      changeState: "changed"
    }
  ],
  deployments: [
    {
      id: "dep-2026-03-13-001",
      timestamp: "2026-03-13T09:18:00Z",
      status: "planned",
      summary: "Stage 1 render only. No apply or rollback execution yet."
    }
  ]
};
