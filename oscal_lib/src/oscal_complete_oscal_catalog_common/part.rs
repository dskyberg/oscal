/// File name: ../oscal_lib/src/oscal_complete_oscal_catalog_common/part.rs
/// pub use oscal_complete_oscal_catalog_common::*;
///
/// pub mod oscal_complete_oscal_catalog_common;
use serde::{Deserialize, Serialize};
use serde_with::skip_serializing_none;

use crate::SchemaConstraint;

#[skip_serializing_none]
#[derive(Debug, Clone, PartialEq, Deserialize, Serialize)]
#[serde(rename_all = "kebab-case")]
pub struct Part {
}

impl SchemaConstraint for Part {
    fn constraint_title() -> &'static str {
        "Part"
    }
    fn constraint_description() -> &'static str {
        r#"A partition of a control's definition or a child of another part."#
    }
    fn constraint_id() -> &'static str {
        "#assembly_oscal-catalog-common_part"
    }
    fn schema_path() -> &'static str {
        "oscal-complete-oscal-catalog-common:part"
    }
}
