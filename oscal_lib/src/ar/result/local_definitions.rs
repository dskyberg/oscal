use serde::{Deserialize, Serialize};
use serde_with::skip_serializing_none;

use crate::{
    assessment_common::{assessment_assets::AssessmentAssets, task::Task},
    implementation_common::{
        inventory_item::InventoryItem, system_component::SystemComponent, system_user::SystemUser,
    },
    SchemaElement,
};

#[skip_serializing_none]
#[derive(Debug, Clone, PartialEq, Deserialize, Serialize)]
#[serde(rename_all = "kebab-case")]
pub struct LocalDefinitions {
    pub components: Option<Vec<SystemComponent>>,
    pub inventory_items: Option<Vec<InventoryItem>>,
    pub users: Option<Vec<SystemUser>>,
    pub assessment_assets: Option<AssessmentAssets>,
    pub tasks: Option<Vec<Task>>,
}

impl SchemaElement for LocalDefinitions {
    fn schema_title() -> &'static str {
        "Local Definitions"
    }
    fn schema_description() -> &'static str {
        "Used to define data objects that are used in the assessment plan, that do not appear in the referenced SSP."
    }
    fn schema_id() -> Option<&'static str> {
        None
    }
    fn schema_path() -> &'static str {
        "#/definitions/oscal-complete-oscal-ar:result/local-definitions"
    }
}
