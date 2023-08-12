pub use party_external_identifier::*;
pub use party_type::*;


pub mod party_external_identifier;
pub mod party_type;

/// Party (organization or person)
/// A responsible entity which is either a person or an organization.
/// $id: #assembly_oscal-metadata_party
use serde::{Deserialize, Serialize};
use serde_with::skip_serializing_none;

use crate::assembly::oscal_metadata::Address;
use crate::assembly::oscal_metadata::Link;
use crate::assembly::oscal_metadata::Property;
use crate::definitions::StringDatatype;
use crate::definitions::UuidDatatype;
use crate::field::oscal_metadata::EmailAddress;
use crate::field::oscal_metadata::LocationUuid;
use crate::field::oscal_metadata::Remarks;
use crate::field::oscal_metadata::TelephoneNumber;

#[skip_serializing_none]
#[derive(Debug, Clone, Deserialize, Serialize)]
#[serde(rename_all ="kebab-case")]
pub struct Party {
	pub email_addresses: Option<Vec<EmailAddress>>,
	pub remarks: Option<Remarks>,
	pub links: Option<Vec<Link>>,
	pub telephone_numbers: Option<Vec<TelephoneNumber>>,
	/// Party Type
	/// A category describing the kind of party the object describes.
	#[serde(rename = "type")]
	pub _type: PartyType,
	/// Party Universally Unique Identifier
	/// A machine-oriented, globally unique identifier with cross-instance scope that can be used to reference this defined party elsewhere in this or other OSCAL instances. The locally defined UUID of the party can be used to reference the data item locally or globally (e.g., from an importing OSCAL instance). This UUID should be assigned per-subject, which means it should be consistently used to identify the same subject across revisions of the document.
	pub uuid: UuidDatatype,
	pub external_ids: Option<Vec<PartyExternalIdentifier>>,
	pub addresses: Option<Vec<Address>>,
	pub location_uuids: Option<Vec<LocationUuid>>,
	/// Party Name
	/// The full name of the party. This is typically the legal name associated with the party.
	pub name: Option<StringDatatype>,
	pub member_of_organizations: Option<Vec<UuidDatatype>>,
	pub props: Option<Vec<Property>>,
	/// Party Short Name
	/// A short common name, abbreviation, or acronym for the party.
	pub short_name: Option<StringDatatype>,
}
