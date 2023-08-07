/// Control Statement Implementation
/// Identifies which statements within a control are addressed.
/// $id: #assembly_oscal-component-definition_statement
use serde::{Deserialize, Serialize};
use serde_with::skip_serializing_none;

use crate::assembly::oscal_metadata::Link;
use crate::assembly::oscal_metadata::Property;
use crate::assembly::oscal_metadata::ResponsibleRole;
use crate::definitions::TokenDatatype;
use crate::definitions::UuidDatatype;
use crate::field::oscal_metadata::Remarks;

#[skip_serializing_none]
#[derive(Debug, Clone, Deserialize, Serialize)]
#[serde(rename_all ="kebab-case")]
pub struct Statement {
	pub responsible_roles: Option<Vec<ResponsibleRole>>,
	/// Statement Implementation Description
	/// A summary of how the containing control statement is implemented by the component or capability.
	pub description: String,
	/// Control Statement Reference
	/// A human-oriented identifier reference to a control statement.
	pub statement_id: TokenDatatype,
	/// Control Statement Reference Universally Unique Identifier
	/// A machine-oriented, globally unique identifier with cross-instance scope that can be used to reference this control statement elsewhere in this or other OSCAL instances. The UUID of the control statement in the source OSCAL instance is sufficient to reference the data item locally or globally (e.g., in an imported OSCAL instance).
	pub uuid: UuidDatatype,
	pub links: Option<Vec<Link>>,
	pub remarks: Option<Remarks>,
	pub props: Option<Vec<Property>>,
}
