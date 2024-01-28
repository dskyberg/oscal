/// File name: ../oscal_lib/src/oscal_complete_oscal_ssp/data_flow.rs
/// pub use oscal_complete_oscal_ssp::*;
///
/// pub mod oscal_complete_oscal_ssp;
use serde::{Deserialize, Serialize};
use serde_with::skip_serializing_none;

use crate::SchemaConstraint;

#[skip_serializing_none]
#[derive(Debug, Clone, PartialEq, Deserialize, Serialize)]
#[serde(rename_all = "kebab-case")]
pub struct DataFlow {
}

impl SchemaConstraint for DataFlow {
    fn constraint_title() -> &'static str {
        "Data Flow"
    }
    fn constraint_description() -> &'static str {
        r#"A description of the logical flow of information within the system and across its boundaries, optionally supplemented by diagrams that illustrate these flows."#
    }
    fn constraint_id() -> &'static str {
        "#assembly_oscal-ssp_data-flow"
    }
    fn schema_path() -> &'static str {
        "oscal-complete-oscal-ssp:data-flow"
    }
}
