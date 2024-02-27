use serde::{Deserialize, Serialize};
use serde_with::skip_serializing_none;

use crate::{
    profile::{group::Group, insert_controls::InsertControls},
    SchemaConstraint,
};

#[skip_serializing_none]
#[derive(Debug, Clone, PartialEq, Deserialize, Serialize)]
pub struct CustomGrouping {
    pub groups: Option<Vec<Group>>,
    pub insert_controls: Option<Vec<InsertControls>>,
}

impl SchemaConstraint for CustomGrouping {
    fn constraint_title() -> &'static str {
        "Custom grouping"
    }
    fn constraint_description() -> &'static str {
        "A Custom element frames a structure for embedding represented controls in resolution."
    }
    fn constraint_id() -> &'static str {
        "#assembly_oscal-profile_merge_custom-grouping"
    }
    fn schema_path() -> &'static str {
        "oscal-complete-oscal-profile:merge:custom-grouping"
    }
}
