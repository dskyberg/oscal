use serde::{Deserialize, Serialize};

use crate::SchemaConstraint;

#[derive(Debug, Clone, PartialEq, Deserialize, Serialize)]
pub struct IncludeAll {}

impl SchemaConstraint for IncludeAll {
    fn constraint_title() -> &'static str {
        "Include All"
    }
    fn constraint_description() -> &'static str {
        "Include all controls from the imported catalog or profile resources."
    }
    fn constraint_id() -> &'static str {
        "#assembly_oscal-catalog-common_include-all"
    }
    fn schema_path() -> &'static str {
        "include-all"
    }
}
