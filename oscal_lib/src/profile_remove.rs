use serde::{Deserialize, Serialize};
use serde_with::skip_serializing_none;

use crate::{SchemaConstraint, TokenDatatype};
#[skip_serializing_none]
#[derive(Debug, Clone, PartialEq, Deserialize, Serialize)]
#[serde(rename_all = "kebab-case")]
pub struct ProfileRemove {
    pub by_name: Option<TokenDatatype>,
    pub by_class: Option<TokenDatatype>,
    pub by_id: Option<TokenDatatype>,
    pub by_item_name: Option<TokenDatatype>,
    pub by_ns: Option<TokenDatatype>,
}

impl SchemaConstraint for ProfileRemove {
    fn constraint_title() -> &'static str {
        "Removal"
    }
    fn constraint_description() -> &'static str {
        "Specifies objects to be removed from a control based on specific aspects of the object that must all match."
    }
    fn constraint_id() -> &'static str {
        "#assembly_oscal-profile_remove"
    }
    fn schema_path() -> &'static str {
        "remove"
    }
}
