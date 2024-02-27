/// File name: ../oscal_lib/src/oscal_complete_oscal_implementation_common/authorized_privilege.rs
/// pub use oscal_complete_oscal_implementation_common::*;
///
/// pub mod oscal_complete_oscal_implementation_common;
use serde::{Deserialize, Serialize};
use serde_with::skip_serializing_none;

use crate::SchemaConstraint;

use super::function_performed::FunctionPerformed;

#[skip_serializing_none]
#[derive(Debug, Clone, PartialEq, Deserialize, Serialize)]
#[serde(rename_all = "kebab-case")]
pub struct AuthorizedPrivilege {
    pub title: String,
    pub description: Option<String>,
    pub functions_performed: Vec<FunctionPerformed>,
}

impl SchemaConstraint for AuthorizedPrivilege {
    fn constraint_title() -> &'static str {
        "Privilege"
    }
    fn constraint_description() -> &'static str {
        r#"Identifies a specific system privilege held by the user, along with an associated description and/or rationale for the privilege."#
    }
    fn constraint_id() -> &'static str {
        "#assembly_oscal-implementation-common_authorized-privilege"
    }
    fn schema_path() -> &'static str {
        "oscal-complete-oscal-implementation-common:authorized-privilege"
    }
}
