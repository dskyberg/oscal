/// File name: ../oscal_lib/src/oscal_complete_oscal_catalog_common/parameter_selection.rs
/// pub use oscal_complete_oscal_catalog_common::*;
///
/// pub mod oscal_complete_oscal_catalog_common;
use serde::{Deserialize, Serialize};
use serde_with::skip_serializing_none;

use crate::SchemaConstraint;

#[skip_serializing_none]
#[derive(Debug, Clone, PartialEq, Deserialize, Serialize)]
#[serde(rename_all = "kebab-case")]
pub struct ParameterSelection {
}

impl SchemaConstraint for ParameterSelection {
    fn constraint_title() -> &'static str {
        "Selection"
    }
    fn constraint_description() -> &'static str {
        r#"Presenting a choice among alternatives"#
    }
    fn constraint_id() -> &'static str {
        "#assembly_oscal-catalog-common_parameter-selection"
    }
    fn schema_path() -> &'static str {
        "oscal-complete-oscal-catalog-common:parameter-selection"
    }
}
