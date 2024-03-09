/// File name: ../oscal_lib/src/oscal_complete_oscal_catalog/catalog.rs
/// pub use oscal_complete_oscal_catalog::*;
///
/// pub mod oscal_complete_oscal_catalog;
use serde::{Deserialize, Serialize};
use serde_with::skip_serializing_none;

use crate::{
    catalog_common::parameter::Parameter,
    metadata::{BackMatter, Metadata},
    SchemaElement, UUIDDatatype,
};

use self::{control::Control, group::Group};

pub mod control;
pub mod group;

#[skip_serializing_none]
#[derive(Debug, Clone, PartialEq, Deserialize, Serialize)]
#[serde(rename_all = "kebab-case")]
pub struct Catalog {
    pub uuid: UUIDDatatype,
    pub metadata: Metadata,
    pub params: Option<Vec<Parameter>>,
    pub controls: Option<Vec<Control>>,
    pub groups: Option<Vec<Group>>,
    pub back_matter: Option<BackMatter>,
}

impl SchemaElement for Catalog {
    fn schema_title() -> &'static str {
        "Catalog"
    }
    fn schema_description() -> &'static str {
        r#"A collection of controls."#
    }
    fn schema_id() -> Option<&'static str> {
        Some("#assembly_oscal-catalog_catalog")
    }
    fn schema_path() -> &'static str {
        "oscal-complete-oscal-catalog:catalog"
    }
}
