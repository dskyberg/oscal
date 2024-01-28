/// File name: ../oscal_lib/src/oscal_complete_oscal_catalog/control.rs
/// pub use oscal_complete_oscal_catalog::*;
///
/// pub mod oscal_complete_oscal_catalog;
use serde::{Deserialize, Serialize};
use serde_with::skip_serializing_none;

use crate::SchemaConstraint;

#[skip_serializing_none]
#[derive(Debug, Clone, PartialEq, Deserialize, Serialize)]
#[serde(rename_all = "kebab-case")]
pub struct Control {
}

impl SchemaConstraint for Control {
    fn constraint_title() -> &'static str {
        "Control"
    }
    fn constraint_description() -> &'static str {
        r#"A structured information object representing a security or privacy control. Each security or privacy control within the Catalog is defined by a distinct control instance."#
    }
    fn constraint_id() -> &'static str {
        "#assembly_oscal-catalog_control"
    }
    fn schema_path() -> &'static str {
        "oscal-complete-oscal-catalog:control"
    }
}
