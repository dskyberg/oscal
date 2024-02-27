use serde::{Deserialize, Serialize};
use serde_with::skip_serializing_none;

use crate::{SchemaConstraint, StringDatatype};

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

impl SchemaConstraint for Method {
    fn constraint_title() -> &'static str {
        "Combination method"
    }
    fn constraint_description() -> &'static str {
        "How clashing controls should be handled"
    }
    fn constraint_id() -> &'static str {
        "#assembly_oscal-profile_merge_method"
    }
    fn schema_path() -> &'static str {
        "oscal-complete-oscal-profile:merge:method"
    }
}
