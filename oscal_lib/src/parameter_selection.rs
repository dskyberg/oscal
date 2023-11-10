use serde::{Deserialize, Serialize};
use serde_with::skip_serializing_none;

use crate::{SchemaConstraint, TokenDatatype};

#[skip_serializing_none]
#[derive(Debug, Clone, PartialEq, Deserialize, Serialize)]
#[serde(rename_all = "kebab-case")]
pub struct ParameterSelection {
    pub how_many: Option<TokenDatatype>,
    pub choice: Option<Vec<String>>,
}

/*
"constraint": {
    "title": "Parameter Cardinality",
    "description": "Describes the number of selections that must occur. Without this setting, only one value should be assumed to be permitted.",
    "bindings": [
        pattern: "selection/how-many"
    ],
    "allowed-values": {
        "one",
        "one-or-two"
    }
}
 */
impl SchemaConstraint for ParameterSelection {
    fn constraint_title() -> &'static str {
        "Selection"
    }
    fn constraint_description() -> &'static str {
        "Presenting a choice among alternatives"
    }
    fn constraint_id() -> &'static str {
        "#assembly_oscal-catalog-common_parameter-selection"
    }
    fn schema_path() -> &'static str {
        "selection"
    }
}
