//!
//! ## Common Types
//! uuid: RFC4122 UUID v4
//! dateTime-with-timezone: RFC3339 (ISO 8601) format, with timzedone.
use serde::{Deserialize, Serialize};

use catalog::Catalog;
use profile::Profile;

pub mod catalog;
pub mod global;
pub mod profile;
mod utils;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Oscal {
    pub catalog: Option<Catalog>,
    pub profile: Option<Profile>,
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_basic_catalog_min() {
        let json =
            include_str!("../tests/oscal_content/examples/catalog/json/basic-catalog-min.json");
        let result = serde_json::from_str::<Oscal>(json);
        assert!(result.is_ok());
    }

    #[test]
    fn test_basic_catalog() {
        let json = include_str!("../tests/oscal_content/examples/catalog/json/basic-catalog.json");
        let result = serde_json::from_str::<Oscal>(json);
        assert!(result.is_ok());
    }
}
