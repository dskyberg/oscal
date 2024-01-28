/// File name: ../oscal_lib/src/oscal_complete_oscal_ssp/implemented_requirement.rs
/// pub use oscal_complete_oscal_ssp::*;
///
/// pub mod oscal_complete_oscal_ssp;
use serde::{Deserialize, Serialize};
use serde_with::skip_serializing_none;

use crate::SchemaConstraint;

#[skip_serializing_none]
#[derive(Debug, Clone, PartialEq, Deserialize, Serialize)]
#[serde(rename_all = "kebab-case")]
pub struct ImplementedRequirement {
}

impl SchemaConstraint for ImplementedRequirement {
    fn constraint_title() -> &'static str {
        "Control-based Requirement"
    }
    fn constraint_description() -> &'static str {
        r#"Describes how the system satisfies the requirements of an individual control."#
    }
    fn constraint_id() -> &'static str {
        "#assembly_oscal-ssp_implemented-requirement"
    }
    fn schema_path() -> &'static str {
        "oscal-complete-oscal-ssp:implemented-requirement"
    }
}
