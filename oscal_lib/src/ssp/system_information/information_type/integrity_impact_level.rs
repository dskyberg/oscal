/// File name: ../oscal_lib/src/oscal_complete_oscal_ssp/system_information.rs
/// pub use oscal_complete_oscal_ssp::*;
///
/// pub mod oscal_complete_oscal_ssp;
use serde::{Deserialize, Serialize};
use serde_with::skip_serializing_none;

use crate::{
    metadata::{Link, Property},
    ssp::{adjustment_justification::AdjustmentJustification, base::Base, selected::Selected},
    SchemaConstraint,
};

#[skip_serializing_none]
#[derive(Debug, Clone, PartialEq, Deserialize, Serialize)]
#[serde(rename_all = "kebab-case")]
pub struct IntegrityImpactLevel {
    pub props: Option<Vec<Property>>,
    pub links: Option<Vec<Link>>,
    pub base: Base,
    pub selected: Option<Selected>,
    pub adjustment_justification: Option<AdjustmentJustification>,
}

impl SchemaConstraint for IntegrityImpactLevel {
    fn constraint_title() -> &'static str {
        "Integrity Impact Level"
    }
    fn constraint_description() -> &'static str {
        "The expected level of impact resulting from the unauthorized modification of the described information."
    }
    fn constraint_id() -> &'static str {
        "#assembly_oscal-ssp_system-information_information-type_integrity-impact-level"
    }
    fn schema_path() -> &'static str {
        "oscal-complete-oscal-ssp:system-information:information-type:integrity-impact-level"
    }
}
