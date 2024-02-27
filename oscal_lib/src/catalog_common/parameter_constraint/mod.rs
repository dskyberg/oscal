/// File name: ../oscal_lib/src/oscal_complete_oscal_catalog_common/parameter_constraint.rs
/// pub use oscal_complete_oscal_catalog_common::*;
///
/// pub mod oscal_complete_oscal_catalog_common;
use serde::{Deserialize, Serialize};
use serde_with::skip_serializing_none;

use crate::SchemaConstraint;

use self::constraint_test::ConstraintTest;

mod constraint_test;

#[skip_serializing_none]
#[derive(Debug, Clone, PartialEq, Deserialize, Serialize)]
#[serde(rename_all = "kebab-case")]
pub struct ParameterConstraint {
    pub description: Option<String>,
    pub tests: Option<Vec<ConstraintTest>>,
}

impl SchemaConstraint for ParameterConstraint {
    fn constraint_title() -> &'static str {
        "Constraint"
    }
    fn constraint_description() -> &'static str {
        r#"A formal or informal expression of a constraint or test"#
    }
    fn constraint_id() -> &'static str {
        "#assembly_oscal-catalog-common_parameter-constraint"
    }
    fn schema_path() -> &'static str {
        "oscal-complete-oscal-catalog-common:parameter-constraint"
    }
}
