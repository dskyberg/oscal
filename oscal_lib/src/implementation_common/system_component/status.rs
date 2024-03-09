use serde::{Deserialize, Serialize};
use serde_with::skip_serializing_none;

use crate::{metadata::Remarks, SchemaElement, TokenDatatype};

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

impl SchemaElement for Status {
    fn schema_title() -> &'static str {
        "Status"
    }

    fn schema_description() -> &'static str {
        "Describes the operational status of the system component."
    }

    fn schema_id() -> Option<&'static str> {
        None
    }
    fn schema_path() -> &'static str {
        "oscal-complete-oscal-implementation-common:system-component:status"
    }
}
