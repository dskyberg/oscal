use serde::{Deserialize, Serialize};
use serde_with::skip_serializing_none;

use crate::UUIDDatatype;

#[skip_serializing_none]
#[derive(Debug, Clone, PartialEq, Deserialize, Serialize)]
#[serde(rename_all = "kebab-case")]
pub struct RelatedObservation {
    pub observation_uuid: UUIDDatatype,
}
