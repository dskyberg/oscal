use serde::{Deserialize, Serialize};
use serde_with::skip_serializing_none;

use crate::{SchemaElement, StringDatatype};

#[skip_serializing_none]
#[derive(Debug, Clone, PartialEq, Deserialize, Serialize)]
pub struct Method {
    /// "enum": [
    ///    "use-first",
    ///    "merge",
    ///    "keep"
    /// ]
    pub combine: Option<StringDatatype>,
}

impl SchemaElement for Method {
    fn schema_title() -> &'static str {
        "Combination method"
    }
    fn schema_description() -> &'static str {
        "How clashing controls should be handled"
    }
    fn schema_id() -> Option<&'static str> {
        None
    }
    fn schema_path() -> &'static str {
        "oscal-complete-oscal-profile:merge:method"
    }
}
