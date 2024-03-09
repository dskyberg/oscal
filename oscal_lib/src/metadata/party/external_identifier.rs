use serde::{Deserialize, Serialize};
use serde_with::skip_serializing_none;

use crate::{SchemaElement, StringDatatype, URIDatatype};

#[skip_serializing_none]
#[derive(Debug, Clone, PartialEq, Deserialize, Serialize)]
#[serde(rename_all = "kebab-case")]
pub struct ExternalIdentifier {
    /// enum: ["http://orcid.org/"]
    pub schema: URIDatatype,
    pub id: StringDatatype,
}

impl SchemaElement for ExternalIdentifier {
    fn schema_title() -> &'static str {
        "Party External Identifier"
    }

    fn schema_description() -> &'static str {
        "An identifier for a person or organization using a designated scheme. e.g. an Open Researcher and Contributor ID (ORCID)"
    }

    fn schema_id() -> Option<&'static str> {
        Some("#assembly_oscal-metadata_party:external-ids")
    }
    fn schema_path() -> &'static str {
        "oscal-complete-oscal-metadata:party/external-ids"
    }
}
