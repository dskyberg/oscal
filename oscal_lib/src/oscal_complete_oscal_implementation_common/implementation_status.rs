/// File name: ../oscal_lib/src/oscal_complete_oscal_implementation_common/implementation_status.rs
/// pub use oscal_complete_oscal_implementation_common::*;
///
/// pub mod oscal_complete_oscal_implementation_common;
use serde::{Deserialize, Serialize};
use serde_with::skip_serializing_none;

use crate::SchemaConstraint;

#[skip_serializing_none]
#[derive(Debug, Clone, PartialEq, Deserialize, Serialize)]
#[serde(rename_all = "kebab-case")]
pub struct ImplementationStatus {
}

impl SchemaConstraint for ImplementationStatus {
    fn constraint_title() -> &'static str {
        "Implementation Status"
    }
    fn constraint_description() -> &'static str {
        r#"Indicates the degree to which the a given control is implemented."#
    }
    fn constraint_id() -> &'static str {
        "#assembly_oscal-implementation-common_implementation-status"
    }
    fn schema_path() -> &'static str {
        "oscal-complete-oscal-implementation-common:implementation-status"
    }
}
