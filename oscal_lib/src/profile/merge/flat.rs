use serde::{Deserialize, Serialize};
use serde_with::skip_serializing_none;

use crate::SchemaConstraint;

#[skip_serializing_none]
#[derive(Debug, Clone, PartialEq, Deserialize, Serialize)]
pub struct Flat {}

impl SchemaConstraint for Flat {
    fn constraint_title() -> &'static str {
        "Flat"
    }
    fn constraint_description() -> &'static str {
        "Use the flat structuring method."
    }
    fn constraint_id() -> &'static str {
        "#assembly_oscal-profile_merge_flat"
    }
    fn schema_path() -> &'static str {
        "oscal-complete-oscal-profile:merge:flat"
    }
}
