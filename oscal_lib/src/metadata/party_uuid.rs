use serde::{Deserialize, Serialize};
use std::ops::Deref;

use crate::{Error, SchemaElement, UUIDDatatype};

#[derive(Debug, Clone, PartialEq, Deserialize, Serialize)]
#[serde(transparent)]
pub struct PartyUuid(UUIDDatatype);

impl SchemaElement for PartyUuid {
    fn schema_title() -> &'static str {
        "Party Reference"
    }
    fn schema_description() -> &'static str {
        "A machine-oriented identifier reference to another party defined in metadata. The UUID of the party in the source OSCAL instance is sufficient to reference the data item locally or globally (e.g., in an imported OSCAL instance)."
    }
    fn schema_id() -> Option<&'static str> {
        Some("#field_oscal-metadata_party-uuid")
    }
    fn schema_path() -> &'static str {
        "oscal-complete-oscal-metadata:party-uuid"
    }
}

impl Deref for PartyUuid {
    type Target = str;
    fn deref(&self) -> &Self::Target {
        self.0.deref()
    }
}

impl TryFrom<&str> for PartyUuid {
    type Error = Error;
    fn try_from(value: &str) -> Result<Self, Self::Error> {
        Ok(Self(UUIDDatatype::try_from(value)?))
    }
}
