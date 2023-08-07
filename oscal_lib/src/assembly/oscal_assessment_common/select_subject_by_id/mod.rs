use subject_universally_unique_identifier_reference_type::*;

mod subject_universally_unique_identifier_reference_type;

/// Select Assessment Subject
/// Identifies a set of assessment subjects to include/exclude by UUID.
/// $id: #assembly_oscal-assessment-common_select-subject-by-id
use serde::{Deserialize, Serialize};
use serde_with::skip_serializing_none;

use crate::assembly::oscal_metadata::Link;
use crate::assembly::oscal_metadata::Property;
use crate::definitions::UuidDatatype;
use crate::field::oscal_metadata::Remarks;

#[skip_serializing_none]
#[derive(Debug, Clone, Deserialize, Serialize)]
#[serde(rename_all = "kebab-case")]
pub struct SelectSubjectById {
    pub props: Option<Vec<Property>>,
    pub links: Option<Vec<Link>>,
    pub remarks: Option<Remarks>,
    /// Subject Universally Unique Identifier Reference
    /// A machine-oriented identifier reference to a component, inventory-item, location, party, user, or resource using it's UUID.
    pub subject_uuid: UuidDatatype,
    /// Subject Universally Unique Identifier Reference Type
    /// Used to indicate the type of object pointed to by the uuid-ref within a subject.
    #[serde(rename = "type")]
    pub _type: SubjectUniversallyUniqueIdentifierReferenceType,
}
