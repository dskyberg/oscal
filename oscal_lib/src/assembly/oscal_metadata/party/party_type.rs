/// Party Type
/// A category describing the kind of party the object describes.
/// $id: #assembly_oscal-metadata_party_party-type_party-type
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Deserialize, Serialize)]
#[serde(rename_all ="kebab-case")]
pub enum PartyType {
	// orig: person
	Person,
	// orig: organization
	Organization,
}
