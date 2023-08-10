/// Responsible Party
/// A reference to a set of organizations or persons that have responsibility for performing a referenced role in the context of the containing object.
/// $id: #assembly_oscal-metadata_responsible-party
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
pub struct ResponsibleParty {
	/// Responsible Role
	/// A human-oriented identifier reference to roles served by the user.
	pub role_id: TokenDatatype,
	pub props: Option<Vec<Property>>,
	pub links: Option<Vec<Link>>,
	pub remarks: Option<Remarks>,
	pub party_uuids: Vec<PartyUuid>,
}
