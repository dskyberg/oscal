use serde::{Deserialize, Serialize};

use crate::{Error, SchemaElement, TokenDatatype};

#[derive(Debug, Clone, PartialEq, Deserialize, Serialize)]
pub struct RoleId(TokenDatatype);

impl TryFrom<&str> for RoleId {
    type Error = Error;
    fn try_from(value: &str) -> Result<Self, Self::Error> {
        Ok(Self(TokenDatatype::try_from(value)?))
    }
}

impl SchemaElement for RoleId {
    fn schema_title() -> &'static str {
        "Role Identifier Reference"
    }
    fn schema_description() -> &'static str {
        "A human-oriented identifier reference to roles served by the user."
    }
    fn schema_id() -> Option<&'static str> {
        Some("#field_oscal-metadata_role-id")
    }
    fn schema_path() -> &'static str {
        "oscal-complete-oscal-metadata:role-id"
    }
}
