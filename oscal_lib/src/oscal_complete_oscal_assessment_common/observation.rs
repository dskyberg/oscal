/// File name: ../oscal_lib/src/oscal_complete_oscal_assessment_common/observation.rs
/// pub use oscal_complete_oscal_assessment_common::*;
///
/// pub mod oscal_complete_oscal_assessment_common;
use serde::{Deserialize, Serialize};
use serde_with::skip_serializing_none;

use crate::{
    oscal_complete_oscal_metadata::{link, property, remarks},
    DateTimeWithTimezoneDatatype, SchemaConstraint, StringDatatype, TokenDatatype,
    URIReferenceDatatype, UUIDDatatype,
};

use super::{origin, subject_reference};

#[skip_serializing_none]
#[derive(Debug, Clone, PartialEq, Deserialize, Serialize)]
#[serde(rename_all = "kebab-case")]
pub struct RelevantEvidence {
    pub href: Option<URIReferenceDatatype>,
    pub description: String,
    /// "#assembly_oscal-metadata_property"
    pub props: Option<Vec<property::Property>>,
    /// "#assembly_oscal-metadata_link"
    pub links: Option<Vec<link::Link>>,
    /// "#field_oscal-metadata_remarks"
    pub remarks: Option<remarks::Remarks>,
}
impl SchemaConstraint for RelevantEvidence {
    fn constraint_title() -> &'static str {
        "Relevant Evidence"
    }
    fn constraint_description() -> &'static str {
        "Links this observation to relevant evidence."
    }
    fn constraint_id() -> &'static str {
        "#assembly_oscal-assessment-common_observation:relevant-evidence"
    }
    fn schema_path() -> &'static str {
        "oscal-complete-oscal-assessment-common:observation:relevant-evidence"
    }
}

#[skip_serializing_none]
#[derive(Debug, Clone, PartialEq, Deserialize, Serialize)]
#[serde(rename_all = "kebab-case")]
pub struct Observation {
    pub uuid: UUIDDatatype,
    pub title: Option<String>,
    pub description: String,
    /// "#assembly_oscal-metadata_property"
    pub props: Option<Vec<property::Property>>,
    /// "#assembly_oscal-metadata_link"
    pub links: Option<Vec<link::Link>>,
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
    /// "#assembly_oscal-assessment-common_origin"
    pub origins: Option<Vec<origin::Origin>>,
    /// "#assembly_oscal-assessment-common_subject-reference"
    pub subjects: Option<Vec<subject_reference::SubjectReference>>,
    pub relevant_evidence: Option<Vec<RelevantEvidence>>,
    pub collected: DateTimeWithTimezoneDatatype,
    pub expires: Option<DateTimeWithTimezoneDatatype>,
    /// "#field_oscal-metadata_remarks"
    pub remarks: Option<remarks::Remarks>,
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
        let json = include_str!("test/observation1.json");
        let result = serde_json::from_str::<Observation>(json).expect("oops");
        dbg!(result);
    }

    #[test]
    fn test_array() {
        let json = include_str!("test/observations.json");
        let result = serde_json::from_str::<Vec<Observation>>(json).expect("oops");
        dbg!(result);
    }
}
