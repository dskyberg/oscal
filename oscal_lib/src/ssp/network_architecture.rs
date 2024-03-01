/// File name: ../oscal_lib/src/oscal_complete_oscal_ssp/network_architecture.rs
/// pub use oscal_complete_oscal_ssp::*;
///
/// pub mod oscal_complete_oscal_ssp;
use serde::{Deserialize, Serialize};
use serde_with::skip_serializing_none;

use crate::{
    metadata::{Link, Property, Remarks},
    SchemaConstraint,
};

use super::diagram::Diagram;

#[skip_serializing_none]
#[derive(Debug, Clone, PartialEq, Deserialize, Serialize)]
#[serde(rename_all = "kebab-case")]
pub struct NetworkArchitecture {
    pub description: String,
    pub props: Option<Vec<Property>>,
    pub links: Option<Vec<Link>>,
    pub diagrams: Option<Vec<Diagram>>,
    pub remarks: Option<Remarks>,
}

impl SchemaConstraint for NetworkArchitecture {
    fn constraint_title() -> &'static str {
        "Network Architecture"
    }
    fn constraint_description() -> &'static str {
        r#"A description of the system's network architecture, optionally supplemented by diagrams that illustrate the network architecture."#
    }
    fn constraint_id() -> &'static str {
        "#assembly_oscal-ssp_network-architecture"
    }
    fn schema_path() -> &'static str {
        "oscal-complete-oscal-ssp:network-architecture"
    }
}
