import { SiteViewModel } from "../types/site";

export const mockSite: SiteViewModel = {
  name: "Rivia",
  description: "Managed gateway and Wi-Fi design for a segmented home or small office network.",
  reverseProxyProvider: "nginx",
  wifiExposeAllSsidsOnAllAps: true,
  networks: [
    {
      name: "lab",
      cidr: "10.0.0.0/24",
      vlan: null,
      interface: "enp2s0",
      description: "Trusted network for infrastructure, servers, and administration"
    },
    {
      name: "rivia-home",
      cidr: "10.0.10.0/24",
      vlan: 10,
      interface: "rivia-home",
      description: "Primary network for everyday devices with normal internet access"
    },
    {
      name: "rivia-iot",
      cidr: "10.0.20.0/24",
      vlan: 20,
      interface: "rivia-iot",
      description: "Restricted network for smart home and embedded devices"
    },
    {
      name: "rivia-guest",
      cidr: "10.0.30.0/24",
      vlan: 30,
      interface: "rivia-guest",
      description: "Internet-only network for guests"
    },
    {
      name: "labwifi",
      cidr: "10.0.40.0/24",
      vlan: 40,
      interface: "labwifi",
      description: "Trusted Wi-Fi network for lab devices and administration"
    }
  ],
  interfaces: [
    { logicalName: "wan", name: "enp1s0", role: "wan", addresses: ["dhcp"], networkRefs: [] },
    { logicalName: "lan", name: "enp2s0", role: "lan", addresses: ["10.0.0.1/24"], networkRefs: ["lab"] },
    {
      logicalName: "wifi",
      name: "enp3s0",
      role: "wifi",
      addresses: ["10.0.10.1/24"],
      networkRefs: ["rivia-home", "rivia-iot", "rivia-guest", "labwifi"]
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
      name: "rivia-home-internet",
      sourceZone: "rivia-home",
      destinationZone: "wan",
      action: "accept",
      summary: "Wi-Fi clients can reach DNS, HTTP, and HTTPS."
    },
    {
      name: "rivia-iot-jellyfin",
      sourceZone: "rivia-iot",
      destinationZone: "rivia-home",
      action: "accept",
      summary: "IoT devices can reach Jellyfin only."
    },
    {
      name: "rivia-iot-internet",
      sourceZone: "rivia-iot",
      destinationZone: "wan",
      action: "accept",
      summary: "IoT devices can reach WAN services such as streaming platforms."
    },
    {
      name: "rivia-guest-internet",
      sourceZone: "rivia-guest",
      destinationZone: "wan",
      action: "accept",
      summary: "Guest clients can access the internet only."
    },
    {
      name: "labwifi-internet",
      sourceZone: "labwifi",
      destinationZone: "wan",
      action: "accept",
      summary: "Trusted lab Wi-Fi clients can access the internet."
    },
    {
      name: "labwifi-lab",
      sourceZone: "labwifi",
      destinationZone: "lab",
      action: "accept",
      summary: "Trusted lab Wi-Fi clients can access wired lab systems."
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
    { name: "rivia-home", vlan: 10, network: "rivia-home", groups: ["indoor"] },
    { name: "rivia-iot", vlan: 20, network: "rivia-iot", groups: ["indoor"] },
    { name: "rivia-guest", vlan: 30, network: "rivia-guest", groups: ["indoor"] },
    { name: "labwifi", vlan: 40, network: "labwifi", groups: ["indoor"] }
  ],
  accessPoints: [
    {
      name: "ap1",
      managementIp: "10.0.10.2",
      group: "indoor",
      uplinkPort: "core-sw1 ge-0/0/10",
      ssids: ["rivia-home", "rivia-iot", "rivia-guest", "labwifi"]
    },
    {
      name: "ap2",
      managementIp: "10.0.10.3",
      group: "indoor",
      uplinkPort: "core-sw1 ge-0/0/11",
      ssids: ["rivia-home", "rivia-guest", "labwifi"]
    },
    {
      name: "ap3",
      managementIp: "10.0.10.4",
      group: "indoor",
      uplinkPort: "core-sw1 ge-0/0/12",
      ssids: ["rivia-home", "rivia-iot", "rivia-guest", "labwifi"]
    },
    {
      name: "ap4",
      managementIp: "10.0.10.5",
      group: "indoor",
      uplinkPort: "core-sw1 ge-0/0/13",
      ssids: ["rivia-home", "rivia-guest", "labwifi"]
    },
    {
      name: "ap5",
      managementIp: "10.0.10.6",
      group: "indoor",
      uplinkPort: "core-sw1 ge-0/0/14",
      ssids: ["rivia-home", "rivia-iot", "rivia-guest", "labwifi"]
    },
    {
      name: "ap6",
      managementIp: "10.0.10.7",
      group: "indoor",
      uplinkPort: "core-sw1 ge-0/0/15",
      ssids: ["rivia-home", "rivia-guest", "labwifi"]
    },
    {
      name: "ap7",
      managementIp: "10.0.10.8",
      group: "indoor",
      uplinkPort: "core-sw1 ge-0/0/16",
      ssids: ["rivia-home", "rivia-iot", "rivia-guest", "labwifi"]
    },
    {
      name: "ap8",
      managementIp: "10.0.10.9",
      group: "indoor",
      uplinkPort: "core-sw1 ge-0/0/17",
      ssids: ["rivia-home", "rivia-guest", "labwifi"]
    }
  ],
  reverseProxies: [
    {
      name: "jellyfin-proxy",
      provider: "nginx",
      serverNames: ["jellyfin.rivia.home", "media.rivia.home"],
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
      targetPath: "/etc/nginx/conf.d/lanuminous-proxies.conf",
      renderer: "nginx",
      changeState: "planned"
    },
    {
      logicalName: "dnsmasq_main",
      targetPath: "/etc/dnsmasq.d/lanuminous.conf",
      renderer: "dnsmasq",
      changeState: "planned"
    },
    {
      logicalName: "nftables_main",
      targetPath: "/etc/nftables.d/lanuminous.nft",
      renderer: "nftables",
      changeState: "planned"
    },
    {
      logicalName: "networking_main",
      targetPath: "/etc/systemd/network/90-lanuminous.network",
      renderer: "networking",
      changeState: "synced"
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
