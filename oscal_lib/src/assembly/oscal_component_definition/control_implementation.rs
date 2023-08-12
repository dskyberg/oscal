/// Control Implementation Set
/// Defines how the component or capability supports a set of controls.
/// $id: #assembly_oscal-component-definition_control-implementation
use serde::{Deserialize, Serialize};
use serde_with::skip_serializing_none;

use crate::assembly::oscal_component_definition::ImplementedRequirement;
use crate::assembly::oscal_implementation_common::SetParameter;
use crate::assembly::oscal_metadata::Link;
use crate::assembly::oscal_metadata::Property;
use crate::definitions::UriReferenceDatatype;
use crate::definitions::UuidDatatype;

#[skip_serializing_none]
#[derive(Debug, Clone, Deserialize, Serialize)]
#[serde(rename_all ="kebab-case")]
pub struct ControlImplementation {
	pub links: Option<Vec<Link>>,
	pub implemented_requirements: Vec<ImplementedRequirement>,
	pub props: Option<Vec<Property>>,
	/// Control Implementation Description
	/// A description of how the specified set of controls are implemented for the containing component or capability.
	pub description: String,
	pub set_parameters: Option<Vec<SetParameter>>,
	/// Control Implementation Set Identifier
	/// A machine-oriented, globally unique identifier with cross-instance scope that can be used to reference a set of implemented controls elsewhere in this or other OSCAL instances. The locally defined UUID of the control implementation set can be used to reference the data item locally or globally (e.g., in an imported OSCAL instance). This UUID should be assigned per-subject, which means it should be consistently used to identify the same subject across revisions of the document.
	pub uuid: UuidDatatype,
	/// Source Resource Reference
	/// A reference to an OSCAL catalog or profile providing the referenced control or subcontrol definition.
	pub source: UriReferenceDatatype,
}
