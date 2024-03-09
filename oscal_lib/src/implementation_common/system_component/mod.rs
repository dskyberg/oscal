/// File name: ../oscal_lib/src/oscal_complete_oscal_implementation_common/system_component.rs
/// pub use oscal_complete_oscal_implementation_common::*;
///
/// pub mod oscal_complete_oscal_implementation_common;
use serde::{Deserialize, Serialize};
use serde_with::skip_serializing_none;

use crate::{
    metadata::{Link, Property, Remarks, ResponsibleRole},
    SchemaElement, UUIDDatatype,
};

use self::status::Status;
use super::protocol::Protocol;

pub mod status;

#[skip_serializing_none]
#[derive(Debug, Clone, PartialEq, Deserialize, Serialize)]
#[serde(rename_all = "kebab-case")]
pub struct SystemComponent {
    pub uuid: UUIDDatatype,
    ///"enum": [
    ///    "this-system",
    ///    "system",
    ///    "interconnection",
    ///    "software",
    ///    "hardware",
    ///    "service",
    ///    "policy",
    ///    "physical",
    ///    "process-procedure",
    ///    "plan",
    ///    "guidance",
    ///    "standard",
    ///    "validation",
    ///    "network"
    ///]
    #[serde(rename = "type")]
    pub _type: String,
    pub title: String,
    pub description: String,
    pub purpose: Option<String>,
    pub props: Option<Vec<Property>>,
    pub links: Option<Vec<Link>>,
    pub status: Status,
    pub responsible_roles: Option<Vec<ResponsibleRole>>,
    pub protocols: Option<Vec<Protocol>>,
    pub remarks: Option<Remarks>,
}

impl SchemaElement for SystemComponent {
    fn schema_title() -> &'static str {
        "Component"
    }
    fn schema_description() -> &'static str {
        r#"A defined component that can be part of an implemented system."#
    }
    fn schema_id() -> Option<&'static str> {
        Some("#assembly_oscal-implementation-common_system-component")
    }
    fn schema_path() -> &'static str {
        "oscal-complete-oscal-implementation-common:system-component"
    }
}
