/// Attestation Statements
/// A set of textual statements, typically written by the assessor.
/// $id: #assembly_oscal-ar_result_attestation-statements_attestation-statements
use serde::{Deserialize, Serialize};
use serde_with::skip_serializing_none;

use crate::assembly::oscal_assessment_common::AssessmentPart;
use crate::assembly::oscal_metadata::ResponsibleParty;

#[skip_serializing_none]
#[derive(Debug, Clone, Deserialize, Serialize)]
#[serde(rename_all ="kebab-case")]
pub struct AttestationStatements {
	pub responsible_parties: Option<Vec<ResponsibleParty>>,
	pub parts: Vec<AssessmentPart>,
}
