use crate::domain::{
    DesiredDnsPurpose, DesiredDnsRecord, DesiredDnsRecordType, DesiredDnsValueSource,
    DesiredPublication, DynamicDnsProviderDef, ExposureMode, ManagedSubdomainProviderDef,
    PublicationPlan, PublicationResult, PublicationStatus, RemoteDnsProviderDef,
    RemoteDnsProviderKind, WanAddressUpdate,
};

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct ProviderCapabilities {
    pub service_publication: bool,
    pub wan_address_sync: bool,
    pub supports_direct: bool,
    pub supports_tunnel: bool,
}

pub trait RemoteAccessProvider {
    fn kind_name(&self) -> &'static str;
    fn capabilities(&self) -> ProviderCapabilities;
    fn validate_definition(&self, provider: &RemoteDnsProviderDef) -> Vec<String>;
    fn resolve_external_name(
        &self,
        provider: &RemoteDnsProviderDef,
        publish_as: Option<&str>,
    ) -> Option<String>;
    fn plan_dns_records(
        &self,
        provider: &RemoteDnsProviderDef,
        publications: &[DesiredPublication],
        wan_updates: &[WanAddressUpdate],
    ) -> Vec<DesiredDnsRecord>;
    fn apply_publications(
        &self,
        provider: &RemoteDnsProviderDef,
        plan: &PublicationPlan,
    ) -> Vec<PublicationResult>;
    fn remove_publications(
        &self,
        provider: &RemoteDnsProviderDef,
        plan: &PublicationPlan,
    ) -> Vec<PublicationResult>;
    fn sync_wan_address(
        &self,
        provider: &RemoteDnsProviderDef,
        updates: &[WanAddressUpdate],
    ) -> Vec<PublicationResult>;
}

pub fn provider_backend(kind: &RemoteDnsProviderKind) -> Box<dyn RemoteAccessProvider> {
    match kind {
        RemoteDnsProviderKind::ManagedSubdomain(_) => Box::new(ManagedSubdomainProvider),
        RemoteDnsProviderKind::JokerDynDns(_) => Box::new(JokerDynDnsProvider),
        RemoteDnsProviderKind::GenericDynDns(_) => Box::new(GenericDynDnsProvider),
        RemoteDnsProviderKind::Manual(_) => Box::new(ManualProvider),
    }
}

struct ManagedSubdomainProvider;
struct JokerDynDnsProvider;
struct GenericDynDnsProvider;
struct ManualProvider;

impl RemoteAccessProvider for ManagedSubdomainProvider {
    fn kind_name(&self) -> &'static str {
        "managed_subdomain"
    }

    fn capabilities(&self) -> ProviderCapabilities {
        ProviderCapabilities {
            service_publication: true,
            wan_address_sync: false,
            supports_direct: true,
            supports_tunnel: true,
        }
    }

    fn validate_definition(&self, provider: &RemoteDnsProviderDef) -> Vec<String> {
        let mut issues = Vec::new();

        match &provider.provider {
            RemoteDnsProviderKind::ManagedSubdomain(ManagedSubdomainProviderDef { zone }) => {
                if zone.trim().is_empty() {
                    issues.push("managed_subdomain providers require `zone`".to_string());
                }
            }
            _ => issues.push("provider kind mismatch for managed_subdomain backend".to_string()),
        }

        issues
    }

    fn resolve_external_name(
        &self,
        provider: &RemoteDnsProviderDef,
        publish_as: Option<&str>,
    ) -> Option<String> {
        let publish_as = publish_as?.trim();
        if publish_as.is_empty() {
            return None;
        }
        if publish_as.contains('.') {
            return Some(publish_as.to_string());
        }

        match &provider.provider {
            RemoteDnsProviderKind::ManagedSubdomain(def) if !def.zone.trim().is_empty() => {
                Some(format!("{publish_as}.{}", def.zone))
            }
            _ => None,
        }
    }

    fn plan_dns_records(
        &self,
        _provider: &RemoteDnsProviderDef,
        publications: &[DesiredPublication],
        _wan_updates: &[WanAddressUpdate],
    ) -> Vec<DesiredDnsRecord> {
        publications
            .iter()
            .map(|publication| DesiredDnsRecord {
                provider: publication.provider.clone(),
                name: publication.external_name.clone(),
                record_type: DesiredDnsRecordType::A,
                value_source: DesiredDnsValueSource::WanAddress,
                ttl: Some(300),
                purpose: DesiredDnsPurpose::ServicePublication,
            })
            .collect()
    }

    fn apply_publications(
        &self,
        provider: &RemoteDnsProviderDef,
        plan: &PublicationPlan,
    ) -> Vec<PublicationResult> {
        let count = plan
            .publications
            .iter()
            .filter(|publication| publication.provider == provider.id)
            .count();

        vec![PublicationResult {
            provider: provider.id.clone(),
            action: "apply_publications".to_string(),
            status: PublicationStatus::NotImplemented,
            message: format!(
                "{} planned publication(s) for managed subdomain provider `{}`",
                count, provider.id
            ),
        }]
    }

    fn remove_publications(
        &self,
        provider: &RemoteDnsProviderDef,
        _plan: &PublicationPlan,
    ) -> Vec<PublicationResult> {
        vec![PublicationResult {
            provider: provider.id.clone(),
            action: "remove_publications".to_string(),
            status: PublicationStatus::NotImplemented,
            message: "publication removal is not implemented yet".to_string(),
        }]
    }

    fn sync_wan_address(
        &self,
        provider: &RemoteDnsProviderDef,
        _updates: &[WanAddressUpdate],
    ) -> Vec<PublicationResult> {
        vec![PublicationResult {
            provider: provider.id.clone(),
            action: "sync_wan_address".to_string(),
            status: PublicationStatus::NotImplemented,
            message: "managed subdomain providers do not support WAN sync in this stage"
                .to_string(),
        }]
    }
}

impl RemoteAccessProvider for JokerDynDnsProvider {
    fn kind_name(&self) -> &'static str {
        "joker_dyndns"
    }

    fn capabilities(&self) -> ProviderCapabilities {
        ProviderCapabilities {
            service_publication: true,
            wan_address_sync: true,
            supports_direct: true,
            supports_tunnel: false,
        }
    }

    fn validate_definition(&self, provider: &RemoteDnsProviderDef) -> Vec<String> {
        validate_dynamic_provider(provider, true)
    }

    fn resolve_external_name(
        &self,
        provider: &RemoteDnsProviderDef,
        publish_as: Option<&str>,
    ) -> Option<String> {
        resolve_dynamic_external_name(provider, publish_as)
    }

    fn plan_dns_records(
        &self,
        _provider: &RemoteDnsProviderDef,
        publications: &[DesiredPublication],
        wan_updates: &[WanAddressUpdate],
    ) -> Vec<DesiredDnsRecord> {
        dynamic_dns_records(publications, wan_updates)
    }

    fn apply_publications(
        &self,
        provider: &RemoteDnsProviderDef,
        plan: &PublicationPlan,
    ) -> Vec<PublicationResult> {
        let count = plan
            .publications
            .iter()
            .filter(|publication| publication.provider == provider.id)
            .count();

        vec![PublicationResult {
            provider: provider.id.clone(),
            action: "apply_publications".to_string(),
            status: PublicationStatus::Planned,
            message: format!(
                "dynamic DNS provider `{}` would publish {} direct-service record(s)",
                provider.id, count
            ),
        }]
    }

    fn remove_publications(
        &self,
        provider: &RemoteDnsProviderDef,
        _plan: &PublicationPlan,
    ) -> Vec<PublicationResult> {
        vec![PublicationResult {
            provider: provider.id.clone(),
            action: "remove_publications".to_string(),
            status: PublicationStatus::NotImplemented,
            message: "dynamic DNS publication removal is not implemented yet".to_string(),
        }]
    }

    fn sync_wan_address(
        &self,
        provider: &RemoteDnsProviderDef,
        updates: &[WanAddressUpdate],
    ) -> Vec<PublicationResult> {
        vec![PublicationResult {
            provider: provider.id.clone(),
            action: "sync_wan_address".to_string(),
            status: PublicationStatus::Planned,
            message: format!(
                "dynamic DNS provider `{}` would synchronize {} WAN hostname(s)",
                provider.id,
                updates.len()
            ),
        }]
    }
}

impl RemoteAccessProvider for GenericDynDnsProvider {
    fn kind_name(&self) -> &'static str {
        "generic_dyndns"
    }

    fn capabilities(&self) -> ProviderCapabilities {
        ProviderCapabilities {
            service_publication: true,
            wan_address_sync: true,
            supports_direct: true,
            supports_tunnel: false,
        }
    }

    fn validate_definition(&self, provider: &RemoteDnsProviderDef) -> Vec<String> {
        validate_dynamic_provider(provider, false)
    }

    fn resolve_external_name(
        &self,
        provider: &RemoteDnsProviderDef,
        publish_as: Option<&str>,
    ) -> Option<String> {
        resolve_dynamic_external_name(provider, publish_as)
    }

    fn plan_dns_records(
        &self,
        _provider: &RemoteDnsProviderDef,
        publications: &[DesiredPublication],
        wan_updates: &[WanAddressUpdate],
    ) -> Vec<DesiredDnsRecord> {
        dynamic_dns_records(publications, wan_updates)
    }

    fn apply_publications(
        &self,
        provider: &RemoteDnsProviderDef,
        plan: &PublicationPlan,
    ) -> Vec<PublicationResult> {
        let count = plan
            .publications
            .iter()
            .filter(|publication| publication.provider == provider.id)
            .count();

        vec![PublicationResult {
            provider: provider.id.clone(),
            action: "apply_publications".to_string(),
            status: PublicationStatus::Planned,
            message: format!(
                "generic dynamic DNS provider `{}` would publish {} direct-service record(s)",
                provider.id, count
            ),
        }]
    }

    fn remove_publications(
        &self,
        provider: &RemoteDnsProviderDef,
        _plan: &PublicationPlan,
    ) -> Vec<PublicationResult> {
        vec![PublicationResult {
            provider: provider.id.clone(),
            action: "remove_publications".to_string(),
            status: PublicationStatus::NotImplemented,
            message: "dynamic DNS publication removal is not implemented yet".to_string(),
        }]
    }

    fn sync_wan_address(
        &self,
        provider: &RemoteDnsProviderDef,
        updates: &[WanAddressUpdate],
    ) -> Vec<PublicationResult> {
        vec![PublicationResult {
            provider: provider.id.clone(),
            action: "sync_wan_address".to_string(),
            status: PublicationStatus::Planned,
            message: format!(
                "generic dynamic DNS provider `{}` would synchronize {} WAN hostname(s)",
                provider.id,
                updates.len()
            ),
        }]
    }
}

impl RemoteAccessProvider for ManualProvider {
    fn kind_name(&self) -> &'static str {
        "manual"
    }

    fn capabilities(&self) -> ProviderCapabilities {
        ProviderCapabilities {
            service_publication: true,
            wan_address_sync: true,
            supports_direct: true,
            supports_tunnel: true,
        }
    }

    fn validate_definition(&self, _provider: &RemoteDnsProviderDef) -> Vec<String> {
        Vec::new()
    }

    fn resolve_external_name(
        &self,
        provider: &RemoteDnsProviderDef,
        publish_as: Option<&str>,
    ) -> Option<String> {
        let publish_as = publish_as?.trim();
        if publish_as.is_empty() {
            return None;
        }
        if publish_as.contains('.') {
            return Some(publish_as.to_string());
        }

        match &provider.provider {
            RemoteDnsProviderKind::Manual(def) => def
                .base_domain
                .as_ref()
                .filter(|domain| !domain.trim().is_empty())
                .map(|domain| format!("{publish_as}.{domain}")),
            _ => None,
        }
    }

    fn plan_dns_records(
        &self,
        _provider: &RemoteDnsProviderDef,
        publications: &[DesiredPublication],
        wan_updates: &[WanAddressUpdate],
    ) -> Vec<DesiredDnsRecord> {
        dynamic_dns_records(publications, wan_updates)
    }

    fn apply_publications(
        &self,
        provider: &RemoteDnsProviderDef,
        plan: &PublicationPlan,
    ) -> Vec<PublicationResult> {
        let count = plan
            .publications
            .iter()
            .filter(|publication| publication.provider == provider.id)
            .count();

        vec![PublicationResult {
            provider: provider.id.clone(),
            action: "apply_publications".to_string(),
            status: PublicationStatus::NotImplemented,
            message: format!(
                "manual provider `{}` requires manual publication of {} record(s)",
                provider.id, count
            ),
        }]
    }

    fn remove_publications(
        &self,
        provider: &RemoteDnsProviderDef,
        _plan: &PublicationPlan,
    ) -> Vec<PublicationResult> {
        vec![PublicationResult {
            provider: provider.id.clone(),
            action: "remove_publications".to_string(),
            status: PublicationStatus::NotImplemented,
            message: "manual provider cleanup is not implemented yet".to_string(),
        }]
    }

    fn sync_wan_address(
        &self,
        provider: &RemoteDnsProviderDef,
        updates: &[WanAddressUpdate],
    ) -> Vec<PublicationResult> {
        vec![PublicationResult {
            provider: provider.id.clone(),
            action: "sync_wan_address".to_string(),
            status: PublicationStatus::NotImplemented,
            message: format!(
                "manual provider `{}` requires manual WAN sync for {} hostname(s)",
                provider.id,
                updates.len()
            ),
        }]
    }
}

fn validate_dynamic_provider(
    provider: &RemoteDnsProviderDef,
    require_service_name: bool,
) -> Vec<String> {
    let mut issues = Vec::new();

    match &provider.provider {
        RemoteDnsProviderKind::JokerDynDns(DynamicDnsProviderDef {
            service,
            update_url,
            ..
        }) => {
            if require_service_name && service.as_deref().unwrap_or("").trim().is_empty() {
                issues.push("joker_dyndns providers require `service`".to_string());
            }
            if provider
                .credential_ref
                .as_deref()
                .unwrap_or("")
                .trim()
                .is_empty()
            {
                issues.push("dynamic DNS providers should set `credential_ref`".to_string());
            }
            let _ = update_url;
        }
        RemoteDnsProviderKind::GenericDynDns(DynamicDnsProviderDef { update_url, .. }) => {
            if update_url.as_deref().unwrap_or("").trim().is_empty() {
                issues.push("generic_dyndns providers require `update_url`".to_string());
            }
        }
        _ => issues.push("provider kind mismatch for dynamic DNS backend".to_string()),
    }

    issues
}

fn resolve_dynamic_external_name(
    provider: &RemoteDnsProviderDef,
    publish_as: Option<&str>,
) -> Option<String> {
    let publish_as = publish_as.map(str::trim).filter(|name| !name.is_empty());
    if let Some(name) = publish_as.filter(|name| name.contains('.')) {
        return Some(name.to_string());
    }

    match &provider.provider {
        RemoteDnsProviderKind::JokerDynDns(def) | RemoteDnsProviderKind::GenericDynDns(def) => {
            if def.hostname.trim().is_empty() {
                None
            } else {
                Some(def.hostname.clone())
            }
        }
        _ => None,
    }
}

fn dynamic_dns_records(
    publications: &[DesiredPublication],
    wan_updates: &[WanAddressUpdate],
) -> Vec<DesiredDnsRecord> {
    let mut records = publications
        .iter()
        .filter(|publication| !matches!(publication.exposure_mode, ExposureMode::Tunnel))
        .map(|publication| DesiredDnsRecord {
            provider: publication.provider.clone(),
            name: publication.external_name.clone(),
            record_type: DesiredDnsRecordType::A,
            value_source: DesiredDnsValueSource::WanAddress,
            ttl: Some(300),
            purpose: DesiredDnsPurpose::ServicePublication,
        })
        .collect::<Vec<_>>();

    records.extend(wan_updates.iter().map(|update| DesiredDnsRecord {
        provider: update.provider.clone(),
        name: update.hostname.clone(),
        record_type: DesiredDnsRecordType::A,
        value_source: DesiredDnsValueSource::WanAddress,
        ttl: Some(300),
        purpose: DesiredDnsPurpose::WanSync,
    }));

    records
}
