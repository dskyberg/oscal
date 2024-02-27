use serde::{Deserialize, Serialize};
use serde_with::skip_serializing_none;

use crate::{metadata::Remarks, SchemaConstraint, StringDatatype};

#[skip_serializing_none]
#[derive(Debug, Clone, PartialEq, Deserialize, Serialize)]
#[serde(rename_all = "kebab-case")]
pub struct ConstraintTest {
    pub expression: StringDatatype,
    pub remarks: Option<Remarks>,
}

impl SchemaConstraint for ConstraintTest {
    fn constraint_title() -> &'static str {
        "Constraint Test"
    }

    fn constraint_description() -> &'static str {
        "A test expression which is expected to be evaluated by a tool."
    }

    fn constraint_id() -> &'static str {
        "#assembly_oscal-catalog-common_parameter-constraint:constraint-test"
    }

    fn schema_path() -> &'static str {
        "#assembly_oscal-catalog-common_parameter-constraint:constraint-test"
    }
}
