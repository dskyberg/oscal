/// File name: ../oscal_lib/src/oscal_complete_oscal_assessment_common/logged_by.rs
/// pub use oscal_complete_oscal_assessment_common::*;
///
/// pub mod oscal_complete_oscal_assessment_common;
use serde::{Deserialize, Serialize};
use serde_with::skip_serializing_none;

use crate::{SchemaConstraint, TokenDatatype, UUIDDatatype};

#[skip_serializing_none]
#[derive(Debug, Clone, PartialEq, Deserialize, Serialize)]
#[serde(rename_all = "kebab-case")]
pub struct LoggedBy {
    pub party_uuid: UUIDDatatype,
    pub role_id: Option<TokenDatatype>,
}

impl SchemaConstraint for LoggedBy {
    fn constraint_title() -> &'static str {
        "Logged By"
    }
    fn constraint_description() -> &'static str {
        r#"Used to indicate who created a log entry in what role."#
    }
    fn constraint_id() -> &'static str {
        "#assembly_oscal-assessment-common_logged-by"
    }
    fn schema_path() -> &'static str {
        "oscal-complete-oscal-assessment-common:logged-by"
    }
}
