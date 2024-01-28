/// File name: ../oscal_lib/src/oscal_complete_oscal_implementation_common/system_component.rs
/// pub use oscal_complete_oscal_implementation_common::*;
///
/// pub mod oscal_complete_oscal_implementation_common;
use serde::{Deserialize, Serialize};
use serde_with::skip_serializing_none;

use crate::{
    oscal_complete_oscal_metadata::{link, property, remarks, responsible_role},
    SchemaConstraint, TokenDatatype, UUIDDatatype,
};

#[skip_serializing_none]
#[derive(Debug, Clone, PartialEq, Deserialize, Serialize)]
#[serde(rename_all = "kebab-case")]
pub struct Status {
    /// "enum": [
    ///     "under-development",
    ///     "operational",
    ///     "disposition",
    ///     "other"
    /// ]
    state: TokenDatatype,
    remarks: Option<remarks::Remarks>,
}

impl SchemaConstraint for Status {
    fn constraint_title() -> &'static str {
        "Status"
    }

    fn constraint_description() -> &'static str {
        "Describes the operational status of the system component."
    }

    fn constraint_id() -> &'static str {
        "#assembly_oscal-implementation-common_system-component:status"
    }
    fn schema_path() -> &'static str {
        "oscal-complete-oscal-implementation-common:system-component:status"
    }
}

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
    /// "#assembly_oscal-metadata_property"
    pub props: Option<Vec<property::Property>>,
    /// "#assembly_oscal-metadata_link"
    pub links: Option<Vec<link::Link>>,
    pub status: Status,
    /// "#assembly_oscal-metadata_responsible-role"
    pub responsible_roles: Option<Vec<responsible_role::ResponsibleRole>>,
    /// "#assembly_oscal-implementation-common_protocol"
    pub protocols: Option<Vec<super::protocol::Protocol>>,
    /// "#field_oscal-metadata_remarks"
    pub remarks: Option<remarks::Remarks>,
}

impl SchemaConstraint for SystemComponent {
    fn constraint_title() -> &'static str {
        "Component"
    }
    fn constraint_description() -> &'static str {
        r#"A defined component that can be part of an implemented system."#
    }
    fn constraint_id() -> &'static str {
        "#assembly_oscal-implementation-common_system-component"
    }
    fn schema_path() -> &'static str {
        "oscal-complete-oscal-implementation-common:system-component"
    }
}
