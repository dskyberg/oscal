use serde::{Deserialize, Serialize};

use crate::{DateDatatype, Error, SchemaConstraint};

#[derive(Debug, Clone, PartialEq, Deserialize, Serialize)]
pub struct SystemAuthorizationDate(DateDatatype);

impl TryFrom<&str> for SystemAuthorizationDate {
    type Error = Error;
    fn try_from(value: &str) -> Result<Self, Self::Error> {
        Ok(Self(DateDatatype::try_from(value)?))
    }
}

impl SchemaConstraint for SystemAuthorizationDate {
    fn constraint_title() -> &'static str {
        "System Authorization Date"
    }
    fn constraint_description() -> &'static str {
        "The date the system received its authorization."
    }
    fn constraint_id() -> &'static str {
        "#field_oscal-ssp_date-authorized"
    }
    fn schema_path() -> &'static str {
        "oscal-complete-oscal-ssp:date-authorized"
    }
}
