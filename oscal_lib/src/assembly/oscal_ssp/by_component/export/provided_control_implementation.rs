/// Provided Control Implementation
/// Describes a capability which may be inherited by a leveraging system.
/// $id: #assembly_oscal-ssp_by-component_export_export_provided-control-implementation_provided-control-implementation
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
pub struct ProvidedControlImplementation {
	pub remarks: Option<Remarks>,
	pub props: Option<Vec<Property>>,
	pub responsible_roles: Option<Vec<ResponsibleRole>>,
	/// Provided Universally Unique Identifier
	/// A machine-oriented, globally unique identifier with cross-instance scope that can be used to reference this provided entry elsewhere in this or other OSCAL instances. The locally defined UUID of the provided entry can be used to reference the data item locally or globally (e.g., in an imported OSCAL instance). This UUID should be assigned per-subject, which means it should be consistently used to identify the same subject across revisions of the document.
	pub uuid: UuidDatatype,
	/// Provided Control Implementation Description
	/// An implementation statement that describes the aspects of the control or control statement implementation that can be provided to another system leveraging this system.
	pub description: String,
	pub links: Option<Vec<Link>>,
}
