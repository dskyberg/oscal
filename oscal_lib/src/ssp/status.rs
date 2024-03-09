/// File name: ../oscal_lib/src/oscal_complete_oscal_ssp/status.rs
/// pub use oscal_complete_oscal_ssp::*;
///
/// pub mod oscal_complete_oscal_ssp;
use serde::{Deserialize, Serialize};
use serde_with::skip_serializing_none;

use crate::{metadata::Remarks, SchemaElement, StringDatatype};

#[skip_serializing_none]
#[derive(Debug, Clone, PartialEq, Deserialize, Serialize)]
#[serde(rename_all = "kebab-case")]
pub struct Status {
    /// "enum": [
    ///     "operational",
    ///     "under-development",
    ///     "under-major-modification",
    ///     "disposition",
    ///     "other"
    /// ]
    pub state: StringDatatype,
    pub remarks: Option<Remarks>,
}

impl SchemaElement for Status {
    fn schema_title() -> &'static str {
        "Status"
    }
    fn schema_description() -> &'static str {
        r#"Describes the operational status of the system."#
    }
    fn schema_id() -> Option<&'static str> {
        Some("#assembly_oscal-ssp_status")
    }
    fn schema_path() -> &'static str {
        "oscal-complete-oscal-ssp:status"
    }
}
