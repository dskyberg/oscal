/// File name: ../oscal_lib/src/oscal_complete_oscal_ar/finding.rs
/// pub use oscal_complete_oscal_ar::*;
///
/// pub mod oscal_complete_oscal_ar;
use serde::{Deserialize, Serialize};
use serde_with::skip_serializing_none;

use crate::{
    assessment_common::{finding_target::FindingTarget, origin::Origin},
    metadata::{Link, Property, Remarks},
    SchemaElement, UUIDDatatype,
};

use self::{related_observation::RelatedObservation, related_risk::RelatedRisk};

pub mod related_observation;
pub mod related_risk;

#[skip_serializing_none]
#[derive(Debug, Clone, PartialEq, Deserialize, Serialize)]
#[serde(rename_all = "kebab-case")]
pub struct Finding {
    pub uuid: UUIDDatatype,
    pub title: String,
    pub description: String,
    pub props: Option<Vec<Property>>,
    pub links: Option<Vec<Link>>,
    pub origins: Option<Vec<Origin>>,
    pub target: FindingTarget,
    pub implementation_statement_uuid: Option<UUIDDatatype>,
    pub related_observations: Option<Vec<RelatedObservation>>,
    pub related_risks: Option<Vec<RelatedRisk>>,
    pub remarks: Option<Remarks>,
}

impl SchemaElement for Finding {
    fn schema_title() -> &'static str {
        "Finding"
    }
    fn schema_description() -> &'static str {
        r#"Describes an individual finding."#
    }
    fn schema_id() -> Option<&'static str> {
        Some("#assembly_oscal-ar_finding")
    }
    fn schema_path() -> &'static str {
        "#/definitions/oscal-complete-oscal-ar:finding"
    }
}
