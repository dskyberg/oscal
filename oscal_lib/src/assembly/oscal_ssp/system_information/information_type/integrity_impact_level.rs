/// Integrity Impact Level
/// The expected level of impact resulting from the unauthorized modification of the described information.
/// $id: #assembly_oscal-ssp_system-information_information-type_information-type_integrity-impact-level_integrity-impact-level
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
pub struct IntegrityImpactLevel {
	pub selected: Option<Selected>,
	pub props: Option<Vec<Property>>,
	pub base: Base,
	pub adjustment_justification: Option<AdjustmentJustification>,
	pub links: Option<Vec<Link>>,
}
