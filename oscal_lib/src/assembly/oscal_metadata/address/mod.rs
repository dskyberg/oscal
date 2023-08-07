pub use address_type::*;


pub mod address_type;

/// Address
/// A postal address for the location.
/// $id: #assembly_oscal-metadata_address
use serde::{Deserialize, Serialize};
use serde_with::skip_serializing_none;

use crate::definitions::StringDatatype;
use crate::field::oscal_metadata::AddrLine;

#[skip_serializing_none]
#[derive(Debug, Clone, Deserialize, Serialize)]
#[serde(rename_all ="kebab-case")]
pub struct Address {
	/// State
	/// State, province or analogous geographical region for mailing address
	pub state: Option<StringDatatype>,
	/// Address Type
	/// Indicates the type of address.
	#[serde(rename = "type")]
	pub _type: Option<AddressType>,
	pub addr_lines: Option<Vec<AddrLine>>,
	/// City
	/// City, town or geographical region for the mailing address.
	pub city: Option<StringDatatype>,
	/// Country Code
	/// The ISO 3166-1 alpha-2 country code for the mailing address.
	pub country: Option<StringDatatype>,
	/// Postal Code
	/// Postal or ZIP code for mailing address
	pub postal_code: Option<StringDatatype>,
}
