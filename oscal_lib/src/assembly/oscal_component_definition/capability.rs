/// Capability
/// A grouping of other components and/or capabilities.
/// $id: #assembly_oscal-component-definition_capability
use serde::{Deserialize, Serialize};
use serde_with::skip_serializing_none;

use crate::assembly::oscal_component_definition::ControlImplementation;
use crate::assembly::oscal_component_definition::IncorporatesComponent;
use crate::assembly::oscal_metadata::Link;
use crate::assembly::oscal_metadata::Property;
use crate::definitions::StringDatatype;
use crate::definitions::UuidDatatype;
use crate::field::oscal_metadata::Remarks;

#[skip_serializing_none]
#[derive(Debug, Clone, Deserialize, Serialize)]
#[serde(rename_all ="kebab-case")]
pub struct Capability {
	pub incorporates_components: Option<Vec<IncorporatesComponent>>,
	pub props: Option<Vec<Property>>,
	/// Capability Name
	/// The capability's human-readable name.
	pub name: StringDatatype,
	pub control_implementations: Option<Vec<ControlImplementation>>,
	/// Capability Description
	/// A summary of the capability.
	pub description: String,
	pub links: Option<Vec<Link>>,
	pub remarks: Option<Remarks>,
	/// Capability Identifier
	/// A machine-oriented, globally unique identifier with cross-instance scope that can be used to reference this capability elsewhere in this or other OSCAL instances. The locally defined UUID of the capability can be used to reference the data item locally or globally (e.g., in an imported OSCAL instance).This UUID should be assigned per-subject, which means it should be consistently used to identify the same subject across revisions of the document.
	pub uuid: UuidDatatype,
}
