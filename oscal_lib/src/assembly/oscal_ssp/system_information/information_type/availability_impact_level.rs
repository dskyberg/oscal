/// Availability Impact Level
/// The expected level of impact resulting from the disruption of access to or use of the described information or the information system.
/// $id: #assembly_oscal-ssp_system-information_information-type_information-type_availability-impact-level_availability-impact-level
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
pub struct AvailabilityImpactLevel {
	pub links: Option<Vec<Link>>,
	pub selected: Option<Selected>,
	pub base: Base,
	pub adjustment_justification: Option<AdjustmentJustification>,
	pub props: Option<Vec<Property>>,
}
