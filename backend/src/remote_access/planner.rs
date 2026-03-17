use std::collections::{HashMap, HashSet};

use thiserror::Error;

use crate::{
    domain::{
        DesiredPublication, PublicationPlan, PublicationResult, PublicationRule, PublicationTarget,
        RemoteDnsProviderDef, RemoteDnsProviderKind, SiteConfig,
    },
    validate::{IssueSeverity, ValidationIssue},
};

use super::provider::provider_backend;

#[derive(Debug, Error)]
pub enum RemoteAccessError {
    #[error("remote access configuration is invalid")]
    InvalidConfiguration(Vec<ValidationIssue>),
}

pub fn validate_remote_access(site: &SiteConfig) -> Vec<ValidationIssue> {
    let mut issues = Vec::new();
    let mut provider_ids = HashSet::new();

    for provider in &site.remote_access.providers {
        if !provider_ids.insert(provider.id.clone()) {
            issues.push(error(
                &format!("remote_access.providers.{}", provider.id),
                &format!("duplicate remote access provider `{}`", provider.id),
            ));
        }

        let backend = provider_backend(&provider.provider);
        for message in backend.validate_definition(provider) {
            let severity = if message.contains("should set") {
                IssueSeverity::Warning
            } else {
                IssueSeverity::Error
            };
            issues.push(issue(
                severity,
                &format!("remote_access.providers.{}", provider.id),
                &message,
            ));
        }
    }

    let provider_map = site
        .remote_access
        .providers
        .iter()
        .map(|provider| (provider.id.as_str(), provider))
        .collect::<HashMap<_, _>>();
    let host_names = site
        .hosts
        .iter()
        .map(|host| host.name.as_str())
        .collect::<HashSet<_>>();
    let mut external_names = HashSet::new();
    let mut publication_targets = HashSet::new();
    let mut wan_hostnames = HashSet::new();

    for publication in &site.remote_access.publications {
        let service_name = publication.target.service_name();
        let path = format!("remote_access.services.{service_name}");

        if !host_names.contains(service_name) {
            issues.push(error(
                &path,
                &format!("unknown publication target service `{service_name}`"),
            ));
        }

        if !publication.enabled {
            continue;
        }

        if !publication_targets.insert(service_name.to_string()) {
            issues.push(error(
                &path,
                &format!("conflicting publication definitions for `{service_name}`"),
            ));
        }

        if publication.target_port == 0 {
            issues.push(error(
                &path,
                "enabled remote publication requires a non-zero target port",
            ));
        }

        let Some(provider_id) = publication
            .provider
            .as_deref()
            .filter(|value| !value.trim().is_empty())
        else {
            issues.push(error(
                &path,
                "enabled remote publication requires a provider",
            ));
            continue;
        };

        let Some(provider) = provider_map.get(provider_id) else {
            issues.push(error(
                &path,
                &format!("unknown remote access provider `{provider_id}`"),
            ));
            continue;
        };

        validate_publication_provider(provider, publication, &path, &mut issues);

        let backend = provider_backend(&provider.provider);
        let Some(external_name) =
            backend.resolve_external_name(provider, publication.publish_as.as_deref())
        else {
            issues.push(error(
                &path,
                "enabled remote publication must resolve to an external hostname",
            ));
            continue;
        };

        if !external_names.insert(external_name.clone()) {
            issues.push(error(
                &path,
                &format!("duplicate external publication name `{external_name}`"),
            ));
        }
    }

    for update in &site.remote_access.wan_updates {
        let path = format!("remote_access.wan_updates.{}", update.name);

        if !update.enabled {
            continue;
        }

        if update.provider.trim().is_empty() {
            issues.push(error(
                &path,
                "WAN address synchronization requires a provider",
            ));
            continue;
        }

        if update.hostname.trim().is_empty() {
            issues.push(error(
                &path,
                "WAN address synchronization requires a hostname",
            ));
        }

        let Some(provider) = provider_map.get(update.provider.as_str()) else {
            issues.push(error(
                &path,
                &format!("unknown remote access provider `{}`", update.provider),
            ));
            continue;
        };

        let backend = provider_backend(&provider.provider);
        if !backend.capabilities().wan_address_sync {
            issues.push(error(
                &path,
                &format!(
                    "provider `{}` does not support WAN address synchronization",
                    update.provider
                ),
            ));
        }

        if !wan_hostnames.insert(update.hostname.clone()) {
            issues.push(error(
                &path,
                &format!("conflicting WAN update hostname `{}`", update.hostname),
            ));
        }
    }

    issues
}

pub fn plan_remote_access(site: &SiteConfig) -> Result<PublicationPlan, RemoteAccessError> {
    let validation_issues = validate_remote_access(site)
        .into_iter()
        .filter(|issue| matches!(issue.severity, IssueSeverity::Error))
        .collect::<Vec<_>>();
    if !validation_issues.is_empty() {
        return Err(RemoteAccessError::InvalidConfiguration(validation_issues));
    }

    let provider_map = site
        .remote_access
        .providers
        .iter()
        .map(|provider| (provider.id.clone(), provider))
        .collect::<HashMap<_, _>>();

    let publications = site
        .remote_access
        .publications
        .iter()
        .filter(|publication| publication.enabled)
        .map(|publication| build_desired_publication(site, &provider_map, publication))
        .collect::<Option<Vec<_>>>()
        .ok_or_else(|| RemoteAccessError::InvalidConfiguration(validate_remote_access(site)))?;

    let wan_updates = site
        .remote_access
        .wan_updates
        .iter()
        .filter(|update| update.enabled)
        .cloned()
        .collect::<Vec<_>>();

    let mut dns_records = Vec::new();
    for provider in &site.remote_access.providers {
        let provider_publications = publications
            .iter()
            .filter(|publication| publication.provider == provider.id)
            .cloned()
            .collect::<Vec<_>>();
        let provider_wan_updates = wan_updates
            .iter()
            .filter(|update| update.provider == provider.id)
            .cloned()
            .collect::<Vec<_>>();

        if provider_publications.is_empty() && provider_wan_updates.is_empty() {
            continue;
        }

        let backend = provider_backend(&provider.provider);
        dns_records.extend(backend.plan_dns_records(
            provider,
            &provider_publications,
            &provider_wan_updates,
        ));
    }

    Ok(PublicationPlan {
        publications,
        dns_records,
        wan_updates,
        warnings: Vec::new(),
    })
}

pub fn remote_access_status(
    site: &SiteConfig,
) -> Result<Vec<PublicationResult>, RemoteAccessError> {
    let plan = plan_remote_access(site)?;
    let mut results = Vec::new();

    for provider in &site.remote_access.providers {
        let provider_wan_updates = plan
            .wan_updates
            .iter()
            .filter(|update| update.provider == provider.id)
            .cloned()
            .collect::<Vec<_>>();
        let backend = provider_backend(&provider.provider);
        results.extend(backend.apply_publications(provider, &plan));
        if !provider_wan_updates.is_empty() {
            results.extend(backend.sync_wan_address(provider, &provider_wan_updates));
        }
    }

    Ok(results)
}

fn build_desired_publication(
    site: &SiteConfig,
    provider_map: &HashMap<String, &RemoteDnsProviderDef>,
    publication: &PublicationRule,
) -> Option<DesiredPublication> {
    let target = publication.target.clone();
    let service_name = target.service_name();
    let provider_id = publication.provider.as_ref()?;
    let provider = provider_map.get(provider_id)?;
    let backend = provider_backend(&provider.provider);
    let external_name =
        backend.resolve_external_name(provider, publication.publish_as.as_deref())?;
    let target_address = resolve_target_address(site, &target)?;

    Some(DesiredPublication {
        service: service_name.to_string(),
        provider: provider.id.clone(),
        external_name,
        protocol: publication.protocol.clone(),
        target_port: publication.target_port,
        audience: publication.audience.clone(),
        exposure_mode: publication.exposure_mode.clone(),
        target,
        target_address,
    })
}

fn resolve_target_address(site: &SiteConfig, target: &PublicationTarget) -> Option<String> {
    let host = site
        .hosts
        .iter()
        .find(|host| host.name == target.service_name())?;

    host.management_ip.clone().or_else(|| {
        host.interfaces
            .iter()
            .filter_map(|iface| iface.address.as_deref())
            .find_map(strip_prefix_length)
    })
}

fn strip_prefix_length(address: &str) -> Option<String> {
    let value = address.split('/').next()?.trim();
    if value.is_empty() || value.eq_ignore_ascii_case("dhcp") {
        None
    } else {
        Some(value.to_string())
    }
}

fn validate_publication_provider(
    provider: &&RemoteDnsProviderDef,
    publication: &PublicationRule,
    path: &str,
    issues: &mut Vec<ValidationIssue>,
) {
    let backend = provider_backend(&provider.provider);
    let capabilities = backend.capabilities();

    if !capabilities.service_publication {
        issues.push(error(
            path,
            &format!(
                "provider `{}` does not support service publication",
                provider.id
            ),
        ));
    }

    if matches!(
        publication.exposure_mode,
        crate::domain::ExposureMode::Tunnel
    ) && !capabilities.supports_tunnel
    {
        issues.push(error(
            path,
            &format!(
                "provider `{}` does not support tunnel publication mode",
                provider.id
            ),
        ));
    }

    if !matches!(
        publication.exposure_mode,
        crate::domain::ExposureMode::Tunnel
    ) && !capabilities.supports_direct
    {
        issues.push(error(
            path,
            &format!(
                "provider `{}` does not support direct publication mode",
                provider.id
            ),
        ));
    }

    if matches!(
        provider.provider,
        RemoteDnsProviderKind::ManagedSubdomain(_)
    ) && publication
        .publish_as
        .as_deref()
        .unwrap_or("")
        .trim()
        .is_empty()
    {
        issues.push(error(
            path,
            "managed subdomain publications require `publish_as`",
        ));
    }
}

fn issue(severity: IssueSeverity, path: &str, message: &str) -> ValidationIssue {
    ValidationIssue {
        severity,
        path: path.to_string(),
        message: message.to_string(),
    }
}

fn error(path: &str, message: &str) -> ValidationIssue {
    issue(IssueSeverity::Error, path, message)
}
