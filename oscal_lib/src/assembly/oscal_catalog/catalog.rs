/// Catalog
/// A collection of controls.
/// $id: #assembly_oscal-catalog_catalog
use serde::{Deserialize, Serialize};
use serde_with::skip_serializing_none;

use crate::assembly::oscal_catalog::Control;
use crate::assembly::oscal_catalog::Group;
use crate::assembly::oscal_catalog_common::Parameter;
use crate::assembly::oscal_metadata::BackMatter;
use crate::assembly::oscal_metadata::Metadata;
use crate::definitions::UuidDatatype;

#[skip_serializing_none]
#[derive(Debug, Clone, Deserialize, Serialize)]
#[serde(rename_all ="kebab-case")]
pub struct Catalog {
	pub metadata: Metadata,
	pub controls: Option<Vec<Control>>,
	/// Catalog Universally Unique Identifier
	/// A globally unique identifier with cross-instance scope for this catalog instance. This UUID should be changed when this document is revised.
	pub uuid: UuidDatatype,
	pub params: Option<Vec<Parameter>>,
	pub back_matter: Option<BackMatter>,
	pub groups: Option<Vec<Group>>,
}
