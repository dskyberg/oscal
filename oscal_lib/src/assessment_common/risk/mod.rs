/// File name: ../oscal_lib/src/oscal_complete_oscal_assessment_common/risk.rs
/// pub use oscal_complete_oscal_assessment_common::*;
///
/// pub mod oscal_complete_oscal_assessment_common;
use serde::{Deserialize, Serialize};
use serde_with::skip_serializing_none;

use crate::{
    metadata::{Link, Property},
    DateTimeWithTimezoneDatatype, SchemaElement, UUIDDatatype,
};

use super::{
    characterization::Characterization, origin::Origin, response::Response, threat_id::ThreatId,
};

use self::{
    mitigating_factor::MitigatingFactor, related_observation::RelatedObservation, risk_log::RiskLog,
};

pub mod mitigating_factor;
pub mod related_observation;
pub mod risk_log;

#[skip_serializing_none]
#[derive(Debug, Clone, PartialEq, Deserialize, Serialize)]
#[serde(rename_all = "kebab-case")]
pub struct Risk {
    pub uuid: UUIDDatatype,
    pub title: String,
    pub description: String,
    pub statement: String,
    pub props: Option<Vec<Property>>,
    pub links: Option<Vec<Link>>,
    pub status: super::risk_status::RiskStatus,
    pub origins: Option<Vec<Origin>>,
    pub threat_ids: Option<Vec<ThreatId>>,
    pub characterizations: Option<Vec<Characterization>>,
    pub mitigating_factors: Option<Vec<MitigatingFactor>>,
    pub deadline: Option<DateTimeWithTimezoneDatatype>,
    pub remediations: Option<Vec<Response>>,
    pub risk_log: Option<RiskLog>,
    pub related_observations: Option<Vec<RelatedObservation>>,
}

impl SchemaElement for Risk {
    fn schema_title() -> &'static str {
        "Identified Risk"
    }
    fn schema_description() -> &'static str {
        r#"An identified risk."#
    }
    fn schema_id() -> Option<&'static str> {
        Some("#assembly_oscal-assessment-common_risk")
    }
    fn schema_path() -> &'static str {
        "oscal-complete-oscal-assessment-common:risk"
    }
}
