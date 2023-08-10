pub use status::*;
pub use component_type::*;


pub mod status;
pub mod component_type;

/// Component
/// A defined component that can be part of an implemented system.
/// $id: #assembly_oscal-implementation-common_system-component
use serde::{Deserialize, Serialize};
use serde_with::skip_serializing_none;

use crate::assembly::oscal_implementation_common::Protocol;
use crate::assembly::oscal_metadata::Link;
use crate::assembly::oscal_metadata::Property;
use crate::assembly::oscal_metadata::ResponsibleRole;
use crate::definitions::UuidDatatype;
use crate::field::oscal_metadata::Remarks;

#[skip_serializing_none]
#[derive(Debug, Clone, Deserialize, Serialize)]
#[serde(rename_all ="kebab-case")]
pub struct SystemComponent {
	/// Status
	/// Describes the operational status of the system component.
	pub status: Status,
	pub props: Option<Vec<Property>>,
	pub responsible_roles: Option<Vec<ResponsibleRole>>,
	/// Component Type
	/// A category describing the purpose of the component.
	#[serde(rename = "type")]
	pub _type: ComponentType,
	pub protocols: Option<Vec<Protocol>>,
	/// Component Description
	/// A description of the component, including information about its function.
	pub description: String,
	/// Purpose
	/// A summary of the technological or business purpose of the component.
	pub purpose: Option<String>,
	pub links: Option<Vec<Link>>,
	pub remarks: Option<Remarks>,
	/// Component Title
	/// A human readable name for the system component.
	pub title: String,
	/// Component Identifier
	/// A machine-oriented, globally unique identifier with cross-instance scope that can be used to reference this component elsewhere in this or other OSCAL instances. The locally defined UUID of the component can be used to reference the data item locally or globally (e.g., in an imported OSCAL instance). This UUID should be assigned per-subject, which means it should be consistently used to identify the same subject across revisions of the document.
	pub uuid: UuidDatatype,
}
