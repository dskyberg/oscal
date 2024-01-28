/// File name: ../oscal_lib/src/oscal_complete_oscal_catalog/catalog.rs
/// pub use oscal_complete_oscal_catalog::*;
///
/// pub mod oscal_complete_oscal_catalog;
use serde::{Deserialize, Serialize};
use serde_with::skip_serializing_none;

use crate::SchemaConstraint;

#[skip_serializing_none]
#[derive(Debug, Clone, PartialEq, Deserialize, Serialize)]
#[serde(rename_all = "kebab-case")]
pub struct Catalog {
}

impl SchemaConstraint for Catalog {
    fn constraint_title() -> &'static str {
        "Catalog"
    }
    fn constraint_description() -> &'static str {
        r#"A collection of controls."#
    }
    fn constraint_id() -> &'static str {
        "#assembly_oscal-catalog_catalog"
    }
    fn schema_path() -> &'static str {
        "oscal-complete-oscal-catalog:catalog"
    }
}
