use serde::{Deserialize, Serialize};
use serde_with::skip_serializing_none;

use crate::{
    metadata::{Link, Property, Remarks},
    SchemaConstraint, URIReferenceDatatype,
};

#[skip_serializing_none]
#[derive(Debug, Clone, PartialEq, Deserialize, Serialize)]
#[serde(rename_all = "kebab-case")]
pub struct RelevantEvidence {
    pub href: Option<URIReferenceDatatype>,
    pub description: String,
    pub props: Option<Vec<Property>>,
    pub links: Option<Vec<Link>>,
    pub remarks: Option<Remarks>,
}
impl SchemaConstraint for RelevantEvidence {
    fn constraint_title() -> &'static str {
        "Relevant Evidence"
    }
    fn constraint_description() -> &'static str {
        "Links this observation to relevant evidence."
    }
    fn constraint_id() -> &'static str {
        "#assembly_oscal-assessment-common_observation:relevant-evidence"
    }
    fn schema_path() -> &'static str {
        "oscal-complete-oscal-assessment-common:observation:relevant-evidence"
    }
}
