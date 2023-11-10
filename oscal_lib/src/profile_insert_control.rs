use serde::{Deserialize, Serialize};
use serde_with::skip_serializing_none;

use crate::{IncludeAll, SchemaConstraint, SelectControlById, TokenDatatype};
#[skip_serializing_none]
#[derive(Debug, Clone, PartialEq, Deserialize, Serialize)]
#[serde(rename_all = "kebab-case")]
pub struct ProfileInsertControl {
    pub order: Option<TokenDatatype>,
    pub include_all: Option<IncludeAll>,
    pub include_controls: Option<Vec<SelectControlById>>,
    pub exclude_controls: Option<Vec<SelectControlById>>,
}

/*
"constraint": {
    "bindings": [
        {"pattern": "insert-control"}
    ],
    "allowed-values": [
        "keep",
        "ascending",
        "descending"
    ]
}
 */

impl SchemaConstraint for ProfileInsertControl {
    fn constraint_title() -> &'static str {
        "Select controls"
    }
    fn constraint_description() -> &'static str {
        "Specifies which controls to use in the containing context."
    }
    fn constraint_id() -> &'static str {
        "#assembly_oscal-profile_insert-controls"
    }
    fn schema_path() -> &'static str {
        "insert-control"
    }
}
