use serde::{Deserialize, Serialize};
use serde_with::skip_serializing_none;

use crate::SchemaConstraint;

#[skip_serializing_none]
#[derive(Debug, Clone, PartialEq, Deserialize, Serialize)]
#[serde(rename_all = "kebab-case")]
pub struct ParameterGuideline {
    pub prose: String,
}

impl SchemaConstraint for ParameterGuideline {
    fn constraint_title() -> &'static str {
        "Guideline"
    }
    fn constraint_description() -> &'static str {
        "A prose statement that provides a recommendation for the use of a parameter."
    }
    fn constraint_id() -> &'static str {
        "#assembly_oscal-catalog-common_parameter-guideline"
    }
    fn schema_path() -> &'static str {
        "guideline"
    }
}
