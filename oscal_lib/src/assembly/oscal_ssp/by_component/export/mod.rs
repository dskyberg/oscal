pub use provided_control_implementation::*;
pub use control_implementation_responsibility::*;


pub mod provided_control_implementation;
pub mod control_implementation_responsibility;

/// Export
/// Identifies content intended for external consumption, such as with leveraged organizations.
/// $id: #assembly_oscal-ssp_by-component_export_export
use serde::{Deserialize, Serialize};
use serde_with::skip_serializing_none;

use crate::assembly::oscal_metadata::Link;
use crate::assembly::oscal_metadata::Property;
use crate::field::oscal_metadata::Remarks;

#[skip_serializing_none]
#[derive(Debug, Clone, Deserialize, Serialize)]
#[serde(rename_all ="kebab-case")]
pub struct Export {
	pub provided: Option<Vec<ProvidedControlImplementation>>,
	pub remarks: Option<Remarks>,
	pub responsibilities: Option<Vec<ControlImplementationResponsibility>>,
	pub links: Option<Vec<Link>>,
	pub props: Option<Vec<Property>>,
	/// Control Implementation Export Description
	/// An implementation statement that describes the aspects of the control or control statement implementation that can be available to another system leveraging this system.
	pub description: Option<String>,
}
