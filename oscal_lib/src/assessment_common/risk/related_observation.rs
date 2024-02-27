use serde::{Deserialize, Serialize};

use crate::{SchemaConstraint, UUIDDatatype};

#[derive(Debug, Clone, PartialEq, Deserialize, Serialize)]
#[serde(rename_all = "kebab-case")]
pub struct RelatedObservation {
    pub observation_uuid: UUIDDatatype,
}
impl SchemaConstraint for RelatedObservation {
    fn constraint_title() -> &'static str {
        "Related Observation"
    }
    fn constraint_description() -> &'static str {
        "Relates the finding to a set of referenced observations that were used to determine the finding."
    }
    fn constraint_id() -> &'static str {
        "#assembly_oscal-assessment-common_risk:related-observation"
    }
    fn schema_path() -> &'static str {
        "oscal-complete-oscal-assessment-common:risk:related-observation"
    }
}
