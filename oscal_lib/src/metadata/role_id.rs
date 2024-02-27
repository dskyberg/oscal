use serde::{Deserialize, Serialize};

use crate::{Error, SchemaConstraint, TokenDatatype};

#[derive(Debug, Clone, PartialEq, Deserialize, Serialize)]
pub struct RoleId(TokenDatatype);

impl TryFrom<&str> for RoleId {
    type Error = Error;
    fn try_from(value: &str) -> Result<Self, Self::Error> {
        Ok(Self(TokenDatatype::try_from(value)?))
    }
}

impl SchemaConstraint for RoleId {
    fn constraint_title() -> &'static str {
        "Role Identifier Reference"
    }
    fn constraint_description() -> &'static str {
        "A human-oriented identifier reference to roles served by the user."
    }
    fn constraint_id() -> &'static str {
        "#field_oscal-metadata_role-id"
    }
    fn schema_path() -> &'static str {
        "oscal-complete-oscal-metadata:role-id"
    }
}
