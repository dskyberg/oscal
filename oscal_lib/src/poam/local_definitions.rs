/// File name: ../oscal_lib/src/oscal_complete_oscal_poam/local_definitions.rs
/// pub use oscal_complete_oscal_poam::*;
///
/// pub mod oscal_complete_oscal_poam;
use serde::{Deserialize, Serialize};
use serde_with::skip_serializing_none;

use crate::{
    implementation_common::{inventory_item::InventoryItem, system_component::SystemComponent},
    metadata::Remarks,
    SchemaElement,
};

#[skip_serializing_none]
#[derive(Debug, Clone, PartialEq, Deserialize, Serialize)]
#[serde(rename_all = "kebab-case")]
pub struct LocalDefinitions {
    pub components: Option<Vec<SystemComponent>>,
    pub inventory_items: Option<Vec<InventoryItem>>,
    pub remarks: Option<Remarks>,
}

impl SchemaElement for LocalDefinitions {
    fn schema_title() -> &'static str {
        "Local Definitions"
    }
    fn schema_description() -> &'static str {
        r#"Allows components, and inventory-items to be defined within the POA&M for circumstances where no OSCAL-based SSP exists, or is not delivered with the POA&M."#
    }
    fn schema_id() -> Option<&'static str> {
        Some("#assembly_oscal-poam_local-definitions")
    }
    fn schema_path() -> &'static str {
        "oscal-complete-oscal-poam:local-definitions"
    }
}
