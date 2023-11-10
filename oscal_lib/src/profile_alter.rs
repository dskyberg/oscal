use serde::{Deserialize, Serialize};
use serde_with::skip_serializing_none;

use crate::{ProfileAdd, ProfileRemove, SchemaConstraint, TokenDatatype};
#[skip_serializing_none]
#[derive(Debug, Clone, PartialEq, Deserialize, Serialize)]
#[serde(rename_all = "kebab-case")]
pub struct ProfileAlter {
    pub control_id: TokenDatatype,
    pub removes: Option<Vec<ProfileRemove>>,
    pub adds: Option<Vec<ProfileAdd>>,
}

impl SchemaConstraint for ProfileAlter {
    fn constraint_title() -> &'static str {
        "Alteration"
    }
    fn constraint_description() -> &'static str {
        "An Alter element specifies changes to be made to an included control when a profile is resolved."
    }
    fn constraint_id() -> &'static str {
        "#assembly_oscal-profile_alter"
    }
    fn schema_path() -> &'static str {
        "alter"
    }
}
