use serde::{Deserialize, Serialize};
use serde_with::skip_serializing_none;

use crate::{
    metadata::{Link, Property, Remarks},
    SchemaConstraint, UUIDDatatype,
};

use self::uses_component::UsesComponent;

pub mod uses_component;

#[skip_serializing_none]
#[derive(Debug, Clone, PartialEq, Deserialize, Serialize)]
#[serde(rename_all = "kebab-case")]
pub struct AssessmentPlatform {
    pub uuid: UUIDDatatype,
    pub title: Option<String>,
    pub props: Option<Vec<Property>>,
    pub links: Option<Vec<Link>>,
    pub uses_components: Option<Vec<UsesComponent>>,
    pub remarks: Option<Remarks>,
}

impl SchemaConstraint for AssessmentPlatform {
    fn constraint_title() -> &'static str {
        "Assessment Platform"
    }
    fn constraint_description() -> &'static str {
        "Used to represent the toolset used to perform aspects of the assessment."
    }
    fn constraint_id() -> &'static str {
        "#assembly_oscal-assessment-common_assessment-assets:assessment-platforrm"
    }
    fn schema_path() -> &'static str {
        "oscal-complete-oscal-assessment-common:assessment-assets:assessment-platforrm"
    }
}
