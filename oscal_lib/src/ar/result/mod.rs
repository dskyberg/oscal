/// File name: ../oscal_lib/src/oscal_complete_oscal_ar/result.rs
/// pub use oscal_complete_oscal_ar::*;
///
/// pub mod oscal_complete_oscal_ar;
use serde::{Deserialize, Serialize};
use serde_with::skip_serializing_none;

use crate::{
    assessment_common::{
        observation::Observation, reviewed_controls::ReviewedControls, risk::Risk,
    },
    metadata::{Link, Property, Remarks},
    DateTimeWithTimezoneDatatype, SchemaElement, UUIDDatatype,
};

use super::finding::Finding;

use self::{
    assessment_log::AssessmentLog, attestation_statement::AttestationStatement,
    local_definitions::LocalDefinitions,
};

pub mod assessment_log;
pub mod attestation_statement;
pub mod local_definitions;

#[skip_serializing_none]
#[derive(Debug, Clone, PartialEq, Deserialize, Serialize)]
#[serde(rename_all = "kebab-case")]
pub struct Result {
    pub uuid: UUIDDatatype,
    pub title: String,
    pub description: String,
    pub start: DateTimeWithTimezoneDatatype,
    pub end: Option<DateTimeWithTimezoneDatatype>,
    pub props: Option<Vec<Property>>,
    pub links: Option<Vec<Link>>,
    pub local_definitions: Option<LocalDefinitions>,
    pub reviewed_controls: ReviewedControls,
    pub attestations: Option<Vec<AttestationStatement>>,
    pub assessment_log: Option<AssessmentLog>,
    pub observations: Option<Vec<Observation>>,
    pub risks: Option<Vec<Risk>>,
    pub findings: Option<Vec<Finding>>,
    pub remarks: Option<Remarks>,
}

impl SchemaElement for Result {
    fn schema_title() -> &'static str {
        "Assessment Result"
    }
    fn schema_description() -> &'static str {
        r#"Used by the assessment results and POA&M. In the assessment results, this identifies all of the assessment observations and findings, initial and residual risks, deviations, and disposition. In the POA&M, this identifies initial and residual risks, deviations, and disposition."#
    }
    fn schema_id() -> Option<&'static str> {
        Some("#assembly_oscal-ar_result")
    }
    fn schema_path() -> &'static str {
        "oscal-complete-oscal-ar:result"
    }
}
