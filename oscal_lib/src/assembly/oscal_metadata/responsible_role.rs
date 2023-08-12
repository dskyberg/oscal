/// Responsible Role
/// A reference to one or more roles with responsibility for performing a function relative to the containing object.
/// $id: #assembly_oscal-metadata_responsible-role
use serde::{Deserialize, Serialize};
use serde_with::skip_serializing_none;

use crate::assembly::oscal_metadata::Link;
use crate::assembly::oscal_metadata::Property;
use crate::definitions::TokenDatatype;
use crate::field::oscal_metadata::PartyUuid;
use crate::field::oscal_metadata::Remarks;

#[skip_serializing_none]
#[derive(Debug, Clone, Deserialize, Serialize)]
#[serde(rename_all ="kebab-case")]
pub struct ResponsibleRole {
	pub remarks: Option<Remarks>,
	pub links: Option<Vec<Link>>,
	pub party_uuids: Option<Vec<PartyUuid>>,
	pub props: Option<Vec<Property>>,
	/// Responsible Role ID
	/// A human-oriented identifier reference to roles responsible for the business function.
	pub role_id: TokenDatatype,
}
