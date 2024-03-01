/// File name: ../oscal_lib/src/oscal_complete_oscal_ssp/authorization_boundary.rs
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
pub struct AuthorizationBoundary {
    pub description: String,
    pub props: Option<Vec<Property>>,
    pub links: Option<Vec<Link>>,
    pub diagrams: Option<Vec<Diagram>>,
    pub remarks: Option<Remarks>,
}

impl SchemaConstraint for AuthorizationBoundary {
    fn constraint_title() -> &'static str {
        "Authorization Boundary"
    }
    fn constraint_description() -> &'static str {
        r#"A description of this system's authorization boundary, optionally supplemented by diagrams that illustrate the authorization boundary."#
    }
    fn constraint_id() -> &'static str {
        "#assembly_oscal-ssp_authorization-boundary"
    }
    fn schema_path() -> &'static str {
        "oscal-complete-oscal-ssp:authorization-boundary"
    }
}
