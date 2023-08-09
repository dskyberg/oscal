/// Data Flow
/// A description of the logical flow of information within the system and across its boundaries, optionally supplemented by diagrams that illustrate these flows.
/// $id: #assembly_oscal-ssp_data-flow
use serde::{Deserialize, Serialize};
use serde_with::skip_serializing_none;

use crate::assembly::oscal_metadata::Link;
use crate::assembly::oscal_metadata::Property;
use crate::assembly::oscal_ssp::Diagram;
use crate::field::oscal_metadata::Remarks;

#[skip_serializing_none]
#[derive(Debug, Clone, Deserialize, Serialize)]
#[serde(rename_all ="kebab-case")]
pub struct DataFlow {
	pub diagrams: Option<Vec<Diagram>>,
	pub props: Option<Vec<Property>>,
	pub remarks: Option<Remarks>,
	pub links: Option<Vec<Link>>,
	/// Data Flow Description
	/// A summary of the system's data flow.
	pub description: String,
}
