/// File name: ../oscal_lib/src/oscal_complete_oscal_assessment_common/risk.rs
/// pub use oscal_complete_oscal_assessment_common::*;
///
/// pub mod oscal_complete_oscal_assessment_common;
use serde::{Deserialize, Serialize};
use serde_with::skip_serializing_none;

use crate::{
    oscal_complete_oscal_metadata::{link, property, remarks},
    DateTimeWithTimezoneDatatype, SchemaConstraint, UUIDDatatype,
};

use super::{
    characterization, logged_by, origin, related_task, response, risk_status, subject_reference,
    threat_id,
};

#[skip_serializing_none]
#[derive(Debug, Clone, PartialEq, Deserialize, Serialize)]
#[serde(rename_all = "kebab-case")]
pub struct MitigatingFactor {
    pub uuid: UUIDDatatype,
    pub implementation_uuid: Option<UUIDDatatype>,
    pub description: String,
    /// "#assembly_oscal-metadata_property"
    pub props: Option<Vec<property::Property>>,
    /// "#assembly_oscal-metadata_link"
    pub links: Option<Vec<link::Link>>,
    /// TODO: "#assembly_oscal-assessment-common_subject-reference"
    pub subjects: Option<Vec<subject_reference::SubjectReference>>,
}

impl SchemaConstraint for MitigatingFactor {
    fn constraint_title() -> &'static str {
        "Mitigating Factor"
    }
    fn constraint_description() -> &'static str {
        "Describes an existing mitigating factor that may affect the overall determination of the risk, with an optional link to an implementation statement in the SSP."
    }
    fn constraint_id() -> &'static str {
        "#assembly_oscal-assessment-common_risk:mitigating-factor"
    }
    fn schema_path() -> &'static str {
        "oscal-complete-oscal-assessment-common:risk:mitigating-factor"
    }
}

#[skip_serializing_none]
#[derive(Debug, Clone, PartialEq, Deserialize, Serialize)]
#[serde(rename_all = "kebab-case")]
pub struct RiskResponseReference {
    response_uuid: UUIDDatatype,
    /// "#assembly_oscal-metadata_property"
    pub props: Option<Vec<property::Property>>,
    /// "#assembly_oscal-metadata_link"
    pub links: Option<Vec<link::Link>>,
    /// TODO: "#assembly_oscal-assessment-common_related-task"
    pub related_tasks: Option<Vec<related_task::RelatedTask>>,
    /// "#field_oscal-metadata_remarks"
    pub remarks: Option<remarks::Remarks>,
}

impl SchemaConstraint for RiskResponseReference {
    fn constraint_title() -> &'static str {
        "Risk Response Reference"
    }
    fn constraint_description() -> &'static str {
        "Identifies an individual risk response that this log entry is for."
    }
    fn constraint_id() -> &'static str {
        "#assembly_oscal-assessment-common_risk:risk-log:risk-log-entry:related-responses"
    }
    fn schema_path() -> &'static str {
        "oscal-complete-oscal-assessment-common:risk:risk-log:risk-log-entry:related-responses"
    }
}

#[skip_serializing_none]
#[derive(Debug, Clone, PartialEq, Deserialize, Serialize)]
#[serde(rename_all = "kebab-case")]
pub struct RiskLogEntry {
    uuid: UUIDDatatype,
    title: Option<String>,
    description: Option<String>,
    start: DateTimeWithTimezoneDatatype,
    end: Option<DateTimeWithTimezoneDatatype>,
    /// "#assembly_oscal-metadata_property"
    pub props: Option<Vec<property::Property>>,
    /// "#assembly_oscal-metadata_link"
    pub links: Option<Vec<link::Link>>,
    /// TODO: "#assembly_oscal-assessment-common_logged-by"
    pub logged_by: Option<logged_by::LoggedBy>,
    /// TODO: "#field_oscal-assessment-common_risk-status"
    pub status_change: Option<risk_status::RiskStatus>,
    pub related_responses: Option<Vec<RiskResponseReference>>,
    /// "#field_oscal-metadata_remarks"
    pub remarks: Option<remarks::Remarks>,
}

impl SchemaConstraint for RiskLogEntry {
    fn constraint_title() -> &'static str {
        "Risk Log Entry"
    }
    fn constraint_description() -> &'static str {
        "Identifies an individual risk response that occurred as part of managing an identified risk."
    }
    fn constraint_id() -> &'static str {
        "#assembly_oscal-assessment-common_risk:risk-log:risk-log-entry"
    }
    fn schema_path() -> &'static str {
        "oscal-complete-oscal-assessment-common:risk:risk-log:risk-log-entry"
    }
}

#[skip_serializing_none]
#[derive(Debug, Clone, PartialEq, Deserialize, Serialize)]
#[serde(rename_all = "kebab-case")]
pub struct RiskLog {
    pub entries: Vec<RiskLogEntry>,
}

impl SchemaConstraint for RiskLog {
    fn constraint_title() -> &'static str {
        "Risk Log"
    }
    fn constraint_description() -> &'static str {
        "A log of all risk-related tasks taken."
    }
    fn constraint_id() -> &'static str {
        "#assembly_oscal-assessment-common_risk:risk-log"
    }
    fn schema_path() -> &'static str {
        "oscal-complete-oscal-assessment-common:risk:risk-log"
    }
}

#[derive(Debug, Clone, PartialEq, Deserialize, Serialize)]
#[serde(rename_all = "kebab-case")]
pub struct RelatedObservation {
    pub observation_uuid: UUIDDatatype,
}
impl SchemaConstraint for RelatedObservation {
    fn constraint_title() -> &'static str {
        "Related Observation"
    }
    fn constraint_description() -> &'static str {
        "Relates the finding to a set of referenced observations that were used to determine the finding."
    }
    fn constraint_id() -> &'static str {
        "#assembly_oscal-assessment-common_risk:related-observation"
    }
    fn schema_path() -> &'static str {
        "oscal-complete-oscal-assessment-common:risk:related-observation"
    }
}

#[skip_serializing_none]
#[derive(Debug, Clone, PartialEq, Deserialize, Serialize)]
#[serde(rename_all = "kebab-case")]
pub struct Risk {
    pub uuid: UUIDDatatype,
    pub title: String,
    pub description: String,
    pub statement: String,
    pub props: Option<Vec<property::Property>>,
    pub links: Option<Vec<link::Link>>,
    pub status: super::risk_status::RiskStatus,
    /// "#assembly_oscal-assessment-common_origin"
    pub origins: Option<Vec<origin::Origin>>,
    /// "#field_oscal-assessment-common_threat-id"
    pub threat_ids: Option<Vec<threat_id::ThreatId>>,
    /// "#assembly_oscal-assessment-common_characterization"
    pub characterizations: Option<Vec<characterization::Characterization>>,
    pub mitigating_factors: Option<Vec<MitigatingFactor>>,
    pub deadline: Option<DateTimeWithTimezoneDatatype>,
    /// "#assembly_oscal-assessment-common_response"
    pub remediations: Option<Vec<response::Response>>,
    pub risk_log: Option<RiskLog>,
    pub related_observations: Option<Vec<RelatedObservation>>,
}

impl SchemaConstraint for Risk {
    fn constraint_title() -> &'static str {
        "Identified Risk"
    }
    fn constraint_description() -> &'static str {
        r#"An identified risk."#
    }
    fn constraint_id() -> &'static str {
        "#assembly_oscal-assessment-common_risk"
    }
    fn schema_path() -> &'static str {
        "oscal-complete-oscal-assessment-common:risk"
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_1() {
        let json = include_str!("test/risk1.json");
        let result = serde_json::from_str::<Risk>(json).expect("oops");
        dbg!(result);
    }

    #[test]
    fn test_array() {
        let json = include_str!("test/risks.json");
        let result = serde_json::from_str::<Vec<Risk>>(json).expect("oops");
        dbg!(result);
    }
}
