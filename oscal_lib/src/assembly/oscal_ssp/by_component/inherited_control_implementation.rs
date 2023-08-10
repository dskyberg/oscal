/// Inherited Control Implementation
/// Describes a control implementation inherited by a leveraging system.
/// $id: #assembly_oscal-ssp_by-component_inherited-control-implementation_inherited-control-implementation
use serde::{Deserialize, Serialize};
use serde_with::skip_serializing_none;

use crate::assembly::oscal_metadata::Link;
use crate::assembly::oscal_metadata::Property;
use crate::assembly::oscal_metadata::ResponsibleRole;
use crate::definitions::UuidDatatype;

#[skip_serializing_none]
#[derive(Debug, Clone, Deserialize, Serialize)]
#[serde(rename_all ="kebab-case")]
pub struct InheritedControlImplementation {
	pub links: Option<Vec<Link>>,
	pub props: Option<Vec<Property>>,
	/// Provided UUID
	/// A machine-oriented identifier reference to an inherited control implementation that a leveraging system is inheriting from a leveraged system.
	pub provided_uuid: Option<UuidDatatype>,
	pub responsible_roles: Option<Vec<ResponsibleRole>>,
	/// Inherited Universally Unique Identifier
	/// A machine-oriented, globally unique identifier with cross-instance scope that can be used to reference this inherited entry elsewhere in this or other OSCAL instances. The locally defined UUID of the inherited control implementation can be used to reference the data item locally or globally (e.g., in an imported OSCAL instance). This UUID should be assigned per-subject, which means it should be consistently used to identify the same subject across revisions of the document.
	pub uuid: UuidDatatype,
	/// Inherited Control Implementation Description
	/// An implementation statement that describes the aspects of a control or control statement implementation that a leveraging system is inheriting from a leveraged system.
	pub description: String,
}
