/// File name: ../oscal_lib/src/oscal_complete_oscal_profile/profile.rs
/// pub use oscal_complete_oscal_profile::*;
///
/// pub mod oscal_complete_oscal_profile;
use serde::{Deserialize, Serialize};
use serde_with::skip_serializing_none;

use crate::{
    metadata::{BackMatter, Metadata},
    SchemaElement, UUIDDatatype,
};

use self::{import::Import, merge::Merge, modify::Modify};

pub mod add;
pub mod alter;
pub mod group;
pub mod import;
pub mod insert_controls;
pub mod merge;
pub mod modify;
pub mod remove;
pub mod select_control_by_id;

#[skip_serializing_none]
#[derive(Debug, Clone, PartialEq, Deserialize, Serialize)]
#[serde(rename_all = "kebab-case")]
pub struct Profile {
    pub uuid: UUIDDatatype,
    pub metadata: Metadata,
    pub imports: Vec<Import>,
    pub merge: Option<Merge>,
    pub modify: Option<Modify>,
    pub back_mater: Option<BackMatter>,
}

impl SchemaElement for Profile {
    fn schema_title() -> &'static str {
        "Profile"
    }
    fn schema_description() -> &'static str {
        r#"Each OSCAL profile is defined by a Profile element"#
    }
    fn schema_id() -> Option<&'static str> {
        Some("#assembly_oscal-profile_profile")
    }
    fn schema_path() -> &'static str {
        "oscal-complete-oscal-profile:profile"
    }
}
