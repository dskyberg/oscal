use serde::{Deserialize, Serialize};
use serde_with::skip_serializing_none;

use crate::{metadata::Hash, SchemaConstraint, StringDatatype, URIReferenceDatatype};

#[skip_serializing_none]
#[derive(Debug, Clone, PartialEq, Deserialize, Serialize)]
#[serde(rename_all = "kebab-case")]
pub struct ResourceLink {
    href: URIReferenceDatatype,
    media_type: Option<StringDatatype>,
    hashes: Option<Vec<Hash>>,
}
impl SchemaConstraint for ResourceLink {
    fn constraint_title() -> &'static str {
        "Resource link"
    }

    fn constraint_description() -> &'static str {
        "A resolvable URI reference to a resource."
    }

    fn constraint_id() -> &'static str {
        "#assembly_oscal-metadata_back-matter:resource:rlink"
    }

    fn schema_path() -> &'static str {
        "oscal-complete-oscal-metadata:back-matter/resources/rlink"
    }
}
