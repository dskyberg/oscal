/// File name: ../oscal_lib/src/oscal_complete_oscal_implementation_common/set_parameter.rs
/// pub use oscal_complete_oscal_implementation_common::*;
///
/// pub mod oscal_complete_oscal_implementation_common;
use serde::{Deserialize, Serialize};
use serde_with::skip_serializing_none;

use crate::SchemaConstraint;

#[skip_serializing_none]
#[derive(Debug, Clone, PartialEq, Deserialize, Serialize)]
#[serde(rename_all = "kebab-case")]
pub struct SetParameter {
}

impl SchemaConstraint for SetParameter {
    fn constraint_title() -> &'static str {
        "Set Parameter Value"
    }
    fn constraint_description() -> &'static str {
        r#"Identifies the parameter that will be set by the enclosed value."#
    }
    fn constraint_id() -> &'static str {
        "#assembly_oscal-implementation-common_set-parameter"
    }
    fn schema_path() -> &'static str {
        "oscal-complete-oscal-implementation-common:set-parameter"
    }
}
