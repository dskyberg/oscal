/// Citation
/// A citation consisting of end note text and optional structured bibliographic data.
/// $id: #assembly_oscal-metadata_back-matter_resource_resource_citation_citation
use serde::{Deserialize, Serialize};
use serde_with::skip_serializing_none;

use crate::assembly::oscal_metadata::Link;
use crate::assembly::oscal_metadata::Property;

#[skip_serializing_none]
#[derive(Debug, Clone, Deserialize, Serialize)]
#[serde(rename_all ="kebab-case")]
pub struct Citation {
	pub links: Option<Vec<Link>>,
	pub props: Option<Vec<Property>>,
	/// Citation Text
	/// A line of citation text.
	pub text: String,
}
