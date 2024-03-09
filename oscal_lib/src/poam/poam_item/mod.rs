/// File name: ../oscal_lib/src/oscal_complete_oscal_poam/poam_item.rs
/// pub use oscal_complete_oscal_poam::*;
///
/// pub mod oscal_complete_oscal_poam;
use serde::{Deserialize, Serialize};
use serde_with::skip_serializing_none;

use crate::{
    metadata::{link, property, remarks},
    SchemaElement, UUIDDatatype,
};

use self::{
    associated_risk::AssociatedRisk, origin::Origin, related_observation::RelatedObservation,
};

pub mod associated_risk;
pub mod origin;
pub mod related_observation;

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

impl SchemaElement for PoamItem {
    fn schema_title() -> &'static str {
        "POA&M Item"
    }
    fn schema_description() -> &'static str {
        r#"Describes an individual POA&M item."#
    }
    fn schema_id() -> Option<&'static str> {
        Some("#assembly_oscal-poam_poam-item")
    }
    fn schema_path() -> &'static str {
        "oscal-complete-oscal-poam:poam-item"
    }
}
