/// File name: ../oscal_lib/src/oscal_complete_oscal_implementation_common/inventory_item.rs
/// pub use oscal_complete_oscal_implementation_common::*;
///
/// pub mod oscal_complete_oscal_implementation_common;
use serde::{Deserialize, Serialize};
use serde_with::skip_serializing_none;

use crate::{
    metadata::{Link, Property, Remarks, ResponsibleParty},
    SchemaElement, UUIDDatatype,
};

use self::implemented_component::ImplementedComponent;

pub mod implemented_component;

#[skip_serializing_none]
#[derive(Debug, Clone, PartialEq, Deserialize, Serialize)]
#[serde(rename_all = "kebab-case")]
pub struct InventoryItem {
    pub uuid: UUIDDatatype,
    pub description: String,
    pub props: Option<Vec<Property>>,
    pub links: Option<Vec<Link>>,
    pub responsible_parties: Option<Vec<ResponsibleParty>>,
    pub implemented_components: Option<Vec<ImplementedComponent>>,
    pub remarks: Option<Remarks>,
}

impl SchemaElement for InventoryItem {
    fn schema_title() -> &'static str {
        "Inventory Item"
    }
    fn schema_description() -> &'static str {
        r#"A single managed inventory item within the system."#
    }
    fn schema_id() -> Option<&'static str> {
        Some("#assembly_oscal-implementation-common_inventory-item")
    }
    fn schema_path() -> &'static str {
        "oscal-complete-oscal-implementation-common:inventory-item"
    }
}
