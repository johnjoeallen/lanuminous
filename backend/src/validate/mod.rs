mod rules;

use schemars::JsonSchema;
use serde::{Deserialize, Serialize};

use crate::domain::SiteConfig;

pub use rules::validate_site;

#[derive(Debug, Clone, Serialize, Deserialize, JsonSchema, PartialEq, Eq)]
pub struct ValidationIssue {
    pub severity: IssueSeverity,
    pub path: String,
    pub message: String,
}

#[derive(Debug, Clone, Serialize, Deserialize, JsonSchema, PartialEq, Eq)]
#[serde(rename_all = "snake_case")]
pub enum IssueSeverity {
    Error,
    Warning,
}

#[derive(Debug, Clone, Serialize, Deserialize, JsonSchema, PartialEq, Eq, Default)]
pub struct ValidationReport {
    pub issues: Vec<ValidationIssue>,
}

impl ValidationReport {
    pub fn is_valid(&self) -> bool {
        !self
            .issues
            .iter()
            .any(|issue| matches!(issue.severity, IssueSeverity::Error))
    }
}

pub fn run_validation(site: &SiteConfig) -> ValidationReport {
    ValidationReport {
        issues: validate_site(site),
    }
}
