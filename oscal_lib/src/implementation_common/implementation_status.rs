/// File name: ../oscal_lib/src/oscal_complete_oscal_implementation_common/implementation_status.rs
/// pub use oscal_complete_oscal_implementation_common::*;
///
/// pub mod oscal_complete_oscal_implementation_common;
use serde::{Deserialize, Serialize};
use serde_with::skip_serializing_none;

use crate::{metadata::Remarks, SchemaElement, TokenDatatype};

#[skip_serializing_none]
#[derive(Debug, Clone, PartialEq, Deserialize, Serialize)]
#[serde(rename_all = "kebab-case")]
pub struct ImplementationStatus {
    /// "enum": [
    ///     "implemented",
    ///     "partial",
    ///     "planned",
    ///  "alternative",
    ///     "not-applicable"
    /// ]
    pub state: TokenDatatype,
    pub remarks: Option<Remarks>,
}

impl SchemaElement for ImplementationStatus {
    fn schema_title() -> &'static str {
        "Implementation Status"
    }
    fn schema_description() -> &'static str {
        r#"Indicates the degree to which the a given control is implemented."#
    }
    fn schema_id() -> Option<&'static str> {
        Some("#assembly_oscal-implementation-common_implementation-status")
    }
    fn schema_path() -> &'static str {
        "oscal-complete-oscal-implementation-common:implementation-status"
    }
}
