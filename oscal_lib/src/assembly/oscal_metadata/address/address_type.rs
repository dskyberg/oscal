/// Address Type
/// Indicates the type of address.
/// $id: #assembly_oscal-metadata_address_address-type_address-type
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Deserialize, Serialize)]
#[serde(rename_all ="kebab-case")]
pub enum AddressType {
	// orig: home
	Home,
	// orig: work
	Work,
}
