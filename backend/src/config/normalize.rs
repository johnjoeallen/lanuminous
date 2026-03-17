use crate::{
    config::ConfigBundle,
    domain::{
        AccessPointDef, AccessPointGroupDef, ApBackend, DhcpConfig, DhcpPool, DnsConfig, DnsRecord,
        FirewallConfig, HostDef, HostInterfaceDef, InterfaceDef, ManagedPath, Metadata, NetworkDef,
        PolicyRule, PortForwardConfig, PortForwardRule, ProxyBackend, ReservationDef,
        ReverseProxyConfig, ReverseProxyProvider, ReverseProxySite, RouteDef, ServiceDef,
        ServiceType, SiteConfig, UplinkDef, VlanDef, WifiConfig, ZoneDef,
    },
};

pub fn normalize_bundle(bundle: ConfigBundle) -> SiteConfig {
    let metadata = Metadata {
        name: bundle.site.metadata.name,
        description: bundle.site.metadata.description,
        managed_prefix: bundle
            .site
            .metadata
            .managed_prefix
            .unwrap_or_else(|| "# Managed by Lanuminous".to_string()),
    };

    let interfaces = bundle
        .interfaces
        .interfaces
        .into_iter()
        .map(|interface| InterfaceDef {
            logical_name: interface
                .logical_name
                .unwrap_or_else(|| format!("{:?}", interface.role).to_lowercase()),
            name: interface.name,
            role: interface.role,
            kind: interface.kind,
            addresses: interface.addresses,
            network_refs: interface.network_refs,
            vlan_tags: interface.vlan_tags,
        })
        .collect::<Vec<_>>();

    let networks = bundle
        .networks
        .into_iter()
        .map(|network| NetworkDef {
            name: network.name.clone(),
            cidr: network.cidr,
            zone: network.zone.unwrap_or_else(|| network.name.clone()),
            description: network
                .description
                .or_else(|| network.dns_domain.clone())
                .unwrap_or_else(|| "Managed network".to_string()),
            dns_domain: network.dns_domain,
            vlan: network.vlan.map(|id| VlanDef {
                id,
                parent_interface: network.parent_interface,
            }),
            dhcp_pool: network.dhcp.map(|dhcp| DhcpPool {
                network: network.name,
                start: dhcp.start,
                end: dhcp.end,
                lease_time: dhcp.lease_time,
            }),
            routes: network
                .routes
                .into_iter()
                .map(|route| RouteDef {
                    destination: route.destination,
                    via: route.via,
                    metric: route.metric,
                })
                .collect(),
        })
        .collect::<Vec<_>>();

    let hosts = bundle
        .hosts
        .into_iter()
        .map(|host| HostDef {
            name: host.name,
            role: host.role,
            network: host.network,
            management_ip: host.management_ip,
            interfaces: host
                .interfaces
                .into_iter()
                .map(|iface| HostInterfaceDef {
                    name: iface.name,
                    network: iface.network,
                    address: iface.address,
                })
                .collect(),
            reservations: host
                .reservations
                .into_iter()
                .map(|reservation| ReservationDef {
                    hostname: reservation.hostname,
                    ip: reservation.ip,
                    mac: reservation.mac,
                })
                .collect(),
            wifi: host.wifi.map(|wifi| crate::domain::HostWifiIntent {
                ap_group: wifi.ap_group,
                ssids: wifi.ssids,
            }),
        })
        .collect::<Vec<_>>();

    let dns = DnsConfig {
        domain: bundle.dns.domain,
        upstream_servers: bundle.dns.upstream_servers,
        static_records: bundle
            .dns
            .static_records
            .into_iter()
            .map(|record| DnsRecord {
                name: record.name,
                address: record.address,
            })
            .collect(),
    };

    let dhcp = DhcpConfig {
        default_lease_time: bundle.dhcp.default_lease_time,
        pools: networks
            .iter()
            .filter_map(|network| network.dhcp_pool.clone())
            .collect(),
        reservations: hosts
            .iter()
            .flat_map(|host| host.reservations.clone())
            .collect(),
    };

    let firewall = FirewallConfig {
        zones: bundle
            .firewall_zones
            .zones
            .into_iter()
            .map(|zone| ZoneDef {
                name: zone.name,
                networks: zone.networks,
                description: zone.description,
            })
            .collect(),
        policies: bundle
            .firewall_policies
            .policies
            .into_iter()
            .map(|policy| PolicyRule {
                name: policy.name,
                action: policy.action,
                source_zone: policy.source_zone,
                destination_zone: policy.destination_zone,
                allowed_services: policy.allowed_services,
                destination_hosts: policy.destination_hosts,
                description: policy.description,
            })
            .collect(),
    };

    let port_forwards = PortForwardConfig {
        rules: bundle
            .port_forwards
            .rules
            .into_iter()
            .map(|rule| PortForwardRule {
                name: rule.name,
                protocol: rule.protocol,
                external_port: rule.external_port,
                destination_host: rule.destination_host,
                destination_port: rule.destination_port,
                source_zone: rule.source_zone,
                description: rule.description,
            })
            .collect(),
    };

    let reverse_proxies = ReverseProxyConfig {
        provider: bundle.reverse_proxy.provider,
        sites: bundle
            .reverse_proxy
            .sites
            .into_iter()
            .map(|site| ReverseProxySite {
                name: site.name,
                server_names: site.server_names,
                listen_port: site.listen_port,
                backend: ProxyBackend {
                    host_ref: site.backend.host_ref,
                    port: site.backend.port,
                    scheme: site.backend.scheme,
                },
                tls_mode: site.tls_mode,
            })
            .collect(),
    };

    let wifi = WifiConfig {
        controller: bundle
            .wifi_ssids
            .controller
            .unwrap_or(crate::domain::ApController::Manual),
        expose_all_ssids_on_all_aps: bundle.wifi_ssids.expose_all_ssids_on_all_aps,
        ssids: bundle
            .wifi_ssids
            .ssids
            .into_iter()
            .map(|ssid| crate::domain::SsidDef {
                name: ssid.name,
                vlan: ssid.vlan,
                zone: ssid.zone,
                broadcast_groups: ssid.broadcast_groups,
            })
            .collect(),
        access_points: bundle
            .wifi_aps
            .aps
            .into_iter()
            .map(|ap| AccessPointDef {
                name: ap.name,
                management_ip: ap.management_ip,
                group: ap.group,
                backend: ap.backend.unwrap_or(ApBackend::Manual),
                uplink: UplinkDef {
                    switch_name: ap.uplink.switch_name,
                    port: ap.uplink.port,
                    native_vlan: ap.uplink.native_vlan,
                    tagged_vlans: ap.uplink.tagged_vlans,
                    expected_networks: ap.uplink.expected_networks,
                },
                ssids: ap.ssids,
            })
            .collect(),
        groups: bundle
            .wifi_groups
            .groups
            .into_iter()
            .map(|group| AccessPointGroupDef {
                name: group.name,
                ssids: group.ssids,
                ap_names: group.ap_names,
                description: group.description,
            })
            .collect(),
    };

    let mut services = vec![
        ServiceDef {
            name: "dnsmasq".to_string(),
            service_type: ServiceType::Dnsmasq,
            enabled: true,
            reload_command: Some("systemctl reload dnsmasq".to_string()),
            managed_paths: vec![ManagedPath {
                logical_name: "dnsmasq_main".to_string(),
                path: "/etc/dnsmasq.d/lanuminous.conf".to_string(),
                service: Some("dnsmasq".to_string()),
            }],
        },
        ServiceDef {
            name: "networking".to_string(),
            service_type: ServiceType::Networking,
            enabled: true,
            reload_command: Some("networkctl reload".to_string()),
            managed_paths: networking_managed_paths(&interfaces, &networks),
        },
        ServiceDef {
            name: "wifi-intent".to_string(),
            service_type: ServiceType::WifiSummary,
            enabled: true,
            reload_command: None,
            managed_paths: vec![ManagedPath {
                logical_name: "wifi_summary".to_string(),
                path: "/var/lib/lanuminous/generated/wifi-summary.txt".to_string(),
                service: None,
            }],
        },
    ];

    services.push(ServiceDef {
        name: bundle
            .nftables_service
            .as_ref()
            .map(|service| service.name.clone())
            .unwrap_or_else(|| "nftables".to_string()),
        service_type: ServiceType::Nftables,
        enabled: bundle
            .nftables_service
            .as_ref()
            .map(|service| service.enabled)
            .unwrap_or(true),
        reload_command: bundle
            .nftables_service
            .and_then(|service| service.reload_command)
            .or_else(|| Some("systemctl reload nftables".to_string())),
        managed_paths: vec![ManagedPath {
            logical_name: "nftables_main".to_string(),
            path: "/etc/nftables.d/lanuminous.nft".to_string(),
            service: Some("nftables".to_string()),
        }],
    });

    if !reverse_proxies.sites.is_empty() {
        let (service_name, service_type, reload_command, managed_path) =
            reverse_proxy_service_definition(&reverse_proxies.provider);
        services.push(ServiceDef {
            name: service_name,
            service_type,
            enabled: true,
            reload_command: Some(reload_command),
            managed_paths: vec![ManagedPath {
                logical_name: "reverse_proxy_main".to_string(),
                path: managed_path,
                service: Some("reverse-proxy".to_string()),
            }],
        });
    }

    SiteConfig {
        metadata,
        interfaces,
        networks,
        hosts,
        services,
        dns: Some(dns),
        dhcp: Some(dhcp),
        port_forwards,
        reverse_proxies,
        firewall,
        wifi,
        switches: Vec::new(),
    }
}

fn reverse_proxy_service_definition(
    provider: &ReverseProxyProvider,
) -> (String, ServiceType, String, String) {
    match provider {
        ReverseProxyProvider::Apache2 => (
            "apache2".to_string(),
            ServiceType::Apache2,
            "systemctl reload apache2".to_string(),
            "/etc/apache2/sites-available/lanuminous-proxies.conf".to_string(),
        ),
        ReverseProxyProvider::Nginx => (
            "nginx".to_string(),
            ServiceType::Nginx,
            "systemctl reload nginx".to_string(),
            "/etc/nginx/conf.d/lanuminous-proxies.conf".to_string(),
        ),
        ReverseProxyProvider::Caddy => (
            "caddy".to_string(),
            ServiceType::Caddy,
            "systemctl reload caddy".to_string(),
            "/etc/caddy/conf.d/lanuminous-proxies.caddy".to_string(),
        ),
        ReverseProxyProvider::Traefik => (
            "traefik".to_string(),
            ServiceType::Traefik,
            "systemctl reload traefik".to_string(),
            "/etc/traefik/dynamic/lanuminous.yml".to_string(),
        ),
        ReverseProxyProvider::Haproxy => (
            "haproxy".to_string(),
            ServiceType::Haproxy,
            "systemctl reload haproxy".to_string(),
            "/etc/haproxy/lanuminous.cfg".to_string(),
        ),
    }
}

fn networking_managed_paths(
    interfaces: &[InterfaceDef],
    networks: &[NetworkDef],
) -> Vec<ManagedPath> {
    let mut managed_paths = Vec::new();

    for interface in interfaces {
        managed_paths.push(ManagedPath {
            logical_name: format!("networking_link_{}", interface.logical_name),
            path: format!("/etc/systemd/network/10-{}.link", interface.logical_name),
            service: Some("systemd-networkd".to_string()),
        });
        managed_paths.push(ManagedPath {
            logical_name: format!("networking_main_{}", interface.logical_name),
            path: format!("/etc/systemd/network/20-{}.network", interface.logical_name),
            service: Some("systemd-networkd".to_string()),
        });
    }

    for network in networks.iter().filter(|network| network.vlan.is_some()) {
        managed_paths.push(ManagedPath {
            logical_name: format!("networking_vlan_netdev_{}", network.name),
            path: format!("/etc/systemd/network/30-{}.netdev", network.name),
            service: Some("systemd-networkd".to_string()),
        });
        managed_paths.push(ManagedPath {
            logical_name: format!("networking_vlan_network_{}", network.name),
            path: format!("/etc/systemd/network/40-{}.network", network.name),
            service: Some("systemd-networkd".to_string()),
        });
    }

    managed_paths.sort_by(|left, right| left.logical_name.cmp(&right.logical_name));
    managed_paths
}
