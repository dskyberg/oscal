use serde::{Deserialize, Serialize};
use serde_with::skip_serializing_none;

use crate::{metadata::Remarks, SchemaConstraint, TokenDatatype};

#[skip_serializing_none]
#[derive(Debug, Clone, PartialEq, Deserialize, Serialize)]
#[serde(rename_all = "kebab-case")]
pub struct Status {
    /// "enum": [
    ///     "under-development",
    ///     "operational",
    ///     "disposition",
    ///     "other"
    /// ]
    state: TokenDatatype,
    remarks: Option<Remarks>,
}

impl SchemaConstraint for Status {
    fn constraint_title() -> &'static str {
        "Status"
    }

    fn constraint_description() -> &'static str {
        "Describes the operational status of the system component."
    }

    fn constraint_id() -> &'static str {
        "#assembly_oscal-implementation-common_system-component:status"
    }
    fn schema_path() -> &'static str {
        "oscal-complete-oscal-implementation-common:system-component:status"
    }
}
