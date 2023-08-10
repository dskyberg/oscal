/// Location
/// A location, with associated metadata that can be referenced.
/// $id: #assembly_oscal-metadata_location
use serde::{Deserialize, Serialize};
use serde_with::skip_serializing_none;

use crate::assembly::oscal_metadata::Address;
use crate::assembly::oscal_metadata::Link;
use crate::assembly::oscal_metadata::Property;
use crate::definitions::UriDatatype;
use crate::definitions::UuidDatatype;
use crate::field::oscal_metadata::EmailAddress;
use crate::field::oscal_metadata::Remarks;
use crate::field::oscal_metadata::TelephoneNumber;

#[skip_serializing_none]
#[derive(Debug, Clone, Deserialize, Serialize)]
#[serde(rename_all ="kebab-case")]
pub struct Location {
	pub email_addresses: Option<Vec<EmailAddress>>,
	pub address: Address,
	pub links: Option<Vec<Link>>,
	/// Location Universally Unique Identifier
	/// A machine-oriented, globally unique identifier with cross-instance scope that can be used to reference this defined location elsewhere in this or other OSCAL instances. The locally defined UUID of the location can be used to reference the data item locally or globally (e.g., from an importing OSCAL instance). This UUID should be assigned per-subject, which means it should be consistently used to identify the same subject across revisions of the document.
	pub uuid: UuidDatatype,
	pub telephone_numbers: Option<Vec<TelephoneNumber>>,
	pub remarks: Option<Remarks>,
	pub props: Option<Vec<Property>>,
	/// Location Title
	/// A name given to the location, which may be used by a tool for display and navigation.
	pub title: Option<String>,
	pub urls: Option<Vec<UriDatatype>>,
}
