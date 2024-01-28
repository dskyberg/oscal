/// File name: ../oscal_lib/src/oscal_complete_oscal_catalog_common/parameter.rs
/// pub use oscal_complete_oscal_catalog_common::*;
///
/// pub mod oscal_complete_oscal_catalog_common;
use serde::{Deserialize, Serialize};
use serde_with::skip_serializing_none;

use crate::SchemaConstraint;

#[skip_serializing_none]
#[derive(Debug, Clone, PartialEq, Deserialize, Serialize)]
#[serde(rename_all = "kebab-case")]
pub struct Parameter {
}

impl SchemaConstraint for Parameter {
    fn constraint_title() -> &'static str {
        "Parameter"
    }
    fn constraint_description() -> &'static str {
        r#"Parameters provide a mechanism for the dynamic assignment of value(s) in a control."#
    }
    fn constraint_id() -> &'static str {
        "#assembly_oscal-catalog-common_parameter"
    }
    fn schema_path() -> &'static str {
        "oscal-complete-oscal-catalog-common:parameter"
    }
}
