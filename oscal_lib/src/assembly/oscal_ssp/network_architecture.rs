/// Network Architecture
/// A description of the system's network architecture, optionally supplemented by diagrams that illustrate the network architecture.
/// $id: #assembly_oscal-ssp_network-architecture
use serde::{Deserialize, Serialize};
use serde_with::skip_serializing_none;

use crate::assembly::oscal_metadata::Link;
use crate::assembly::oscal_metadata::Property;
use crate::assembly::oscal_ssp::Diagram;
use crate::field::oscal_metadata::Remarks;

#[skip_serializing_none]
#[derive(Debug, Clone, Deserialize, Serialize)]
#[serde(rename_all ="kebab-case")]
pub struct NetworkArchitecture {
	pub remarks: Option<Remarks>,
	pub diagrams: Option<Vec<Diagram>>,
	pub links: Option<Vec<Link>>,
	pub props: Option<Vec<Property>>,
	/// Network Architecture Description
	/// A summary of the system's network architecture.
	pub description: String,
}
