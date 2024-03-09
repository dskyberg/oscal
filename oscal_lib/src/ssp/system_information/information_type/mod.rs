/// File name: ../oscal_lib/src/oscal_complete_oscal_ssp/system_information.rs
/// pub use oscal_complete_oscal_ssp::*;
///
/// pub mod oscal_complete_oscal_ssp;
use serde::{Deserialize, Serialize};
use serde_with::skip_serializing_none;

use crate::{
    metadata::{Link, Property},
    SchemaElement, UUIDDatatype,
};

use self::{
    availability_impact_level::AvailabilityImpactLevel,
    confidentiality_impact_level::ConfidentialityImpactLevel,
    information_type_categorization::InformationTypeCategorization,
    integrity_impact_level::IntegrityImpactLevel,
};

pub mod availability_impact_level;
pub mod confidentiality_impact_level;
pub mod information_type_categorization;
pub mod integrity_impact_level;

#[skip_serializing_none]
#[derive(Debug, Clone, PartialEq, Deserialize, Serialize)]
#[serde(rename_all = "kebab-case")]
pub struct InformationType {
    pub uuid: Option<UUIDDatatype>,
    pub title: String,
    pub description: String,
    pub catagoriations: Option<Vec<InformationTypeCategorization>>,
    pub props: Option<Vec<Property>>,
    pub links: Option<Vec<Link>>,
    pub confidentiality_impact: ConfidentialityImpactLevel,
    pub integrity_impact: IntegrityImpactLevel,
    pub availability_impact: AvailabilityImpactLevel,
}

impl SchemaElement for InformationType {
    fn schema_title() -> &'static str {
        "Information Type"
    }
    fn schema_description() -> &'static str {
        r#"Contains details about one information type that is stored, processed, or transmitted by the system, such as privacy information, and those defined in NIST SP 800-60."#
    }
    fn schema_id() -> Option<&'static str> {
        None
    }
    fn schema_path() -> &'static str {
        "oscal-complete-oscal-ssp:system-information:information-type"
    }
}
