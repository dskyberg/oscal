pub use export::*;
pub use inherited_control_implementation::*;
pub use satisfied_control_implementation_responsibility::*;


pub mod export;
pub mod inherited_control_implementation;
pub mod satisfied_control_implementation_responsibility;

/// Component Control Implementation
/// Defines how the referenced component implements a set of controls.
/// $id: #assembly_oscal-ssp_by-component
use serde::{Deserialize, Serialize};
use serde_with::skip_serializing_none;

use crate::assembly::oscal_implementation_common::ImplementationStatus;
use crate::assembly::oscal_implementation_common::SetParameter;
use crate::assembly::oscal_metadata::Link;
use crate::assembly::oscal_metadata::Property;
use crate::assembly::oscal_metadata::ResponsibleRole;
use crate::definitions::UuidDatatype;
use crate::field::oscal_metadata::Remarks;

#[skip_serializing_none]
#[derive(Debug, Clone, Deserialize, Serialize)]
#[serde(rename_all ="kebab-case")]
pub struct ByComponent {
	pub remarks: Option<Remarks>,
	pub props: Option<Vec<Property>>,
	pub inherited: Option<Vec<InheritedControlImplementation>>,
	pub satisfied: Option<Vec<SatisfiedControlImplementationResponsibility>>,
	pub implementation_status: Option<ImplementationStatus>,
	pub set_parameters: Option<Vec<SetParameter>>,
	pub links: Option<Vec<Link>>,
	/// Export
	/// Identifies content intended for external consumption, such as with leveraged organizations.
	pub export: Option<Export>,
	/// Control Implementation Description
	/// An implementation statement that describes how a control or a control statement is implemented within the referenced system component.
	pub description: String,
	/// By-Component Universally Unique Identifier
	/// A machine-oriented, globally unique identifier with cross-instance scope that can be used to reference this by-component entry elsewhere in this or other OSCAL instances. The locally defined UUID of the by-component entry can be used to reference the data item locally or globally (e.g., in an imported OSCAL instance). This UUID should be assigned per-subject, which means it should be consistently used to identify the same subject across revisions of the document.
	pub uuid: UuidDatatype,
	/// Component Universally Unique Identifier Reference
	/// A machine-oriented identifier reference to the component that is implemeting a given control.
	pub component_uuid: UuidDatatype,
	pub responsible_roles: Option<Vec<ResponsibleRole>>,
}
