use serde::{Deserialize, Serialize};
use serde_with::skip_serializing_none;

use crate::{
    metadata::{Link, Property, Remarks},
    SchemaConstraint,
};

use self::{
    control_implementation_responsibility::ControlImplementationResponsibility,
    provided_control_implementation::ProvidedControlImplementation,
};

pub mod control_implementation_responsibility;
pub mod provided_control_implementation;

#[skip_serializing_none]
#[derive(Debug, Clone, PartialEq, Deserialize, Serialize)]
#[serde(rename_all = "kebab-case")]
pub struct Export {
    pub description: Option<String>,
    pub props: Option<Vec<Property>>,
    pub links: Option<Vec<Link>>,
    pub provided: Option<Vec<ProvidedControlImplementation>>,
    pub responsibilities: Option<Vec<ControlImplementationResponsibility>>,
    pub remarks: Option<Remarks>,
}

impl SchemaConstraint for Export {
    fn constraint_title() -> &'static str {
        "Export"
    }
    fn constraint_description() -> &'static str {
        "Identifies content intended for external consumption, such as with leveraged organizations."
    }
    fn constraint_id() -> &'static str {
        "#assembly_oscal-ssp_by-component_export"
    }
    fn schema_path() -> &'static str {
        "oscal-complete-oscal-ssp:by-component:export"
    }
}
