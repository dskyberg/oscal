/// File name: ../oscal_lib/src/oscal_complete_oscal_assessment_common/local_objective.rs
/// pub use oscal_complete_oscal_assessment_common::*;
///
/// pub mod oscal_complete_oscal_assessment_common;
use serde::{Deserialize, Serialize};
use serde_with::skip_serializing_none;

use crate::{
    oscal_complete_oscal_catalog_common::part,
    oscal_complete_oscal_metadata::{link, property},
    SchemaConstraint, TokenDatatype,
};

#[skip_serializing_none]
#[derive(Debug, Clone, PartialEq, Deserialize, Serialize)]
#[serde(rename_all = "kebab-case")]
pub struct LocalObjective {
    pub control_id: TokenDatatype,
    pub description: Option<String>,
    /// "#assembly_oscal-metadata_property"
    pub props: Option<Vec<property::Property>>,
    /// "#assembly_oscal-metadata_link"
    pub links: Option<Vec<link::Link>>,
    // "#assembly_oscal-catalog-common_part"
    pub parts: Vec<part::Part>,
}

impl SchemaConstraint for LocalObjective {
    fn constraint_title() -> &'static str {
        "Assessment-Specific Control Objective"
    }
    fn constraint_description() -> &'static str {
        r#"A local definition of a control objective for this assessment. Uses catalog syntax for control objective and assessment actions."#
    }
    fn constraint_id() -> &'static str {
        "#assembly_oscal-assessment-common_local-objective"
    }
    fn schema_path() -> &'static str {
        "oscal-complete-oscal-assessment-common:local-objective"
    }
}
