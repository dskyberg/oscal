use serde::{Deserialize, Serialize};
use serde_with::skip_serializing_none;

use crate::{MetadataLink, MetadataProperty, SchemaConstraint, TokenDatatype, URIDatatype};

#[skip_serializing_none]
#[derive(Debug, Clone, PartialEq, Deserialize, Serialize)]
#[serde(rename_all = "kebab-case")]
pub struct CommonPart {
    pub id: Option<TokenDatatype>,
    pub name: TokenDatatype,
    pub ns: Option<URIDatatype>,
    pub class: Option<TokenDatatype>,
    pub title: Option<String>,
    pub props: Option<Vec<MetadataProperty>>,
    pub prose: Option<String>,
    pub parts: Option<Vec<CommonPart>>,
    pub links: Option<Vec<MetadataLink>>,
}

impl SchemaConstraint for CommonPart {
    fn constraint_title() -> &'static str {
        "Part"
    }
    fn constraint_description() -> &'static str {
        "A partition of a control's definition or a child of another part."
    }
    fn constraint_id() -> &'static str {
        "#assembly_oscal-catalog-common_part"
    }
    fn schema_path() -> &'static str {
        "control"
    }
}
