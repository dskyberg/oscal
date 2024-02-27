/// File name: ../oscal_lib/src/oscal_complete_oscal_assessment_common/characterization.rs
/// pub use oscal_complete_oscal_assessment_common::*;
///
/// pub mod oscal_complete_oscal_assessment_common;
use serde::{Deserialize, Serialize};
use serde_with::skip_serializing_none;

use crate::{
    metadata::{Link, Property},
    SchemaConstraint,
};

use super::origin::Origin;

use self::facet::Facet;

pub mod facet;

#[skip_serializing_none]
#[derive(Debug, Clone, PartialEq, Deserialize, Serialize)]
#[serde(rename_all = "kebab-case")]
pub struct Characterization {
    pub props: Option<Vec<Property>>,
    pub links: Option<Vec<Link>>,
    pub origin: Origin,
    pub facets: Vec<Facet>,
}

impl SchemaConstraint for Characterization {
    fn constraint_title() -> &'static str {
        "Characterization"
    }
    fn constraint_description() -> &'static str {
        r#"A collection of descriptive data about the containing object from a specific origin."#
    }
    fn constraint_id() -> &'static str {
        "#assembly_oscal-assessment-common_characterization"
    }
    fn schema_path() -> &'static str {
        "oscal-complete-oscal-assessment-common:characterization"
    }
}
