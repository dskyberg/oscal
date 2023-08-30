/// Control Implementation
/// Describes how the containing component or capability implements an individual control.
/// $id: #assembly_oscal-component-definition_implemented-requirement
use serde::{Deserialize, Serialize};
use serde_with::skip_serializing_none;

use crate::assembly::oscal_component_definition::Statement;
use crate::assembly::oscal_implementation_common::SetParameter;
use crate::assembly::oscal_metadata::Link;
use crate::assembly::oscal_metadata::Property;
use crate::assembly::oscal_metadata::ResponsibleRole;
use crate::definitions::TokenDatatype;
use crate::definitions::UuidDatatype;
use crate::field::oscal_metadata::Remarks;

#[skip_serializing_none]
#[derive(Debug, Clone, Deserialize, Serialize)]
#[serde(rename_all ="kebab-case")]
pub struct ImplementedRequirement {
	pub props: Option<Vec<Property>>,
	/// Control Implementation Identifier
	/// A machine-oriented, globally unique identifier with cross-instance scope that can be used to reference a specific control implementation elsewhere in this or other OSCAL instances. The locally defined UUID of the control implementation can be used to reference the data item locally or globally (e.g., in an imported OSCAL instance).This UUID should be assigned per-subject, which means it should be consistently used to identify the same subject across revisions of the document.
	pub uuid: UuidDatatype,
	pub responsible_roles: Option<Vec<ResponsibleRole>>,
	pub remarks: Option<Remarks>,
	/// Control Implementation Description
	/// A suggestion for how the specified control may be implemented if the containing component or capability is instantiated in a system security plan.
	pub description: String,
	pub links: Option<Vec<Link>>,
	pub statements: Option<Vec<Statement>>,
	pub set_parameters: Option<Vec<SetParameter>>,
	/// Control Identifier Reference
	/// A human-oriented identifier reference to a control with a corresponding id value. When referencing an externally defined control, the Control Identifier Reference must be used in the context of the external / imported OSCAL instance (e.g., uri-reference).
	pub control_id: TokenDatatype,
}
