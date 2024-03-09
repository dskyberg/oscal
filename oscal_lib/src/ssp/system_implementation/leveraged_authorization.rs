/// File name: ../oscal_lib/src/oscal_complete_oscal_ssp/system_implementation.rs
/// pub use oscal_complete_oscal_ssp::*;
///
/// pub mod oscal_complete_oscal_ssp;
use serde::{Deserialize, Serialize};
use serde_with::skip_serializing_none;

use crate::SchemaElement;

#[skip_serializing_none]
#[derive(Debug, Clone, PartialEq, Deserialize, Serialize)]
#[serde(rename_all = "kebab-case")]
pub struct LeveragedAuthorization {}

impl SchemaElement for LeveragedAuthorization {
    fn schema_title() -> &'static str {
        "System Implementation"
    }
    fn schema_description() -> &'static str {
        r#"Provides information as to how the system is implemented."#
    }
    fn schema_id() -> Option<&'static str> {
        None
    }
    fn schema_path() -> &'static str {
        "oscal-complete-oscal-ssp:system-implementation:leveraged-authorization"
    }
}
