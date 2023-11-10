use serde::{Deserialize, Serialize};
use serde_with::skip_serializing_none;

use crate::{MetadataRemarks, SchemaConstraint, StringDatatype};

#[skip_serializing_none]
#[derive(Debug, Clone, PartialEq, Deserialize, Serialize)]
#[serde(rename_all = "kebab-case")]
pub struct ParameterConstraintTest {
    pub expression: StringDatatype,
    pub remarks: Option<MetadataRemarks>,
}

impl SchemaConstraint for ParameterConstraintTest {
    fn constraint_title() -> &'static str {
        "Constraint Test"
    }
    fn constraint_description() -> &'static str {
        "A test expression which is expected to be evaluated by a tool."
    }
    fn schema_path() -> &'static str {
        "test"
    }
    fn constraint_id() -> &'static str {
        ""
    }
}

#[skip_serializing_none]
#[derive(Debug, Clone, PartialEq, Deserialize, Serialize)]
#[serde(rename_all = "kebab-case")]
pub struct ParameterConstraint {
    pub description: Option<String>,
    pub tests: Option<Vec<ParameterConstraintTest>>,
}

impl SchemaConstraint for ParameterConstraint {
    fn constraint_title() -> &'static str {
        "Constraint"
    }
    fn constraint_description() -> &'static str {
        "A formal or informal expression of a constraint or test"
    }
    fn constraint_id() -> &'static str {
        "#assembly_oscal-catalog-common_parameter-constraint"
    }
    fn schema_path() -> &'static str {
        "constraint"
    }
}
