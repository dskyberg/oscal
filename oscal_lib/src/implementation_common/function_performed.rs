use serde::{Deserialize, Serialize};
use std::ops::Deref;

use crate::{Error, SchemaConstraint, StringDatatype};

#[derive(Debug, Clone, PartialEq, Deserialize, Serialize)]
pub struct FunctionPerformed(StringDatatype);

impl Deref for FunctionPerformed {
    type Target = str;

    fn deref(&self) -> &Self::Target {
        self.0.deref()
    }
}

impl TryFrom<&str> for FunctionPerformed {
    type Error = Error;
    fn try_from(value: &str) -> Result<Self, Self::Error> {
        Ok(Self(StringDatatype::try_from(value)?))
    }
}

impl SchemaConstraint for FunctionPerformed {
    fn constraint_title() -> &'static str {
        "Functions Performed"
    }
    fn constraint_description() -> &'static str {
        "Describes a function performed for a given authorized privilege by this user class."
    }
    fn constraint_id() -> &'static str {
        "#field_oscal-implementation-common_function-performed"
    }
    fn schema_path() -> &'static str {
        "oscal-complete-oscal-implementation-common:function-performed"
    }
}
