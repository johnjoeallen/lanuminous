use schemars::JsonSchema;
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize, JsonSchema, PartialEq, Eq)]
pub struct Metadata {
    pub name: String,
    pub description: Option<String>,
    pub managed_prefix: String,
}
