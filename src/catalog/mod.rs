use serde::{Deserialize, Serialize};
use serde_with::skip_serializing_none;
use uuid::Uuid;

use back_matter::BackMatter;
use control::Controls;
use group::Groups;
use metadata::Metadata;

use crate::global::Params;

pub mod back_matter;
pub mod control;
pub mod group;
pub mod metadata;
pub mod part;

// Provides information about the publication and availability of the containing document.
#[skip_serializing_none]
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Catalog {
    pub uuid: Uuid,
    pub metadata: Metadata,
    pub params: Option<Params>,
    pub controls: Option<Controls>,
    pub groups: Option<Groups>,
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
