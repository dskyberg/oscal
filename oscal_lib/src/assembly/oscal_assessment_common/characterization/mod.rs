pub use facet::*;

pub mod facet;
pub mod naming_system;

/// Characterization
/// A collection of descriptive data about the containing object from a specific origin.
/// $id: #assembly_oscal-assessment-common_characterization
use serde::{Deserialize, Serialize};
use serde_with::skip_serializing_none;

use crate::assembly::oscal_assessment_common::Origin;
use crate::assembly::oscal_metadata::Link;
use crate::assembly::oscal_metadata::Property;

#[skip_serializing_none]
#[derive(Debug, Clone, Deserialize, Serialize)]
#[serde(rename_all = "kebab-case")]
pub struct Characterization {
    pub facets: Vec<Facet>,
    pub origin: Origin,
    pub props: Option<Vec<Property>>,
    pub links: Option<Vec<Link>>,
}
