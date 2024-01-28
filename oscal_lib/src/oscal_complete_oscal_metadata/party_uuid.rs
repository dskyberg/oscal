use serde::{Deserialize, Serialize};
use std::ops::Deref;

use crate::{SchemaConstraint, UUIDDatatype};

#[derive(Debug, Clone, PartialEq, Deserialize, Serialize)]
#[serde(transparent)]
pub struct PartyUuid(UUIDDatatype);

impl SchemaConstraint for PartyUuid {
    fn constraint_title() -> &'static str {
        "Party Reference"
    }
    fn constraint_description() -> &'static str {
        "A machine-oriented identifier reference to another party defined in metadata. The UUID of the party in the source OSCAL instance is sufficient to reference the data item locally or globally (e.g., in an imported OSCAL instance)."
    }
    fn constraint_id() -> &'static str {
        "#field_oscal-metadata_party-uuid"
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

impl From<&str> for PartyUuid {
    fn from(value: &str) -> Self {
        Self(UUIDDatatype::from(value))
    }
}
