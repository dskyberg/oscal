/// Document Identification Scheme
/// Qualifies the kind of document identifier using a URI. If the scheme is not provided the value of the element will be interpreted as a string of characters.
/// $id: #field_oscal-metadata_document-id_document-identification-scheme_document-identification-scheme
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Deserialize, Serialize)]
#[serde(rename_all ="kebab-case")]
pub enum DocumentIdentificationScheme {
	// orig: http://www.doi.org/
#[serde(rename = "http://www.doi.org/")]
	HttpWwwDoiOrg,
	// orig: https://www.doi.org/
#[serde(rename = "https://www.doi.org/")]
	HttpsWwwDoiOrg,
}
