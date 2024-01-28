/// File name: ../oscal_lib/src/oscal_complete_oscal_implementation_common/system_user.rs
/// pub use oscal_complete_oscal_implementation_common::*;
///
/// pub mod oscal_complete_oscal_implementation_common;
use serde::{Deserialize, Serialize};
use serde_with::skip_serializing_none;

use crate::SchemaConstraint;

#[skip_serializing_none]
#[derive(Debug, Clone, PartialEq, Deserialize, Serialize)]
#[serde(rename_all = "kebab-case")]
pub struct SystemUser {
}

impl SchemaConstraint for SystemUser {
    fn constraint_title() -> &'static str {
        "System User"
    }
    fn constraint_description() -> &'static str {
        r#"A type of user that interacts with the system based on an associated role."#
    }
    fn constraint_id() -> &'static str {
        "#assembly_oscal-implementation-common_system-user"
    }
    fn schema_path() -> &'static str {
        "oscal-complete-oscal-implementation-common:system-user"
    }
}
