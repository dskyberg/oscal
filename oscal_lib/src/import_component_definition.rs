use serde::{Deserialize, Serialize};
use serde_with::skip_serializing_none;

use crate::{SchemaConstraint, URIReferenceDatatype};

#[skip_serializing_none]
#[derive(Debug, Clone, PartialEq, Deserialize, Serialize)]
#[serde(rename_all = "kebab-case")]
pub struct ImportComponentDefinition {
    pub href: URIReferenceDatatype,
}

impl SchemaConstraint for ImportComponentDefinition {
    fn constraint_title() -> &'static str {
        "Import Component Definition"
    }
    fn constraint_description() -> &'static str {
        "Loads a component definition from another resource."
    }
    fn constraint_id() -> &'static str {
        "#assembly_oscal-component-definition_import-component-definition"
    }
    fn schema_path() -> &'static str {
        ""
    }
}
