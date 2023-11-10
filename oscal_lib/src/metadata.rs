// 425
use serde::{Deserialize, Serialize};
use serde_with::skip_serializing_none;

use crate::{
    DateTimeWithTimezoneDatatype, MetadataDocumentId, MetadataLink, MetadataLocation,
    MetadataParty, MetadataProperty, MetadataRemarks, MetadataResponsibleParty, MetadataRevision,
    MetadataRole, SchemaConstraint, StringDatatype,
};

#[skip_serializing_none]
#[derive(Debug, Clone, PartialEq, Deserialize, Serialize)]
#[serde(rename_all = "kebab-case")]
pub struct Metadata {
    pub title: String,
    pub published: Option<DateTimeWithTimezoneDatatype>,
    pub last_modified: DateTimeWithTimezoneDatatype,
    pub version: StringDatatype,
    pub oscal_version: StringDatatype,
    pub revisions: Option<Vec<MetadataRevision>>,
    pub document_ids: Option<Vec<MetadataDocumentId>>,
    pub props: Option<Vec<MetadataProperty>>,
    pub links: Option<Vec<MetadataLink>>,
    pub rols: Option<Vec<MetadataRole>>,
    pub locations: Option<Vec<MetadataLocation>>,
    pub parties: Option<Vec<MetadataParty>>,
    pub responsible_parties: Option<Vec<MetadataResponsibleParty>>,
    pub remarks: Option<MetadataRemarks>,
}

impl SchemaConstraint for Metadata {
    fn constraint_title() -> &'static str {
        "Publication metadata"
    }
    fn constraint_description() -> &'static str {
        "Provides information about the publication and availability of the containing document."
    }
    fn schema_path() -> &'static str {
        "metadata"
    }
    fn constraint_id() -> &'static str {
        "#assembly_oscal-metadata_metadata"
    }
}
