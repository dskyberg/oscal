/// Assessment-Specific Control Objective
/// A local definition of a control objective for this assessment. Uses catalog syntax for control objective and assessment actions.
/// $id: #assembly_oscal-assessment-common_local-objective
use serde::{Deserialize, Serialize};
use serde_with::skip_serializing_none;

use crate::assembly::oscal_catalog_common::Part;
use crate::assembly::oscal_metadata::Link;
use crate::assembly::oscal_metadata::Property;
use crate::definitions::TokenDatatype;
use crate::field::oscal_metadata::Remarks;

#[skip_serializing_none]
#[derive(Debug, Clone, Deserialize, Serialize)]
#[serde(rename_all ="kebab-case")]
pub struct LocalObjective {
	pub parts: Vec<Part>,
	/// Objective Description
	/// A human-readable description of this control objective.
	pub description: Option<String>,
	pub links: Option<Vec<Link>>,
	/// Control Identifier Reference
	/// A human-oriented identifier reference to a control with a corresponding id value. When referencing an externally defined control, the Control Identifier Reference must be used in the context of the external / imported OSCAL instance (e.g., uri-reference).
	pub control_id: TokenDatatype,
	pub props: Option<Vec<Property>>,
	pub remarks: Option<Remarks>,
}
