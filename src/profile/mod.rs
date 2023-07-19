use serde::{Deserialize, Serialize};
use serde_with::skip_serializing_none;
use uuid::Uuid;

use crate::global::{BackMatter, Imports, Metadata};

use merge::Merge;
use modify::Modify;

mod merge;
mod modify;

#[skip_serializing_none]
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Profile {
    pub uuid: Uuid,
    pub metadat: Metadata,
    pub imports: Imports,
    pub merge: Option<Merge>,
    pub modify: Option<Modify>,
    #[serde(rename = "back-matter")]
    pub back_matter: Option<BackMatter>,
}
