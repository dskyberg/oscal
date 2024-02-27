use serde::{Deserialize, Serialize};
use serde_with::skip_serializing_none;

use crate::{
    metadata::{Link, Property, Remarks},
    SchemaConstraint, StringDatatype, TokenDatatype, URIDatatype,
};

#[skip_serializing_none]
#[derive(Debug, Clone, PartialEq, Deserialize, Serialize)]
#[serde(rename_all = "kebab-case")]
pub struct Facet {
    pub name: TokenDatatype,
    /// "enum": [
    ///    "http://fedramp.gov",
    ///    "http://fedramp.gov/ns/oscal",
    ///    "http://csrc.nist.gov/ns/oscal",
    ///   "http://csrc.nist.gov/ns/oscal/unknown",
    ///   "http://cve.mitre.org",
    ///    "http://www.first.org/cvss/v2.0",
    ///    "http://www.first.org/cvss/v3.0",
    ///    "http://www.first.org/cvss/v3.1"
    // ]
    pub system: URIDatatype,
    pub value: StringDatatype,
    pub props: Option<Vec<Property>>,
    pub links: Option<Vec<Link>>,
    pub remarks: Option<Remarks>,
}

impl SchemaConstraint for Facet {
    fn constraint_title() -> &'static str {
        "Facet"
    }
    fn constraint_description() -> &'static str {
        "An individual characteristic that is part of a larger set produced by the same actor."
    }
    fn constraint_id() -> &'static str {
        "#assembly_oscal-assessment-common_characterization:facet"
    }
    fn schema_path() -> &'static str {
        "oscal-complete-oscal-assessment-common:characterization:facet"
    }
}
