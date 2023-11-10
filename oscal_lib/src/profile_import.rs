use serde::{Deserialize, Serialize};
use serde_with::skip_serializing_none;

use crate::{IncludeAll, SchemaConstraint, SelectControlById, URIReferenceDatatype};
#[skip_serializing_none]
#[derive(Debug, Clone, PartialEq, Deserialize, Serialize)]
#[serde(rename_all = "kebab-case")]
pub struct ProfileImport {
    pub href: URIReferenceDatatype,
    pub include_all: Option<IncludeAll>,
    pub include_controls: Option<Vec<SelectControlById>>,
    pub exclude_controls: Option<Vec<SelectControlById>>,
}

impl SchemaConstraint for ProfileImport {
    fn constraint_title() -> &'static str {
        "Import resource"
    }
    fn constraint_description() -> &'static str {
        "The import designates a catalog or profile to be included (referenced and potentially modified) by this profile. The import also identifies which controls to select using the include-all, include-controls, and exclude-controls directives."
    }
    fn constraint_id() -> &'static str {
        "#assembly_oscal-profile_import"
    }
    fn schema_path() -> &'static str {
        "profile/import"
    }
}
