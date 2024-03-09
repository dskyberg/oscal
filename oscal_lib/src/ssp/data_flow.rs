/// File name: ../oscal_lib/src/oscal_complete_oscal_ssp/data_flow.rs
/// pub use oscal_complete_oscal_ssp::*;
///
/// pub mod oscal_complete_oscal_ssp;
use serde::{Deserialize, Serialize};
use serde_with::skip_serializing_none;

use crate::{
    metadata::{Link, Property, Remarks},
    SchemaElement,
};

use super::diagram::Diagram;

#[skip_serializing_none]
#[derive(Debug, Clone, PartialEq, Deserialize, Serialize)]
#[serde(rename_all = "kebab-case")]
pub struct DataFlow {
    pub description: String,
    pub props: Option<Vec<Property>>,
    pub links: Option<Vec<Link>>,
    pub diagrams: Option<Vec<Diagram>>,
    pub remarks: Option<Remarks>,
}

impl SchemaElement for DataFlow {
    fn schema_title() -> &'static str {
        "Data Flow"
    }
    fn schema_description() -> &'static str {
        r#"A description of the logical flow of information within the system and across its boundaries, optionally supplemented by diagrams that illustrate these flows."#
    }
    fn schema_id() -> Option<&'static str> {
        Some("#assembly_oscal-ssp_data-flow")
    }
    fn schema_path() -> &'static str {
        "oscal-complete-oscal-ssp:data-flow"
    }
}
