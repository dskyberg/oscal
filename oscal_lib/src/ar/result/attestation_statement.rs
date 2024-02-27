use serde::{Deserialize, Serialize};
use serde_with::skip_serializing_none;

use crate::{
    assessment_common::assessment_part::AssessmentPart, metadata::ResponsibleParty,
    SchemaConstraint,
};

#[skip_serializing_none]
#[derive(Debug, Clone, PartialEq, Deserialize, Serialize)]
#[serde(rename_all = "kebab-case")]
pub struct AttestationStatement {
    pub responsible_parties: Option<Vec<ResponsibleParty>>,
    pub parts: Vec<AssessmentPart>,
}

impl SchemaConstraint for AttestationStatement {
    fn constraint_title() -> &'static str {
        "Attestation Statements"
    }
    fn constraint_description() -> &'static str {
        "A set of textual statements, typically written by the assessor."
    }
    fn constraint_id() -> &'static str {
        "#assembly_oscal-ar_result_attestation-statement"
    }
    fn schema_path() -> &'static str {
        "oscal-complete-oscal-ar:result:attestation"
    }
}
