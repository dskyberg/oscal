/// Control Implementation Responsibility
/// Describes a control implementation responsibility imposed on a leveraging system.
/// $id: #assembly_oscal-ssp_by-component_export_control-implementation-responsibility_control-implementation-responsibility
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
pub struct ControlImplementationResponsibility {
	pub responsible_roles: Option<Vec<ResponsibleRole>>,
	pub links: Option<Vec<Link>>,
	/// Provided UUID
	/// A machine-oriented identifier reference to an inherited control implementation that a leveraging system is inheriting from a leveraged system.
	pub provided_uuid: Option<UuidDatatype>,
	/// Responsibility Universally Unique Identifier
	/// A machine-oriented, globally unique identifier with cross-instance scope that can be used to reference this responsibility elsewhere in this or other OSCAL instances. The locally defined UUID of the responsibility can be used to reference the data item locally or globally (e.g., in an imported OSCAL instance). This UUID should be assigned per-subject, which means it should be consistently used to identify the same subject across revisions of the document.
	pub uuid: UuidDatatype,
	/// Control Implementation Responsibility Description
	/// An implementation statement that describes the aspects of the control or control statement implementation that a leveraging system must implement to satisfy the control provided by a leveraged system.
	pub description: String,
	pub props: Option<Vec<Property>>,
	pub remarks: Option<Remarks>,
}
