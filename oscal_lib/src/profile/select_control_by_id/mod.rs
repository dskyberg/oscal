/// File name: ../oscal_lib/src/oscal_complete_oscal_profile/select_control_by_id.rs
/// pub use oscal_complete_oscal_profile::*;
///
/// pub mod oscal_complete_oscal_profile;
use serde::{Deserialize, Serialize};
use serde_with::skip_serializing_none;

use crate::{SchemaElement, TokenDatatype};

use self::pattern::Pattern;

pub mod pattern;

#[skip_serializing_none]
#[derive(Debug, Clone, PartialEq, Deserialize, Serialize)]
#[serde(rename_all = "kebab-case")]
pub struct SelectControlById {
    /// "enum": [
    ///    "yes",
    ///    "no"
    /// ]
    pub with_child_controls: Option<TokenDatatype>,
    pub with_ids: Option<TokenDatatype>,
    pub matching: Option<Vec<Pattern>>,
}

impl SchemaElement for SelectControlById {
    fn schema_title() -> &'static str {
        "Call"
    }
    fn schema_description() -> &'static str {
        r#"Call a control by its ID"#
    }
    fn schema_id() -> Option<&'static str> {
        Some("#assembly_oscal-profile_select-control-by-id")
    }
    fn schema_path() -> &'static str {
        "oscal-complete-oscal-profile:select-control-by-id"
    }
}
