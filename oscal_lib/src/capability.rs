///2013
use serde::{Deserialize, Serialize};
use serde_with::skip_serializing_none;

use crate::{SchemaConstraint, StringDatatype, UUIDDatatype};
#[skip_serializing_none]
#[derive(Debug, Clone, PartialEq, Deserialize, Serialize)]
#[serde(rename_all = "kebab-case")]
pub struct Capability {
    pub uuid: UUIDDatatype,
    pub name: StringDatatype,
    pub description: String,
    pub props: Option<Vec<MetadataProperty>>,
    pub links: Option<Vec<MetadataLink>>,
    pub incorporates_components: Option<Vec<IncorporatesComponent>>,
    pub control_implementations: Option<Vec<ControlImplementation>>,
    pub remarks: Option<MetadataRemarks>,
}

impl SchemaConstraint for Capability {
    fn constraint_title() -> &'static str {
        ""
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
