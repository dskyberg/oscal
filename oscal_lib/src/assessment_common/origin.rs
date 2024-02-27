/// File name: ../oscal_lib/src/oscal_complete_oscal_assessment_common/origin.rs
/// pub use oscal_complete_oscal_assessment_common::*;
///
/// pub mod oscal_complete_oscal_assessment_common;
use serde::{Deserialize, Serialize};
use serde_with::skip_serializing_none;

use crate::SchemaConstraint;

use super::{origin_actor, related_task::RelatedTask};

#[skip_serializing_none]
#[derive(Debug, Clone, PartialEq, Deserialize, Serialize)]
#[serde(rename_all = "kebab-case")]
pub struct Origin {
    pub actors: Vec<origin_actor::OriginActor>,
    pub related_tasks: Option<Vec<RelatedTask>>,
}

impl SchemaConstraint for Origin {
    fn constraint_title() -> &'static str {
        "Origin"
    }
    fn constraint_description() -> &'static str {
        r#"Identifies the source of the finding, such as a tool, interviewed person, or activity."#
    }
    fn constraint_id() -> &'static str {
        "#assembly_oscal-assessment-common_origin"
    }
    fn schema_path() -> &'static str {
        "oscal-complete-oscal-assessment-common:origin"
    }
}
