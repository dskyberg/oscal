/// File name: ../oscal_lib/src/oscal_complete_oscal_assessment_common/assessment_assets.rs
/// pub use oscal_complete_oscal_assessment_common::*;
///
/// pub mod oscal_complete_oscal_assessment_common;
use serde::{Deserialize, Serialize};
use serde_with::skip_serializing_none;

use crate::{
    oscal_complete_oscal_metadata::{link, property, remarks, responsible_party},
    SchemaConstraint, UUIDDatatype,
};

#[skip_serializing_none]
#[derive(Debug, Clone, PartialEq, Deserialize, Serialize)]
#[serde(rename_all = "kebab-case")]
pub struct UsesComponent {
    pub component_uuid: UUIDDatatype,
    /// "#assembly_oscal-metadata_property"
    pub props: Option<Vec<property::Property>>,
    /// "#assembly_oscal-metadata_link"
    pub links: Option<Vec<link::Link>>,
    // "#assembly_oscal-metadata_responsible-party"
    pub responsible_parties: Option<Vec<responsible_party::ResponsibleParty>>,
    /// "#field_oscal-metadata_remarks"
    pub remarks: Option<remarks::Remarks>,
}

impl SchemaConstraint for UsesComponent {
    fn constraint_title() -> &'static str {
        "Uses Component"
    }
    fn constraint_description() -> &'static str {
        "The set of components that are used by the assessment platform."
    }
    fn constraint_id() -> &'static str {
        "#assembly_oscal-assessment-common_assessment-assets:assessment-platforrm:uses-component"
    }
    fn schema_path() -> &'static str {
        "oscal-complete-oscal-assessment-common:assessment-assets:assessment-platforrm:uses-component"
    }
}

#[skip_serializing_none]
#[derive(Debug, Clone, PartialEq, Deserialize, Serialize)]
#[serde(rename_all = "kebab-case")]
pub struct AssessmentPlatform {
    pub uuid: UUIDDatatype,
    pub title: Option<String>,
    /// "#assembly_oscal-metadata_property"
    pub props: Option<Vec<property::Property>>,
    /// "#assembly_oscal-metadata_link"
    pub links: Option<Vec<link::Link>>,
    pub uses_components: Option<Vec<UsesComponent>>,
    /// "#field_oscal-metadata_remarks"
    pub remarks: Option<remarks::Remarks>,
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

#[skip_serializing_none]
#[derive(Debug, Clone, PartialEq, Deserialize, Serialize)]
#[serde(rename_all = "kebab-case")]
pub struct AssessmentAssets {}

impl SchemaConstraint for AssessmentAssets {
    fn constraint_title() -> &'static str {
        "Assessment Assets"
    }
    fn constraint_description() -> &'static str {
        r#"Identifies the assets used to perform this assessment, such as the assessment team, scanning tools, and assumptions."#
    }
    fn constraint_id() -> &'static str {
        "#assembly_oscal-assessment-common_assessment-assets"
    }
    fn schema_path() -> &'static str {
        "oscal-complete-oscal-assessment-common:assessment-assets"
    }
}
