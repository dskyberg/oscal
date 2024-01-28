/// File name: ../oscal_lib/src/oscal_complete_oscal_assessment_common/characterization.rs
/// pub use oscal_complete_oscal_assessment_common::*;
///
/// pub mod oscal_complete_oscal_assessment_common;
use serde::{Deserialize, Serialize};
use serde_with::skip_serializing_none;

use crate::{
    oscal_complete_oscal_metadata::{link, property, remarks},
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
    /// "#assembly_oscal-metadata_property"
    pub props: Option<Vec<property::Property>>,
    /// "#assembly_oscal-metadata_link"
    pub links: Option<Vec<link::Link>>,
    /// "#field_oscal-metadata_remarks"
    pub remarks: Option<remarks::Remarks>,
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

#[skip_serializing_none]
#[derive(Debug, Clone, PartialEq, Deserialize, Serialize)]
#[serde(rename_all = "kebab-case")]
pub struct Characterization {}

impl SchemaConstraint for Characterization {
    fn constraint_title() -> &'static str {
        "Characterization"
    }
    fn constraint_description() -> &'static str {
        r#"A collection of descriptive data about the containing object from a specific origin."#
    }
    fn constraint_id() -> &'static str {
        "#assembly_oscal-assessment-common_characterization"
    }
    fn schema_path() -> &'static str {
        "oscal-complete-oscal-assessment-common:characterization"
    }
}
