use serde::{Deserialize, Serialize};
use serde_with::skip_serializing_none;

use crate::{
    metadata::{Link, Property, Remarks, ResponsibleRole},
    SchemaConstraint, UUIDDatatype,
};

#[skip_serializing_none]
#[derive(Debug, Clone, PartialEq, Deserialize, Serialize)]
#[serde(rename_all = "kebab-case")]
pub struct ControlImplementationResponsibility {
    pub uuid: UUIDDatatype,
    pub provided_uuid: Option<UUIDDatatype>,
    pub description: String,
    pub props: Option<Vec<Property>>,
    pub links: Option<Vec<Link>>,
    pub responsible_roles: Option<Vec<ResponsibleRole>>,
    pub remarks: Option<Remarks>,
}

impl SchemaConstraint for ControlImplementationResponsibility {
    fn constraint_title() -> &'static str {
        "Control Implementation Responsibility"
    }
    fn constraint_description() -> &'static str {
        "Describes a control implementation responsibility imposed on a leveraging system."
    }
    fn constraint_id() -> &'static str {
        "#assembly_oscal-ssp_by-component_export_control-implementation-responsibility"
    }
    fn schema_path() -> &'static str {
        "oscal-complete-oscal-ssp:by-component:export:control-implementation-responsibility"
    }
}
