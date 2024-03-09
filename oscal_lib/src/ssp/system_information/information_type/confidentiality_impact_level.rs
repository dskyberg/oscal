/// File name: ../oscal_lib/src/oscal_complete_oscal_ssp/system_information.rs
/// pub use oscal_complete_oscal_ssp::*;
///
/// pub mod oscal_complete_oscal_ssp;
use serde::{Deserialize, Serialize};
use serde_with::skip_serializing_none;

use crate::{
    metadata::{Link, Property},
    ssp::{adjustment_justification::AdjustmentJustification, base::Base, selected::Selected},
    SchemaElement,
};

#[skip_serializing_none]
#[derive(Debug, Clone, PartialEq, Deserialize, Serialize)]
#[serde(rename_all = "kebab-case")]
pub struct ConfidentialityImpactLevel {
    pub props: Option<Vec<Property>>,
    pub links: Option<Vec<Link>>,
    pub base: Base,
    pub selected: Option<Selected>,
    pub adjustment_justification: Option<AdjustmentJustification>,
}

impl SchemaElement for ConfidentialityImpactLevel {
    fn schema_title() -> &'static str {
        "Confidentiality Impact Level"
    }
    fn schema_description() -> &'static str {
        "The expected level of impact resulting from the unauthorized disclosure of the described information."
    }
    fn schema_id() -> Option<&'static str> {
        None
    }
    fn schema_path() -> &'static str {
        "oscal-complete-oscal-ssp:system-information:information-type:confidentiality-impact-level"
    }
}
