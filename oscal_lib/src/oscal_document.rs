use serde::{Deserialize, Serialize};
use serde_with::skip_serializing_none;

use crate::{
    ap::AssessmentPlan, ar::AssessmentResults, catalog::Catalog,
    component_definition::ComponentDefinition,
    poam::plan_of_action_and_milestones::PlanOfActionAndMilestones, profile::Profile,
    ssp::system_security_plan::SystemSecurityPlan,
};

#[skip_serializing_none]
#[derive(Debug, Clone, PartialEq, Deserialize, Serialize)]
#[serde(rename_all = "kebab-case")]
pub enum OscalDocumentType {
    Catalog(Box<Catalog>),
    Profile(Box<Profile>),
    ComponentDefinition(Box<ComponentDefinition>),
    SystemSecurityPlan(Box<SystemSecurityPlan>),
    AssessmentPlan(Box<AssessmentPlan>),
    AssessmentResults(Box<AssessmentResults>),
    PlanOfActionAndMilestones(Box<PlanOfActionAndMilestones>),
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_oscal_doc_sar() {
        let json = include_str!("../../../fedramp-automation/dist/content/rev5/templates/sar/json/FedRAMP-SAR-OSCAL-Template.json");

        // Deserialize the OSCAL doc
        let result = serde_json::from_str::<OscalDocumentType>(json);
        assert!(result.is_ok());
        let result: OscalDocumentType = result.unwrap();

        match result {
            OscalDocumentType::AssessmentResults(poam) => {
                let poam_type = OscalDocumentType::AssessmentResults(poam);
                let result = serde_json::to_string_pretty(&poam_type);
                assert!(result.is_ok());
            }
            _ => panic!("Oops"),
        }
    }

    #[test]
    fn test_oscal_doc_poam() {
        let json = include_str!("../../../fedramp-automation/dist/content/rev5/templates/poam/json/FedRAMP-POAM-OSCAL-Template.json");

        // Deserialize the OSCAL doc
        let result = serde_json::from_str::<OscalDocumentType>(json);
        assert!(result.is_ok());
        let result: OscalDocumentType = result.unwrap();

        match result {
            OscalDocumentType::PlanOfActionAndMilestones(poam) => {
                let poam_type = OscalDocumentType::PlanOfActionAndMilestones(poam);
                let result = serde_json::to_string_pretty(&poam_type);
                assert!(result.is_ok());
            }
            _ => panic!("Oops"),
        }
    }

    #[test]
    fn test_oscal_doc_ssp() {
        let json = include_str!("../../../fedramp-automation/dist/content/rev5/templates/ssp/json/FedRAMP-SSP-OSCAL-Template.json");

        // Deserialize the OSCAL doc
        let result = serde_json::from_str::<OscalDocumentType>(json);
        assert!(result.is_ok());
        let result: OscalDocumentType = result.unwrap();

        match result {
            OscalDocumentType::SystemSecurityPlan(poam) => {
                let poam_type = OscalDocumentType::SystemSecurityPlan(poam);
                let result = serde_json::to_string_pretty(&poam_type);
                assert!(result.is_ok());
            }
            _ => panic!("Oops"),
        }
    }

    #[test]
    fn test_oscal_doc_sap() {
        let json = include_str!("../../../fedramp-automation/dist/content/rev5/templates/sap/json/FedRAMP-SAP-OSCAL-Template.json");

        // Deserialize the OSCAL doc
        let result = serde_json::from_str::<OscalDocumentType>(json);
        assert!(result.is_ok());
        let result: OscalDocumentType = result.unwrap();

        match result {
            OscalDocumentType::AssessmentPlan(poam) => {
                let poam_type = OscalDocumentType::AssessmentPlan(poam);
                let result = serde_json::to_string_pretty(&poam_type);
                assert!(result.is_ok());
            }
            _ => panic!("Oops"),
        }
    }
}
