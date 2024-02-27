/// File name: ../oscal_lib/src/oscal_complete_oscal_assessment_common/observation.rs
/// pub use oscal_complete_oscal_assessment_common::*;
///
/// pub mod oscal_complete_oscal_assessment_common;
use serde::{Deserialize, Serialize};
use serde_with::skip_serializing_none;

use crate::{
    metadata::{Link, Property, Remarks},
    DateTimeWithTimezoneDatatype, SchemaConstraint, StringDatatype, TokenDatatype, UUIDDatatype,
};

use super::{origin::Origin, subject_reference::SubjectReference};

use self::relevant_evidence::RelevantEvidence;

pub mod relevant_evidence;
#[skip_serializing_none]
#[derive(Debug, Clone, PartialEq, Deserialize, Serialize)]
#[serde(rename_all = "kebab-case")]
pub struct Observation {
    pub uuid: UUIDDatatype,
    pub title: Option<String>,
    pub description: String,
    pub props: Option<Vec<Property>>,
    pub links: Option<Vec<Link>>,
    /// "enum": [
    ///    "EXAMINE",
    ///    "INTERVIEW",
    ///    "TEST",
    ///    "UNKNOWN"
    ///]
    pub methods: Vec<StringDatatype>,
    ///"enum": [
    ///    "ssp-statement-issue",
    ///    "control-objective",
    ///    "mitigation",
    ///    "finding",
    ///    "historic"
    ///]
    pub types: Option<Vec<TokenDatatype>>,
    pub origins: Option<Vec<Origin>>,
    pub subjects: Option<Vec<SubjectReference>>,
    pub relevant_evidence: Option<Vec<RelevantEvidence>>,
    pub collected: DateTimeWithTimezoneDatatype,
    pub expires: Option<DateTimeWithTimezoneDatatype>,
    pub remarks: Option<Remarks>,
}

impl SchemaConstraint for Observation {
    fn constraint_title() -> &'static str {
        "Observation"
    }
    fn constraint_description() -> &'static str {
        r#"Describes an individual observation."#
    }
    fn constraint_id() -> &'static str {
        "#assembly_oscal-assessment-common_observation"
    }
    fn schema_path() -> &'static str {
        "oscal-complete-oscal-assessment-common:observation"
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_1() {
        let json = include_str!("../../../../tests/assessment_common/observation1.json");
        let result = serde_json::from_str::<Observation>(json).expect("oops");
        dbg!(result);
    }

    #[test]
    fn test_array() {
        let json = include_str!("../../../../tests/assessment_common/observations.json");
        let result = serde_json::from_str::<Vec<Observation>>(json).expect("oops");
        dbg!(result);
    }
}
