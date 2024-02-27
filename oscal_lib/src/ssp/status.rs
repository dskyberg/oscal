/// File name: ../oscal_lib/src/oscal_complete_oscal_ssp/status.rs
/// pub use oscal_complete_oscal_ssp::*;
///
/// pub mod oscal_complete_oscal_ssp;
use serde::{Deserialize, Serialize};
use serde_with::skip_serializing_none;

use crate::SchemaConstraint;

#[skip_serializing_none]
#[derive(Debug, Clone, PartialEq, Deserialize, Serialize)]
#[serde(rename_all = "kebab-case")]
pub struct Status {
}

impl SchemaConstraint for Status {
    fn constraint_title() -> &'static str {
        "Status"
    }
    fn constraint_description() -> &'static str {
        r#"Describes the operational status of the system."#
    }
    fn constraint_id() -> &'static str {
        "#assembly_oscal-ssp_status"
    }
    fn schema_path() -> &'static str {
        "oscal-complete-oscal-ssp:status"
    }
}
