/// File name: ../oscal_lib/src/oscal_complete_oscal_assessment_common/activity.rs
/// pub use oscal_complete_oscal_assessment_common::*;
///
/// pub mod oscal_complete_oscal_assessment_common;
use serde::{Deserialize, Serialize};
use serde_with::skip_serializing_none;

use crate::{
    metadata::{Link, Property, Remarks, ResponsibleRole},
    SchemaConstraint, UUIDDatatype,
};

use crate::assessment_common::reviewed_controls::ReviewedControls;

#[skip_serializing_none]
#[derive(Debug, Clone, PartialEq, Deserialize, Serialize)]
#[serde(rename_all = "kebab-case")]
pub struct Step {
    pub uuid: UUIDDatatype,
    pub title: Option<String>,
    pub description: String,
    pub props: Option<Vec<Property>>,
    pub links: Option<Vec<Link>>,
    pub reviewed_controls: Option<ReviewedControls>,
    pub responsible_roles: Option<Vec<ResponsibleRole>>,
    pub remarks: Option<Remarks>,
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
