use serde::{Deserialize, Serialize};
use serde_with::skip_serializing_none;

use crate::SchemaConstraint;
#[skip_serializing_none]
#[derive(Debug, Clone, PartialEq, Deserialize, Serialize)]
#[serde(rename_all = "kebab-case")]
pub struct CommonProtocol {}

impl SchemaConstraint for CommonProtocol {
    fn constraint_title() -> &'static str {
        ""
    }
    fn constraint_description() -> &'static str {
        ""
    }
    fn constraint_id() -> &'static str {
        ""
    }
    fn schema_path() -> &'static str {
        ""
    }
}
