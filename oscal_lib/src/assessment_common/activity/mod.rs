/// File name: ../oscal_lib/src/oscal_complete_oscal_assessment_common/activity.rs
/// pub use oscal_complete_oscal_assessment_common::*;
///
/// pub mod oscal_complete_oscal_assessment_common;
use serde::{Deserialize, Serialize};
use serde_with::skip_serializing_none;

use crate::{
    metadata::{Link, Property, Remarks, ResponsibleParty},
    SchemaElement, UUIDDatatype,
};

use super::reviewed_controls::ReviewedControls;

use self::step::Step;

pub mod step;

#[skip_serializing_none]
#[derive(Debug, Clone, PartialEq, Deserialize, Serialize)]
#[serde(rename_all = "kebab-case")]
pub struct Activity {
    pub uuid: UUIDDatatype,
    pub title: Option<String>,
    pub description: String,
    /// "#assembly_oscal-metadata_property"
    pub props: Option<Vec<Property>>,
    /// "#assembly_oscal-metadata_link"
    pub links: Option<Vec<Link>>,
    pub steps: Option<Vec<Step>>,
    // "#assembly_oscal-assessment-common_reviewed-controls"
    pub related_controls: Option<ReviewedControls>,
    pub responsible_parties: Option<Vec<ResponsibleParty>>,
    /// "#field_oscal-metadata_remarks"
    pub remarks: Option<Remarks>,
}

impl SchemaElement for Activity {
    fn schema_title() -> &'static str {
        "Activity"
    }
    fn schema_description() -> &'static str {
        r#"Identifies an assessment or related process that can be performed. In the assessment plan, this is an intended activity which may be associated with an assessment task. In the assessment results, this an activity that was actually performed as part of an assessment."#
    }
    fn schema_id() -> Option<&'static str> {
        Some("#assembly_oscal-assessment-common_activity")
    }
    fn schema_path() -> &'static str {
        "oscal-complete-oscal-assessment-common:activity"
    }
}
