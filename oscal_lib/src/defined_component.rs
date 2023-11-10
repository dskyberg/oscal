use serde::{Deserialize, Serialize};
use serde_with::skip_serializing_none;

use crate::{
    responsible_role::ResponsibleRole, CommonProtocol, ControlImplementation, MetadataLink,
    MetadataProperty, MetadataRemarks, SchemaConstraint, StringDatatype, UUIDDatatype,
};
#[skip_serializing_none]
#[derive(Debug, Clone, PartialEq, Deserialize, Serialize)]
#[serde(rename_all = "kebab-case")]
pub struct DefinedComponent {
    pub uuid: UUIDDatatype,
    #[serde(rename = "type")]
    pub type_: StringDatatype,
    pub title: String,
    pub description: String,
    pub purpose: Option<String>,
    pub props: Option<Vec<MetadataProperty>>,
    pub links: Option<Vec<MetadataLink>>,
    pub responsible_roles: Option<Vec<ResponsibleRole>>,
    pub protocols: Option<Vec<CommonProtocol>>,
    pub control_implementations: Option<Vec<ControlImplementation>>,
    pub remarks: Option<MetadataRemarks>,
}

/*
"constraint": {
    "bindings": [
        {"pattern": "defined-component[@type]"}
    ],
    "allowed-values": [
        "interconnection",
        "software",
        "hardware",
        "service",
        "policy",
        "physical",
        "process-procedure",
        "plan",
        "guidance",
        "standard",
        "validation"
    ]
}
*/

impl SchemaConstraint for DefinedComponent {
    fn constraint_title() -> &'static str {
        ""
    }
    fn constraint_description() -> &'static str {
        ""
    }
    fn constraint_id() -> &'static str {
        ""
    }
    fn schema_path() -> &'static str {
        ""
    }
}
