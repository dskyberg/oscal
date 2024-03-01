/// File name: ../oscal_lib/src/oscal_complete_oscal_ssp/system_implementation.rs
/// pub use oscal_complete_oscal_ssp::*;
///
/// pub mod oscal_complete_oscal_ssp;
use serde::{Deserialize, Serialize};
use serde_with::skip_serializing_none;

use crate::SchemaConstraint;

#[skip_serializing_none]
#[derive(Debug, Clone, PartialEq, Deserialize, Serialize)]
#[serde(rename_all = "kebab-case")]
pub struct LeveragedAuthorization {}

impl SchemaConstraint for LeveragedAuthorization {
    fn constraint_title() -> &'static str {
        "System Implementation"
    }
    fn constraint_description() -> &'static str {
        r#"Provides information as to how the system is implemented."#
    }
    fn constraint_id() -> &'static str {
        "#assembly_oscal-ssp_system-implementation_leveraged-authorization"
    }
    fn schema_path() -> &'static str {
        "oscal-complete-oscal-ssp:system-implementation:leveraged-authorization"
    }
}
