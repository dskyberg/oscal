use serde::{Deserialize, Serialize};
use std::ops::Deref;

use crate::{Error, SchemaConstraint, UUIDDatatype};

#[derive(Debug, Clone, PartialEq, Deserialize, Serialize)]
#[serde(transparent)]
pub struct LocationUuid(UUIDDatatype);

impl SchemaConstraint for LocationUuid {
    fn constraint_title() -> &'static str {
        "Location Reference"
    }
    fn constraint_description() -> &'static str {
        "A machine-oriented identifier reference to a location defined in the metadata section of this or another OSCAL instance. The UUID of the location in the source OSCAL instance is sufficient to reference the data item locally or globally (e.g., in an imported OSCAL instance)."
    }
    fn constraint_id() -> &'static str {
        "#field_oscal-metadata_version"
    }
    fn schema_path() -> &'static str {
        "oscal-complete-oscal-metadata:location-uuid"
    }
}

impl Deref for LocationUuid {
    type Target = uuid::Uuid;
    fn deref(&self) -> &Self::Target {
        self.0.deref()
    }
}

impl TryFrom<&str> for LocationUuid {
    type Error = Error;
    fn try_from(value: &str) -> Result<Self, Self::Error> {
        Ok(Self(UUIDDatatype::try_from(value)?))
    }
}
