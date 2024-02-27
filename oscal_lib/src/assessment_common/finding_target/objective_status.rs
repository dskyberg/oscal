use serde::{Deserialize, Serialize};
use serde_with::skip_serializing_none;

use crate::{metadata::Remarks, SchemaConstraint, TokenDatatype};

#[skip_serializing_none]
#[derive(Debug, Clone, PartialEq, Deserialize, Serialize)]
#[serde(rename_all = "kebab-case")]
pub struct ObjectiveStatus {
    /// "enum": [
    ///    "satisfied",
    ///    "not-satisfied"
    /// ]
    pub state: TokenDatatype,
    /// "enum": [
    ///    "pass",
    ///    "fail",
    ///    "other"
    /// ]
    pub reason: Option<TokenDatatype>,
    pub remarks: Option<Remarks>,
}

impl SchemaConstraint for ObjectiveStatus {
    fn constraint_title() -> &'static str {
        "Objective Status"
    }
    fn constraint_description() -> &'static str {
        "A determination of if the objective is satisfied or not within a given system."
    }
    fn constraint_id() -> &'static str {
        "#assembly_oscal-assessment-common_finding-target:status"
    }
    fn schema_path() -> &'static str {
        "oscal-complete-oscal-assessment-common:finding-target:status"
    }
}
