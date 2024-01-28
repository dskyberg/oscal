/// File name: ../oscal_lib/src/oscal_complete_oscal_assessment_common/origin_actor.rs
/// pub use oscal_complete_oscal_assessment_common::*;
///
/// pub mod oscal_complete_oscal_assessment_common;
use serde::{Deserialize, Serialize};
use serde_with::skip_serializing_none;

use crate::{
    oscal_complete_oscal_metadata::{link, property},
    SchemaConstraint, TokenDatatype, UUIDDatatype,
};

#[skip_serializing_none]
#[derive(Debug, Clone, PartialEq, Deserialize, Serialize)]
#[serde(rename_all = "kebab-case")]
pub struct OriginActor {
    /// "enum": [
    ///    "tool",
    ///    "assessment-platform",
    ///    "party"
    ///]
    #[serde(rename = "type")]
    pub _type: TokenDatatype,
    pub actor_uuid: UUIDDatatype,
    pub role_id: Option<TokenDatatype>,
    /// "#assembly_oscal-metadata_property"
    pub props: Option<Vec<property::Property>>,
    /// "#assembly_oscal-metadata_link"
    pub links: Option<Vec<link::Link>>,
}

impl SchemaConstraint for OriginActor {
    fn constraint_title() -> &'static str {
        "Originating Actor"
    }
    fn constraint_description() -> &'static str {
        r#"The actor that produces an observation, a finding, or a risk. One or more actor type can be used to specify a person that is using a tool."#
    }
    fn constraint_id() -> &'static str {
        "#assembly_oscal-assessment-common_origin-actor"
    }
    fn schema_path() -> &'static str {
        "oscal-complete-oscal-assessment-common:origin-actor"
    }
}
