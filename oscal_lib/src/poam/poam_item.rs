/// File name: ../oscal_lib/src/oscal_complete_oscal_poam/poam_item.rs
/// pub use oscal_complete_oscal_poam::*;
///
/// pub mod oscal_complete_oscal_poam;
use serde::{Deserialize, Serialize};
use serde_with::skip_serializing_none;

use crate::{
    assessment_common::origin_actor::OriginActor,
    metadata::{link, property, remarks},
    SchemaConstraint, UUIDDatatype,
};

#[skip_serializing_none]
#[derive(Debug, Clone, PartialEq, Deserialize, Serialize)]
#[serde(rename_all = "kebab-case")]
pub struct Origin {
    pub actors: Vec<OriginActor>,
}

impl SchemaConstraint for Origin {
    fn constraint_title() -> &'static str {
        "Origin"
    }

    fn constraint_description() -> &'static str {
        "Identifies the source of the finding, such as a tool or person."
    }
    fn constraint_id() -> &'static str {
        "#assembly_oscal-poam_poam-item:origin"
    }
    fn schema_path() -> &'static str {
        "oscal-complete-oscal-poam:poam-item:origin"
    }
}

#[skip_serializing_none]
#[derive(Debug, Clone, PartialEq, Deserialize, Serialize)]
#[serde(rename_all = "kebab-case")]
pub struct RelatedObservation {
    observation_uuid: UUIDDatatype,
}

impl SchemaConstraint for RelatedObservation {
    fn constraint_title() -> &'static str {
        "Related Observation"
    }

    fn constraint_description() -> &'static str {
        "Relates the poam-item to a set of referenced observations that were used to determine the finding."
    }
    fn constraint_id() -> &'static str {
        "#assembly_oscal-poam_poam-item:related-observation"
    }
    fn schema_path() -> &'static str {
        "oscal-complete-oscal-poam:poam-item:related-observation"
    }
}

#[skip_serializing_none]
#[derive(Debug, Clone, PartialEq, Deserialize, Serialize)]
#[serde(rename_all = "kebab-case")]
pub struct AssociatedRisk {
    risk_uuid: UUIDDatatype,
}

impl SchemaConstraint for AssociatedRisk {
    fn constraint_title() -> &'static str {
        "Associated Risk"
    }

    fn constraint_description() -> &'static str {
        "Relates the finding to a set of referenced risks that were used to determine the finding."
    }
    fn constraint_id() -> &'static str {
        "#assembly_oscal-poam_poam-item:associated-risk"
    }
    fn schema_path() -> &'static str {
        "oscal-complete-oscal-poam:poam-item:associated-risk"
    }
}

#[skip_serializing_none]
#[derive(Debug, Clone, PartialEq, Deserialize, Serialize)]
#[serde(rename_all = "kebab-case")]
pub struct PoamItem {
    pub uuid: Option<UUIDDatatype>,
    pub title: String,
    pub description: String,
    /// "#assembly_oscal-metadata_property"
    pub props: Option<Vec<property::Property>>,
    /// "#assembly_oscal-metadata_link"
    pub links: Option<Vec<link::Link>>,
    /// "#assembly_oscal-poam_poam-item:origin"
    pub origins: Option<Vec<Origin>>,
    /// "#assembly_oscal-poam_poam-item:related-observation"
    pub related_observations: Option<Vec<RelatedObservation>>,
    /// "#assembly_oscal-poam_poam-item:associated-risk"
    pub related_risks: Option<Vec<AssociatedRisk>>,
    /// "#field_oscal-metadata_remarks"
    pub remarks: Option<remarks::Remarks>,
}

impl SchemaConstraint for PoamItem {
    fn constraint_title() -> &'static str {
        "POA&M Item"
    }
    fn constraint_description() -> &'static str {
        r#"Describes an individual POA&M item."#
    }
    fn constraint_id() -> &'static str {
        "#assembly_oscal-poam_poam-item"
    }
    fn schema_path() -> &'static str {
        "oscal-complete-oscal-poam:poam-item"
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_it() {
        let json = include_str!("test/poam_item.json");
        let result = serde_json::from_str::<PoamItem>(json).expect("oops");
        dbg!(result);
    }
    #[test]
    fn test_array() {
        let json = include_str!("test/poam_items.json");
        let result = serde_json::from_str::<Vec<PoamItem>>(json).expect("oops");
        dbg!(result);
    }
}
