use serde::{Deserialize, Serialize};
use serde_with::skip_serializing_none;

use crate::{
    assessment_common::assessment_part::AssessmentPart, metadata::ResponsibleParty, SchemaElement,
};

#[skip_serializing_none]
#[derive(Debug, Clone, PartialEq, Deserialize, Serialize)]
#[serde(rename_all = "kebab-case")]
pub struct AttestationStatement {
    pub responsible_parties: Option<Vec<ResponsibleParty>>,
    pub parts: Vec<AssessmentPart>,
}

impl SchemaElement for AttestationStatement {
    fn schema_title() -> &'static str {
        "Attestation Statements"
    }
    fn schema_description() -> &'static str {
        "A set of textual statements, typically written by the assessor."
    }
    fn schema_id() -> Option<&'static str> {
        None
    }
    fn schema_path() -> &'static str {
        "#/definitions/oscal-complete-oscal-ar:result/attestation"
    }
}
