use serde::{Deserialize, Serialize};
use serde_with::skip_serializing_none;

use crate::{assessment_common::origin_actor::OriginActor, SchemaElement};

#[skip_serializing_none]
#[derive(Debug, Clone, PartialEq, Deserialize, Serialize)]
#[serde(rename_all = "kebab-case")]
pub struct Origin {
    pub actors: Vec<OriginActor>,
}

impl SchemaElement for Origin {
    fn schema_title() -> &'static str {
        "Origin"
    }

    fn schema_description() -> &'static str {
        "Identifies the source of the finding, such as a tool or person."
    }
    fn schema_id() -> Option<&'static str> {
        None
    }
    fn schema_path() -> &'static str {
        "oscal-complete-oscal-poam:poam-item:origin"
    }
}
