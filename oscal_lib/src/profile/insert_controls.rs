/// File name: ../oscal_lib/src/oscal_complete_oscal_profile/insert_controls.rs
/// pub use oscal_complete_oscal_profile::*;
///
/// pub mod oscal_complete_oscal_profile;
use serde::{Deserialize, Serialize};
use serde_with::skip_serializing_none;

use crate::{catalog_common::include_all::IncludeAll, SchemaElement, TokenDatatype};

use super::select_control_by_id::SelectControlById;

#[skip_serializing_none]
#[derive(Debug, Clone, PartialEq, Deserialize, Serialize)]
#[serde(rename_all = "kebab-case")]
pub struct InsertControls {
    ///"enum": [
    ///    "keep",
    ///    "ascending",
    ///    "descending"
    /// ]
    pub order: Option<TokenDatatype>,
    pub include_all: Option<IncludeAll>,
    pub include_controls: Option<Vec<SelectControlById>>,
    pub exclude_controls: Option<Vec<SelectControlById>>,
}

impl SchemaElement for InsertControls {
    fn schema_title() -> &'static str {
        "Select controls"
    }
    fn schema_description() -> &'static str {
        r#"Specifies which controls to use in the containing context."#
    }
    fn schema_id() -> Option<&'static str> {
        Some("#assembly_oscal-profile_insert-controls")
    }
    fn schema_path() -> &'static str {
        "oscal-complete-oscal-profile:insert-controls"
    }
}
