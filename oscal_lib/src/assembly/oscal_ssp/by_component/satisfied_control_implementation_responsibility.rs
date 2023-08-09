/// Satisfied Control Implementation Responsibility
/// Describes how this system satisfies a responsibility imposed by a leveraged system.
/// $id: #assembly_oscal-ssp_by-component_satisfied-control-implementation-responsibility_satisfied-control-implementation-responsibility
use serde::{Deserialize, Serialize};
use serde_with::skip_serializing_none;

use crate::assembly::oscal_metadata::Link;
use crate::assembly::oscal_metadata::Property;
use crate::assembly::oscal_metadata::ResponsibleRole;
use crate::definitions::UuidDatatype;
use crate::field::oscal_metadata::Remarks;

#[skip_serializing_none]
#[derive(Debug, Clone, Deserialize, Serialize)]
#[serde(rename_all ="kebab-case")]
pub struct SatisfiedControlImplementationResponsibility {
	/// Satisfied Control Implementation Responsibility Description
	/// An implementation statement that describes the aspects of a control or control statement implementation that a leveraging system is implementing based on a requirement from a leveraged system.
	pub description: String,
	pub props: Option<Vec<Property>>,
	/// Responsibility UUID
	/// A machine-oriented identifier reference to a control implementation that satisfies a responsibility imposed by a leveraged system.
	pub responsibility_uuid: Option<UuidDatatype>,
	pub remarks: Option<Remarks>,
	pub links: Option<Vec<Link>>,
	pub responsible_roles: Option<Vec<ResponsibleRole>>,
	/// Satisfied Universally Unique Identifier
	/// A machine-oriented, globally unique identifier with cross-instance scope that can be used to reference this satisfied control implementation entry elsewhere in this or other OSCAL instances. The locally defined UUID of the control implementation can be used to reference the data item locally or globally (e.g., in an imported OSCAL instance). This UUID should be assigned per-subject, which means it should be consistently used to identify the same subject across revisions of the document.
	pub uuid: UuidDatatype,
}
