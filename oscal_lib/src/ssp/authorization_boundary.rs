/// File name: ../oscal_lib/src/oscal_complete_oscal_ssp/authorization_boundary.rs
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
pub struct AuthorizationBoundary {
    pub description: String,
    pub props: Option<Vec<Property>>,
    pub links: Option<Vec<Link>>,
    pub diagrams: Option<Vec<Diagram>>,
    pub remarks: Option<Remarks>,
}

impl SchemaElement for AuthorizationBoundary {
    fn schema_title() -> &'static str {
        "Authorization Boundary"
    }
    fn schema_description() -> &'static str {
        r#"A description of this system's authorization boundary, optionally supplemented by diagrams that illustrate the authorization boundary."#
    }
    fn schema_id() -> Option<&'static str> {
        Some("#assembly_oscal-ssp_authorization-boundary")
    }
    fn schema_path() -> &'static str {
        "oscal-complete-oscal-ssp:authorization-boundary"
    }
}
