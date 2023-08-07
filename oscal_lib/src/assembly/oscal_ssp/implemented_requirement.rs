/// Control-based Requirement
/// Describes how the system satisfies the requirements of an individual control.
/// $id: #assembly_oscal-ssp_implemented-requirement
use serde::{Deserialize, Serialize};
use serde_with::skip_serializing_none;

use crate::assembly::oscal_implementation_common::SetParameter;
use crate::assembly::oscal_metadata::Link;
use crate::assembly::oscal_metadata::Property;
use crate::assembly::oscal_metadata::ResponsibleRole;
use crate::assembly::oscal_ssp::ByComponent;
use crate::assembly::oscal_ssp::Statement;
use crate::definitions::TokenDatatype;
use crate::definitions::UuidDatatype;
use crate::field::oscal_metadata::Remarks;

#[skip_serializing_none]
#[derive(Debug, Clone, Deserialize, Serialize)]
#[serde(rename_all ="kebab-case")]
pub struct ImplementedRequirement {
	pub props: Option<Vec<Property>>,
	/// Control Requirement Universally Unique Identifier
	/// A machine-oriented, globally unique identifier with cross-instance scope that can be used to reference this control requirement elsewhere in this or other OSCAL instances. The locally defined UUID of the control requirement can be used to reference the data item locally or globally (e.g., in an imported OSCAL instance). This UUID should be assigned per-subject, which means it should be consistently used to identify the same subject across revisions of the document.
	pub uuid: UuidDatatype,
	/// Control Identifier Reference
	/// A human-oriented identifier reference to a control with a corresponding id value. When referencing an externally defined control, the Control Identifier Reference must be used in the context of the external / imported OSCAL instance (e.g., uri-reference).
	pub control_id: TokenDatatype,
	pub by_components: Option<Vec<ByComponent>>,
	pub set_parameters: Option<Vec<SetParameter>>,
	pub links: Option<Vec<Link>>,
	pub remarks: Option<Remarks>,
	pub statements: Option<Vec<Statement>>,
	pub responsible_roles: Option<Vec<ResponsibleRole>>,
}
