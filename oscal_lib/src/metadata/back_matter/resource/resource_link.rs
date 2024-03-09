use serde::{Deserialize, Serialize};
use serde_with::skip_serializing_none;

use crate::{metadata::Hash, SchemaElement, StringDatatype, URIReferenceDatatype};

#[skip_serializing_none]
#[derive(Debug, Clone, PartialEq, Deserialize, Serialize)]
#[serde(rename_all = "kebab-case")]
pub struct ResourceLink {
    href: URIReferenceDatatype,
    media_type: Option<StringDatatype>,
    hashes: Option<Vec<Hash>>,
}
impl SchemaElement for ResourceLink {
    fn schema_title() -> &'static str {
        "Resource link"
    }

    fn schema_description() -> &'static str {
        "A resolvable URI reference to a resource."
    }

    fn schema_id() -> Option<&'static str> {
        None
    }

    fn schema_path() -> &'static str {
        "oscal-complete-oscal-metadata:back-matter/resources/rlink"
    }
}
