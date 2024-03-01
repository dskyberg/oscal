/// File name: ../oscal_lib/src/oscal_complete_oscal_ssp/control_implementation.rs
/// pub use oscal_complete_oscal_ssp::*;
///
/// pub mod oscal_complete_oscal_ssp;
use serde::{Deserialize, Serialize};
use serde_with::skip_serializing_none;

use crate::{implementation_common::set_parameter::SetParameter, SchemaConstraint};

use super::implemented_requirement::ImplementedRequirement;

#[skip_serializing_none]
#[derive(Debug, Clone, PartialEq, Deserialize, Serialize)]
#[serde(rename_all = "kebab-case")]
pub struct ControlImplementation {
    pub description: String,
    pub set_parameters: Option<Vec<SetParameter>>,
    pub implemented_requirements: Vec<ImplementedRequirement>,
}

impl SchemaConstraint for ControlImplementation {
    fn constraint_title() -> &'static str {
        "Control Implementation"
    }
    fn constraint_description() -> &'static str {
        r#"Describes how the system satisfies a set of controls."#
    }
    fn constraint_id() -> &'static str {
        "#assembly_oscal-ssp_control-implementation"
    }
    fn schema_path() -> &'static str {
        "oscal-complete-oscal-ssp:control-implementation"
    }
}
