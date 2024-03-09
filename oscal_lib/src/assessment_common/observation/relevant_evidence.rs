use serde::{Deserialize, Serialize};
use serde_with::skip_serializing_none;

use crate::{
    metadata::{Link, Property, Remarks},
    SchemaElement, URIReferenceDatatype,
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
impl SchemaElement for RelevantEvidence {
    fn schema_title() -> &'static str {
        "Relevant Evidence"
    }
    fn schema_description() -> &'static str {
        "Links this observation to relevant evidence."
    }
    fn schema_id() -> Option<&'static str> {
        None
    }
    fn schema_path() -> &'static str {
        "oscal-complete-oscal-assessment-common:observation:relevant-evidence"
    }
}
