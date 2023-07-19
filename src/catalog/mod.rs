/// A collection of controls.
///
/// Catalogs may use one or more group objects to subdivide the control contents of a catalog.

/// An OSCAL catalog model provides a structured representation of control information.
use serde::{Deserialize, Serialize};
use serde_with::skip_serializing_none;
use uuid::Uuid;

use crate::global::{BackMatter, ControlGroups, Controls, Metadata, Parameters};

// Provides information about the publication and availability of the containing document.
#[skip_serializing_none]
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Catalog {
    pub uuid: Uuid,
    pub metadata: Metadata,
    pub params: Option<Parameters>,
    pub controls: Option<Controls>,
    pub groups: Option<ControlGroups>,
    #[serde(rename = "back-matter")]
    pub back_matter: Option<BackMatter>,
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_it() {
        let json = include_str!("../../tests/catalog_1.json");

        let result = serde_json::from_str::<Catalog>(json);
        assert!(result.is_ok());
    }
}
