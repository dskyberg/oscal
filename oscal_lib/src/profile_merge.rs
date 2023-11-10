use serde::{Deserialize, Serialize};
use serde_with::skip_serializing_none;

use crate::{
    BooleanDatatype, ProfileGroup, ProfileInsertControl, SchemaConstraint, StringDatatype,
};

#[skip_serializing_none]
#[derive(Debug, Clone, PartialEq, Deserialize, Serialize)]
pub struct CustomGrouping {
    pub groups: Option<Vec<ProfileGroup>>,
    pub insert_controls: Option<Vec<ProfileInsertControl>>,
}

#[skip_serializing_none]
#[derive(Debug, Clone, PartialEq, Deserialize, Serialize)]
pub struct Flat {}

#[skip_serializing_none]
#[derive(Debug, Clone, PartialEq, Deserialize, Serialize)]
#[serde(rename_all = "kebab-case")]
pub struct ProfileMerge {
    pub combine: StringDatatype,
    pub flat: Option<Flat>,
    pub as_is: Option<BooleanDatatype>,
    pub custom: Option<CustomGrouping>,
}

impl SchemaConstraint for ProfileMerge {
    fn constraint_title() -> &'static str {
        "Merge controls"
    }
    fn constraint_description() -> &'static str {
        "A Merge element provides structuring directives that drive how controls are organized after resolution."
    }
    fn constraint_id() -> &'static str {
        "#assembly_oscal-profile_merge"
    }
    fn schema_path() -> &'static str {
        "merge"
    }
}
