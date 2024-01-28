/// File name: ../oscal_lib/src/oscal_complete_oscal_implementation_common/inventory_item.rs
/// pub use oscal_complete_oscal_implementation_common::*;
///
/// pub mod oscal_complete_oscal_implementation_common;
use serde::{Deserialize, Serialize};
use serde_with::skip_serializing_none;

use crate::{
    oscal_complete_oscal_metadata::{link, property, remarks, responsible_party},
    SchemaConstraint, UUIDDatatype,
};

#[skip_serializing_none]
#[derive(Debug, Clone, PartialEq, Deserialize, Serialize)]
#[serde(rename_all = "kebab-case")]
pub struct ImplementedComponent {
    pub component_uuid: UUIDDatatype,
    /// "#assembly_oscal-metadata_property"
    pub props: Option<Vec<property::Property>>,
    /// "#assembly_oscal-metadata_link"
    pub links: Option<Vec<link::Link>>,
    /// "#assembly_oscal-metadata_responsible-party"
    pub responsible_partie: Option<Vec<responsible_party::ResponsibleParty>>,
    /// "#field_oscal-metadata_remarks"
    pub remarks: Option<remarks::Remarks>,
}
impl SchemaConstraint for ImplementedComponent {
    fn constraint_title() -> &'static str {
        "Implemented Component"
    }

    fn constraint_description() -> &'static str {
        "The set of components that are implemented in a given system inventory item."
    }

    fn constraint_id() -> &'static str {
        "#assembly_oscal-implementation-common_inventory-item:implemented-component"
    }
    fn schema_path() -> &'static str {
        "oscal-complete-oscal-implementation-common:inventory-item:implemented-component"
    }
}

#[skip_serializing_none]
#[derive(Debug, Clone, PartialEq, Deserialize, Serialize)]
#[serde(rename_all = "kebab-case")]
pub struct InventoryItem {
    uuid: UUIDDatatype,
    description: String,
    /// "#assembly_oscal-metadata_property"
    props: Option<Vec<property::Property>>,
    /// "#assembly_oscal-metadata_link"
    links: Option<Vec<link::Link>>,
    /// "#assembly_oscal-metadata_responsible-party"
    responsible_parties: Option<Vec<responsible_party::ResponsibleParty>>,

    /// "#field_oscal-metadata_remarks"
    remarks: Option<remarks::Remarks>,
}

impl SchemaConstraint for InventoryItem {
    fn constraint_title() -> &'static str {
        "Inventory Item"
    }
    fn constraint_description() -> &'static str {
        r#"A single managed inventory item within the system."#
    }
    fn constraint_id() -> &'static str {
        "#assembly_oscal-implementation-common_inventory-item"
    }
    fn schema_path() -> &'static str {
        "oscal-complete-oscal-implementation-common:inventory-item"
    }
}
