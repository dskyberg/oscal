use serde::{Deserialize, Serialize};
use serde_with::skip_serializing_none;

use crate::{
    assessment_common::{activity::Activity, local_objective::LocalObjective},
    implementation_common::{
        inventory_item::InventoryItem, system_component::SystemComponent, system_user::SystemUser,
    },
    metadata::Remarks,
    SchemaConstraint,
};

#[skip_serializing_none]
#[derive(Debug, Clone, PartialEq, Deserialize, Serialize)]
#[serde(rename_all = "kebab-case")]
pub struct LocalDefinitions {
    pub components: Option<Vec<SystemComponent>>,
    pub inventory_items: Option<Vec<InventoryItem>>,
    pub users: Option<Vec<SystemUser>>,
    pub objectives_and_methods: Option<Vec<LocalObjective>>,
    pub activities: Option<Vec<Activity>>,
    pub remarks: Option<Remarks>,
}

impl SchemaConstraint for LocalDefinitions {
    fn constraint_title() -> &'static str {
        "Local Definitions"
    }
    fn constraint_description() -> &'static str {
        ""
    }
    fn constraint_id() -> &'static str {
        ""
    }
    fn schema_path() -> &'static str {
        ""
    }
}
