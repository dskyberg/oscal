use serde::{Deserialize, Serialize};
use serde_with::skip_serializing_none;

use crate::{metadata::Remarks, SchemaElement, StringDatatype};

#[skip_serializing_none]
#[derive(Debug, Clone, PartialEq, Deserialize, Serialize)]
#[serde(rename_all = "kebab-case")]
pub struct ConstraintTest {
    pub expression: StringDatatype,
    pub remarks: Option<Remarks>,
}

impl SchemaElement for ConstraintTest {
    fn schema_title() -> &'static str {
        "Constraint Test"
    }

    fn schema_description() -> &'static str {
        "A test expression which is expected to be evaluated by a tool."
    }

    fn schema_id() -> Option<&'static str> {
        None
    }

    fn schema_path() -> &'static str {
        "#assembly_oscal-catalog-common_parameter-constraint:constraint-test"
    }
}
