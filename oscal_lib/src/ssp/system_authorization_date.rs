use serde::{Deserialize, Serialize};

use crate::{DateDatatype, Error, SchemaElement};

#[derive(Debug, Clone, PartialEq, Deserialize, Serialize)]
pub struct SystemAuthorizationDate(DateDatatype);

impl TryFrom<&str> for SystemAuthorizationDate {
    type Error = Error;
    fn try_from(value: &str) -> Result<Self, Self::Error> {
        Ok(Self(DateDatatype::try_from(value)?))
    }
}

impl SchemaElement for SystemAuthorizationDate {
    fn schema_title() -> &'static str {
        "System Authorization Date"
    }
    fn schema_description() -> &'static str {
        "The date the system received its authorization."
    }
    fn schema_id() -> Option<&'static str> {
        Some("#field_oscal-ssp_date-authorized")
    }
    fn schema_path() -> &'static str {
        "oscal-complete-oscal-ssp:date-authorized"
    }
}
