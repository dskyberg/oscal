/// File name: ../oscal_lib/src/oscal_complete_oscal_assessment_common/import_ssp.rs
/// pub use oscal_complete_oscal_assessment_common::*;
///
/// pub mod oscal_complete_oscal_assessment_common;
use serde::{Deserialize, Serialize};
use serde_with::skip_serializing_none;

use crate::{oscal_complete_oscal_metadata::remarks, SchemaConstraint, URIReferenceDatatype};

#[skip_serializing_none]
#[derive(Debug, Clone, PartialEq, Deserialize, Serialize)]
#[serde(rename_all = "kebab-case")]
pub struct ImportSsp {
    href: URIReferenceDatatype,
    /// "#field_oscal-metadata_remarks"
    remarks: Option<remarks::Remarks>,
}

impl SchemaConstraint for ImportSsp {
    fn constraint_title() -> &'static str {
        "Import System Security Plan"
    }
    fn constraint_description() -> &'static str {
        r#"Used by the assessment plan and POA&M to import information about the system."#
    }
    fn constraint_id() -> &'static str {
        "#assembly_oscal-assessment-common_import-ssp"
    }
    fn schema_path() -> &'static str {
        "oscal-complete-oscal-assessment-common:import-ssp"
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_it() {
        let json = r##"{
            "href": "#7c30125f-c056-4888-9f1a-7ed1b6a1b638",
            "remarks": "Link the SAP to the SSP.\n\nFedRAMP prefers the path for the SSP be relative to the location of this SAP file. Absolute links will likely not work when FedRAMP tools import the content.\n\nThis may point to a back-matter resource using a URI fragment.\n\nIf no OSCAL-based SSP exists, this must be a URI fragment pointing to a special back-matter resource. The resource must include the `no-oscal-ssp` conformity tag."
          }"##;
        let result = serde_json::from_str::<ImportSsp>(json).expect("oops");
        dbg!(result);
    }
}
