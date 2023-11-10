/// 1332
use serde::{Deserialize, Serialize};
use serde_with::skip_serializing_none;

use crate::{
    BackMatter, Metadata, ProfileImport, ProfileMerge, ProfileModify, SchemaConstraint,
    UUIDDatatype,
};
#[skip_serializing_none]
#[derive(Debug, Clone, PartialEq, Deserialize, Serialize)]
#[serde(rename_all = "kebab-case")]
pub struct Profile {
    pub uuid: UUIDDatatype,
    pub metadata: Metadata,
    pub imports: Vec<ProfileImport>,
    pub merg: Option<ProfileMerge>,
    pub modify: Option<ProfileModify>,
    pub back_matter: Option<BackMatter>,
}

impl SchemaConstraint for Profile {
    fn constraint_title() -> &'static str {
        "Profile"
    }
    fn constraint_description() -> &'static str {
        "Each OSCAL profile is defined by a Profile element"
    }
    fn constraint_id() -> &'static str {
        "#assembly_oscal-profile_profile"
    }
    fn schema_path() -> &'static str {
        "profile"
    }
}
