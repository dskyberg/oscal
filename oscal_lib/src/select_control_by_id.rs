use serde::{Deserialize, Serialize};
use serde_with::skip_serializing_none;

use crate::{SchemaConstraint, StringDatatype, TokenDatatype};

#[skip_serializing_none]
#[derive(Debug, Clone, PartialEq, Deserialize, Serialize)]
#[serde(rename_all = "kebab-case")]
pub struct SelectControlByIdMatching {
    pub pattern: StringDatatype,
}

#[skip_serializing_none]
#[derive(Debug, Clone, PartialEq, Deserialize, Serialize)]
#[serde(rename_all = "kebab-case")]
pub struct SelectControlById {
    pub with_child_controls: Option<TokenDatatype>,
    pub with_ids: Option<Vec<TokenDatatype>>,
    pub matching: Option<Vec<SelectControlByIdMatching>>,
}

/*
"constraint": {
    "bindings": [
        {"pattern": "select-control-by-id[@with-child-controls"]}
    ],
    "allowed-values": [
        "yes",
        "no"
    ]
}
 */
impl SchemaConstraint for SelectControlById {
    fn constraint_title() -> &'static str {
        "Call"
    }
    fn constraint_description() -> &'static str {
        "Call a control by its ID"
    }
    fn constraint_id() -> &'static str {
        "#assembly_oscal-profile_select-control-by-id"
    }
    fn schema_path() -> &'static str {
        "select-control-by-id"
    }
}
