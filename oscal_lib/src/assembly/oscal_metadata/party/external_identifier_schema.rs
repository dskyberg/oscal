/// External Identifier Schema
/// Indicates the type of external identifier.
/// $id: #assembly_oscal-metadata_party_party-external-identifier_external-identifier-schema_external-identifier-schema
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Deserialize, Serialize)]
#[serde(rename_all ="kebab-case")]
pub enum ExternalIdentifierSchema {
	// orig: http://orcid.org/
#[serde(rename = "http://orcid.org/")]
	HttpOrcidOrg,
}
