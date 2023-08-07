/// Import Component Definition
/// Loads a component definition from another resource.
/// $id: #assembly_oscal-component-definition_import-component-definition
use serde::{Deserialize, Serialize};
use serde_with::skip_serializing_none;

use crate::definitions::UriReferenceDatatype;

#[skip_serializing_none]
#[derive(Debug, Clone, Deserialize, Serialize)]
#[serde(rename_all ="kebab-case")]
pub struct ImportComponentDefinition {
	/// Hyperlink Reference
	/// A link to a resource that defines a set of components and/or capabilities to import into this collection.
	pub href: UriReferenceDatatype,
}
