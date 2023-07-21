use serde::{Deserialize, Serialize};
use serde_with::skip_serializing_none;

/// Describes the number of selections that must occur. Without this setting, only one value should be assumed to be permitted.
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, PartialOrd)]
#[serde(rename_all = "kebab-case")]
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
#[serde(rename_all = "kebab-case")]
pub struct Select {
    /// Describes the number of selections that must occur. Without this setting, only one value should be assumed to be permitted.
    pub how_many: Option<Cardinality>,
    /// A value selection among several such options
    pub choice: Option<Vec<String>>,
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_de() {
        let json = r#"{"how-many":"one-or-more"}"#;

        let result = serde_json::from_str::<Select>(json);
        assert!(result.is_ok());
        let select = result.unwrap();
        assert_eq!(select.how_many, Some(Cardinality::OneOrMore));
    }
}
