/// File name: ../oscal_lib/src/oscal_complete_oscal_implementation_common/system_user.rs
/// pub use oscal_complete_oscal_implementation_common::*;
///
/// pub mod oscal_complete_oscal_implementation_common;
use serde::{Deserialize, Serialize};
use serde_with::skip_serializing_none;

use crate::{
    metadata::{Link, Property, Remarks, RoleId},
    SchemaElement, StringDatatype, UUIDDatatype,
};

use super::authorized_privilege::AuthorizedPrivilege;

#[skip_serializing_none]
#[derive(Debug, Clone, PartialEq, Deserialize, Serialize)]
#[serde(rename_all = "kebab-case")]
pub struct SystemUser {
    pub uuid: UUIDDatatype,
    pub title: Option<String>,
    pub short_name: Option<StringDatatype>,
    pub description: Option<String>,
    pub props: Option<Vec<Property>>,
    pub links: Option<Vec<Link>>,
    pub role_ids: Option<Vec<RoleId>>,
    pub authorized_privileges: Option<Vec<AuthorizedPrivilege>>,
    pub remarks: Option<Remarks>,
}

impl SchemaElement for SystemUser {
    fn schema_title() -> &'static str {
        "System User"
    }
    fn schema_description() -> &'static str {
        r#"A type of user that interacts with the system based on an associated role."#
    }
    fn schema_id() -> Option<&'static str> {
        Some("#assembly_oscal-implementation-common_system-user")
    }
    fn schema_path() -> &'static str {
        "oscal-complete-oscal-implementation-common:system-user"
    }
}
