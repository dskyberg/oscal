use serde::{Deserialize, Serialize};
use serde_with::skip_serializing_none;

use crate::{
    profile::{group::Group, insert_controls::InsertControls},
    SchemaElement,
};

#[skip_serializing_none]
#[derive(Debug, Clone, PartialEq, Deserialize, Serialize)]
pub struct CustomGrouping {
    pub groups: Option<Vec<Group>>,
    pub insert_controls: Option<Vec<InsertControls>>,
}

impl SchemaElement for CustomGrouping {
    fn schema_title() -> &'static str {
        "Custom grouping"
    }
    fn schema_description() -> &'static str {
        "A Custom element frames a structure for embedding represented controls in resolution."
    }
    fn schema_id() -> Option<&'static str> {
        None
    }
    fn schema_path() -> &'static str {
        "oscal-complete-oscal-profile:merge:custom-grouping"
    }
}
