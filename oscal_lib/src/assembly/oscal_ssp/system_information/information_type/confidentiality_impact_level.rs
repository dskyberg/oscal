/// Confidentiality Impact Level
/// The expected level of impact resulting from the unauthorized disclosure of the described information.
/// $id: #assembly_oscal-ssp_system-information_information-type_information-type_confidentiality-impact-level_confidentiality-impact-level
use serde::{Deserialize, Serialize};
use serde_with::skip_serializing_none;

use crate::assembly::oscal_metadata::Link;
use crate::assembly::oscal_metadata::Property;
use crate::field::oscal_ssp::AdjustmentJustification;
use crate::field::oscal_ssp::Base;
use crate::field::oscal_ssp::Selected;

#[skip_serializing_none]
#[derive(Debug, Clone, Deserialize, Serialize)]
#[serde(rename_all ="kebab-case")]
pub struct ConfidentialityImpactLevel {
	pub base: Base,
	pub links: Option<Vec<Link>>,
	pub props: Option<Vec<Property>>,
	pub selected: Option<Selected>,
	pub adjustment_justification: Option<AdjustmentJustification>,
}
