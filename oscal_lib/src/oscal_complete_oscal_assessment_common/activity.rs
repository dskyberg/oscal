/// File name: ../oscal_lib/src/oscal_complete_oscal_assessment_common/activity.rs
/// pub use oscal_complete_oscal_assessment_common::*;
///
/// pub mod oscal_complete_oscal_assessment_common;
use serde::{Deserialize, Serialize};
use serde_with::skip_serializing_none;

use crate::{
    oscal_complete_oscal_metadata::{link, property, remarks, responsible_party, responsible_role},
    SchemaConstraint, UUIDDatatype,
};

use super::reviewed_controls;

#[skip_serializing_none]
#[derive(Debug, Clone, PartialEq, Deserialize, Serialize)]
#[serde(rename_all = "kebab-case")]
pub struct Step {
    pub uuid: UUIDDatatype,
    pub title: Option<String>,
    pub description: String,
    /// "#assembly_oscal-metadata_property"
    pub props: Option<Vec<property::Property>>,
    /// "#assembly_oscal-metadata_link"
    pub links: Option<Vec<link::Link>>,
    /// "#assembly_oscal-assessment-common_reviewed-controls"
    pub reviewed_controls: Option<reviewed_controls::ReviewedControls>,
    // "#assembly_oscal-metadata_responsible-role"
    pub responsible_roles: Option<Vec<responsible_role::ResponsibleRole>>,
    /// "#field_oscal-metadata_remarks"
    pub remarks: Option<remarks::Remarks>,
}

impl SchemaConstraint for Step {
    fn constraint_title() -> &'static str {
        "Step"
    }
    fn constraint_description() -> &'static str {
        "Identifies an individual step in a series of steps related to an activity, such as an assessment test or examination procedure."
    }
    fn constraint_id() -> &'static str {
        "#assembly_oscal-assessment-common_activity:step"
    }
    fn schema_path() -> &'static str {
        "oscal-complete-oscal-assessment-common:activity:step"
    }
}

#[skip_serializing_none]
#[derive(Debug, Clone, PartialEq, Deserialize, Serialize)]
#[serde(rename_all = "kebab-case")]
pub struct Activity {
    pub uuid: UUIDDatatype,
    pub title: Option<String>,
    pub description: String,
    /// "#assembly_oscal-metadata_property"
    pub props: Option<Vec<property::Property>>,
    /// "#assembly_oscal-metadata_link"
    pub links: Option<Vec<link::Link>>,
    pub steps: Option<Vec<Step>>,
    // "#assembly_oscal-assessment-common_reviewed-controls"
    pub related_controls: Option<reviewed_controls::ReviewedControls>,
    pub responsible_parties: Option<Vec<responsible_party::ResponsibleParty>>,
    /// "#field_oscal-metadata_remarks"
    pub remarks: Option<remarks::Remarks>,
}

impl SchemaConstraint for Activity {
    fn constraint_title() -> &'static str {
        "Activity"
    }
    fn constraint_description() -> &'static str {
        r#"Identifies an assessment or related process that can be performed. In the assessment plan, this is an intended activity which may be associated with an assessment task. In the assessment results, this an activity that was actually performed as part of an assessment."#
    }
    fn constraint_id() -> &'static str {
        "#assembly_oscal-assessment-common_activity"
    }
    fn schema_path() -> &'static str {
        "oscal-complete-oscal-assessment-common:activity"
    }
}
