use serde::{Deserialize, Serialize};
use serde_with::skip_serializing_none;

/// Describes the number of selections that must occur. Without this setting, only one value should be assumed to be permitted.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum Cardinality {
    /// Only one value is permitted.
    #[serde(rename = "one")]
    One,
    /// One or more values are permitted.
    #[serde(rename = "one-or-more")]
    OneOrMore,
}

#[skip_serializing_none]
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Select {
    /// Describes the number of selections that must occur. Without this setting, only one value should be assumed to be permitted.
    #[serde(rename = "how-many")]
    pub how_many: Option<Cardinality>,
    /// A value selection among several such options
    pub choice: Option<Vec<String>>,
}
