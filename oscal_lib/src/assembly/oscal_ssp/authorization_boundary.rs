/// Authorization Boundary
/// A description of this system's authorization boundary, optionally supplemented by diagrams that illustrate the authorization boundary.
/// $id: #assembly_oscal-ssp_authorization-boundary
use serde::{Deserialize, Serialize};
use serde_with::skip_serializing_none;

use crate::assembly::oscal_metadata::Link;
use crate::assembly::oscal_metadata::Property;
use crate::assembly::oscal_ssp::Diagram;
use crate::field::oscal_metadata::Remarks;

#[skip_serializing_none]
#[derive(Debug, Clone, Deserialize, Serialize)]
#[serde(rename_all ="kebab-case")]
pub struct AuthorizationBoundary {
	pub props: Option<Vec<Property>>,
	pub links: Option<Vec<Link>>,
	pub diagrams: Option<Vec<Diagram>>,
	/// Authorization Boundary Description
	/// A summary of the system's authorization boundary.
	pub description: String,
	pub remarks: Option<Remarks>,
}
