use serde::{Deserialize, Serialize};
use serde_with::skip_serializing_none;

use crate::{SchemaElement, UUIDDatatype};

#[skip_serializing_none]
#[derive(Debug, Clone, PartialEq, Deserialize, Serialize)]
#[serde(rename_all = "kebab-case")]
pub struct RelatedObservation {
    observation_uuid: UUIDDatatype,
}

impl SchemaElement for RelatedObservation {
    fn schema_title() -> &'static str {
        "Related Observation"
    }

    fn schema_description() -> &'static str {
        "Relates the poam-item to a set of referenced observations that were used to determine the finding."
    }
    fn schema_id() -> Option<&'static str> {
        None
    }
    fn schema_path() -> &'static str {
        "oscal-complete-oscal-poam:poam-item:related-observation"
    }
}
