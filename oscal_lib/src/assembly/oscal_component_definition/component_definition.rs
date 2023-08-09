/// Component Definition
/// A collection of component descriptions, which may optionally be grouped by capability.
/// $id: #assembly_oscal-component-definition_component-definition
use serde::{Deserialize, Serialize};
use serde_with::skip_serializing_none;

use crate::assembly::oscal_component_definition::Capability;
use crate::assembly::oscal_component_definition::DefinedComponent;
use crate::assembly::oscal_component_definition::ImportComponentDefinition;
use crate::assembly::oscal_metadata::BackMatter;
use crate::assembly::oscal_metadata::Metadata;
use crate::definitions::UuidDatatype;

#[skip_serializing_none]
#[derive(Debug, Clone, Deserialize, Serialize)]
#[serde(rename_all ="kebab-case")]
pub struct ComponentDefinition {
	pub back_matter: Option<BackMatter>,
	pub metadata: Metadata,
	pub components: Option<Vec<DefinedComponent>>,
	pub capabilities: Option<Vec<Capability>>,
	pub import_component_definitions: Option<Vec<ImportComponentDefinition>>,
	/// Component Definition Universally Unique Identifier
	/// A machine-oriented, globally unique identifier with cross-instance scope that can be used to reference this component definition elsewhere in this or other OSCAL instances. The locally defined UUID of the component definition can be used to reference the data item locally or globally (e.g., in an imported OSCAL instance). This UUID should be assigned per-subject, which means it should be consistently used to identify the same subject across revisions of the document.
	pub uuid: UuidDatatype,
}
