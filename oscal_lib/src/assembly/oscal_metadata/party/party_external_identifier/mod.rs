pub use external_identifier_schema::*;


pub mod external_identifier_schema;

/// Party External Identifier
/// An identifier for a person or organization using a designated scheme. e.g. an Open Researcher and Contributor ID (ORCID)
/// $id: #assembly_oscal-metadata_party_party-external-identifier_party-external-identifier
use serde::{Deserialize, Serialize};
use serde_with::skip_serializing_none;

use crate::definitions::StringDatatype;

#[skip_serializing_none]
#[derive(Debug, Clone, Deserialize, Serialize)]
#[serde(rename_all ="kebab-case")]
pub struct PartyExternalIdentifier {
	pub id: StringDatatype,
	/// External Identifier Schema
	/// Indicates the type of external identifier.
	pub scheme: ExternalIdentifierSchema,
}
