use serde::{Deserialize, Serialize};
use serde_with::skip_serializing_none;

use crate::SchemaConstraint;

use crate::BackMatter;
use crate::CatalogControl;
use crate::CatalogGroup;
use crate::CommonParameter;
use crate::Metadata;
use crate::UUIDDatatype;

#[skip_serializing_none]
#[derive(Debug, Clone, PartialEq, Deserialize, Serialize)]
#[serde(rename_all = "kebab-case")]
pub struct Catalog {
    // "title": "Catalog Universally Unique Identifier",
    // "description": "A globally unique identifier with cross-instance scope for this catalog instance. This UUID should be changed when this document is revised.",
    pub uuid: UUIDDatatype,
    pub metadata: Metadata,
    pub params: Option<Vec<CommonParameter>>,
    pub controls: Option<Vec<CatalogControl>>,
    pub groups: Option<Vec<CatalogGroup>>,
    pub back_matter: Option<BackMatter>,
}

impl SchemaConstraint for Catalog {
    fn constraint_title() -> &'static str {
        "Catalog"
    }

    fn constraint_description() -> &'static str {
        "A collection of controls."
    }

    fn constraint_id() -> &'static str {
        "#assembly_oscal-catalog_catalog"
    }

    fn schema_path() -> &'static str {
        "#/definitions/catalog"
    }
}
