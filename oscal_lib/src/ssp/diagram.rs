/// File name: ../oscal_lib/src/oscal_complete_oscal_ssp/diagram.rs
/// pub use oscal_complete_oscal_ssp::*;
///
/// pub mod oscal_complete_oscal_ssp;
use serde::{Deserialize, Serialize};
use serde_with::skip_serializing_none;

use crate::{
    metadata::{Link, Property, Remarks},
    SchemaElement, UUIDDatatype,
};

#[skip_serializing_none]
#[derive(Debug, Clone, PartialEq, Deserialize, Serialize)]
#[serde(rename_all = "kebab-case")]
pub struct Diagram {
    pub uuid: UUIDDatatype,
    pub description: Option<String>,
    pub props: Option<Vec<Property>>,
    pub links: Option<Vec<Link>>,
    pub caption: Option<String>,
    pub remarks: Option<Remarks>,
}

impl SchemaElement for Diagram {
    fn schema_title() -> &'static str {
        "Diagram"
    }
    fn schema_description() -> &'static str {
        r#"A graphic that provides a visual representation the system, or some aspect of it."#
    }
    fn schema_id() -> Option<&'static str> {
        Some("#assembly_oscal-ssp_diagram")
    }
    fn schema_path() -> &'static str {
        "oscal-complete-oscal-ssp:diagram"
    }
}
