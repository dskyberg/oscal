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
    fn test_fedramp_automation_sap() {
        let json = include_str!("../../fedramp-automation/dist/content/rev5/templates/sap/json/FedRAMP-SAP-OSCAL-Template.json");

        // Deserialize the OSCAL doc
        let result = serde_json::from_str::<OscalDocumentType>(json);
        assert!(result.is_ok());
        let result: OscalDocumentType = result.unwrap();

        let OscalDocumentType::AssessmentPlan(ap) = result else {
            panic!("Not an Assessmentresult");
        };

        let doc = OscalDocumentType::AssessmentPlan(ap);
        let result = serde_json::to_string_pretty(&doc);
        assert!(result.is_ok());
    }

    #[test]
    fn test_oscal_content_sap() {
        let json =
            include_str!("../../oscal-content/examples/ap/json/ifa_assessment-plan-example.json");

        // Deserialize the OSCAL doc
        let result = serde_json::from_str::<OscalDocumentType>(json);
        assert!(result.is_ok());
        let result: OscalDocumentType = result.unwrap();

        let OscalDocumentType::AssessmentPlan(ap) = result else {
            panic!("Not an Assessmentresult");
        };

        let doc = OscalDocumentType::AssessmentPlan(ap);
        let result = serde_json::to_string_pretty(&doc);
        assert!(result.is_ok());
    }

    #[test]
    fn test_fedramp_automation_sar() {
        let json = include_str!("../../fedramp-automation/dist/content/rev5/templates/sar/json/FedRAMP-SAR-OSCAL-Template.json");

        // Deserialize the OSCAL doc
        let result = serde_json::from_str::<OscalDocumentType>(json);
        assert!(result.is_ok());
        let result: OscalDocumentType = result.unwrap();
        let OscalDocumentType::AssessmentResults(ar) = result else {
            panic!("Not a SAR");
        };
        let doc = OscalDocumentType::AssessmentResults(ar);
        let result = serde_json::to_string_pretty(&doc);
        assert!(result.is_ok());
    }

    #[test]
    fn test_oscal_content_sar() {
        let json = include_str!(
            "../../oscal-content/examples/ar/json/ifa_assessment-results-example.json"
        );

        // Deserialize the OSCAL doc
        let result = serde_json::from_str::<OscalDocumentType>(json);
        assert!(result.is_ok());
        let result: OscalDocumentType = result.unwrap();
        let OscalDocumentType::AssessmentResults(ar) = result else {
            panic!("Not a SAR");
        };
        let doc = OscalDocumentType::AssessmentResults(ar);
        let result = serde_json::to_string_pretty(&doc);
        assert!(result.is_ok());
    }

    #[test]
    fn test_fedramp_automation_catalog() {
        let json = include_str!("../../fedramp-automation/dist/content/rev5/baselines/json/FedRAMP_rev5_HIGH-baseline-resolved-profile_catalog.json");

        // Deserialize the OSCAL doc
        let result = serde_json::from_str::<OscalDocumentType>(json);
        assert!(result.is_ok());
        let result: OscalDocumentType = result.unwrap();
        let OscalDocumentType::Catalog(catalog) = result else {
            panic!("Not a SAR");
        };
        let doc = OscalDocumentType::Catalog(catalog);
        let result = serde_json::to_string_pretty(&doc);
        assert!(result.is_ok());
    }

    #[test]
    fn test_oscal_content_catalog() {
        let json = include_str!("../../oscal-content/examples/catalog/json/basic-catalog.json");

        // Deserialize the OSCAL doc
        let result = serde_json::from_str::<OscalDocumentType>(json);
        assert!(result.is_ok());
        let result: OscalDocumentType = result.unwrap();
        let OscalDocumentType::Catalog(catalog) = result else {
            panic!("Not a SAR");
        };
        let doc = OscalDocumentType::Catalog(catalog);
        let result = serde_json::to_string_pretty(&doc);
        assert!(result.is_ok());
    }

    #[test]
    fn test_nist_catalog() {
        let json = include_str!(
            "../../oscal-content/nist.gov/SP800-53/rev5/json/NIST_SP-800-53_rev5_catalog.json"
        );

        // Deserialize the OSCAL doc
        let result = serde_json::from_str::<OscalDocumentType>(json);
        assert!(result.is_ok());
        let result: OscalDocumentType = result.unwrap();
        let OscalDocumentType::Catalog(catalog) = result else {
            panic!("Not a SAR");
        };
        let doc = OscalDocumentType::Catalog(catalog);
        let result = serde_json::to_string_pretty(&doc);
        assert!(result.is_ok());
    }

    #[test]
    fn test_oscal_content_component_definition_component() {
        let json = include_str!(
            "../../oscal-content/examples/component-definition/json/example-component.json"
        );

        // Deserialize the OSCAL doc
        let result = serde_json::from_str::<OscalDocumentType>(json);
        assert!(result.is_ok());

        let result: OscalDocumentType = result.unwrap();
        let OscalDocumentType::ComponentDefinition(component) = result else {
            panic!("Not a Component Definition");
        };
        let doc = OscalDocumentType::ComponentDefinition(component);
        let result = serde_json::to_string_pretty(&doc);
        assert!(result.is_ok());
    }

    #[test]
    fn test_oscal_content_component_definition_component_definition() {
        let json = include_str!(
            "../../oscal-content/examples/component-definition/json/example-component-definition.json"
        );

        // Deserialize the OSCAL doc
        let result = serde_json::from_str::<OscalDocumentType>(json);
        assert!(result.is_ok());

        let result: OscalDocumentType = result.unwrap();
        let OscalDocumentType::ComponentDefinition(component) = result else {
            panic!("Not a Component Definition");
        };
        let doc = OscalDocumentType::ComponentDefinition(component);
        let result = serde_json::to_string_pretty(&doc);
        assert!(result.is_ok());
    }

    #[test]
    fn test_fedramp_automation_poam() {
        let json = include_str!("../../fedramp-automation/dist/content/rev5/templates/poam/json/FedRAMP-POAM-OSCAL-Template.json");

        // Deserialize the OSCAL doc
        let result = serde_json::from_str::<OscalDocumentType>(json);
        assert!(result.is_ok());
        let result: OscalDocumentType = result.unwrap();
        let OscalDocumentType::PlanOfActionAndMilestones(poam) = result else {
            panic!("Not a POAM");
        };
        let doc = OscalDocumentType::PlanOfActionAndMilestones(poam);
        let result = serde_json::to_string_pretty(&doc);
        assert!(result.is_ok());
    }

    #[test]
    fn test_oscal_content_poam() {
        let json = include_str!(
            "../../oscal-content/examples/poam/json/ifa_plan-of-action-and-milestones.json"
        );

        // Deserialize the OSCAL doc
        let result = serde_json::from_str::<OscalDocumentType>(json);
        assert!(result.is_ok());

        let result: OscalDocumentType = result.unwrap();
        let OscalDocumentType::PlanOfActionAndMilestones(poam) = result else {
            panic!("Not a Component Definition");
        };
        let doc = OscalDocumentType::PlanOfActionAndMilestones(poam);
        let result = serde_json::to_string_pretty(&doc);
        assert!(result.is_ok());
    }

    #[test]
    fn test_fedramp_automation_ssp() {
        let json = include_str!("../../fedramp-automation/dist/content/rev5/templates/ssp/json/FedRAMP-SSP-OSCAL-Template.json");

        // Deserialize the OSCAL doc
        let result = serde_json::from_str::<OscalDocumentType>(json);
        assert!(result.is_ok());
        let result: OscalDocumentType = result.unwrap();
        let OscalDocumentType::SystemSecurityPlan(ssp) = result else {
            panic!("Not a SSP");
        };
        let doc = OscalDocumentType::SystemSecurityPlan(ssp);
        let result = serde_json::to_string_pretty(&doc);
        assert!(result.is_ok());
    }

    #[test]
    fn test_oscal_content_ssp() {
        let json = include_str!("../../oscal-content/examples/ssp/json/ssp-example.json");

        // Deserialize the OSCAL doc
        let result = serde_json::from_str::<OscalDocumentType>(json);
        assert!(result.is_ok());

        let result: OscalDocumentType = result.unwrap();
        let OscalDocumentType::SystemSecurityPlan(ssp) = result else {
            panic!("Not a Component Definition");
        };
        let doc = OscalDocumentType::SystemSecurityPlan(ssp);
        let result = serde_json::to_string_pretty(&doc);
        assert!(result.is_ok());
    }

    #[test]
    fn test_oscal_content_ssp_oscal_leveraging() {
        let json =
            include_str!("../../oscal-content/examples/ssp/json/oscal_leveraging-example_ssp.json");

        // Deserialize the OSCAL doc
        let result = serde_json::from_str::<OscalDocumentType>(json);
        assert!(result.is_ok());

        let result: OscalDocumentType = result.unwrap();
        let OscalDocumentType::SystemSecurityPlan(ssp) = result else {
            panic!("Not a Component Definition");
        };
        let doc = OscalDocumentType::SystemSecurityPlan(ssp);
        let result = serde_json::to_string_pretty(&doc);
        assert!(result.is_ok());
    }

    #[test]
    fn test_oscal_content_ssp_oscal_leveraged() {
        let json =
            include_str!("../../oscal-content/examples/ssp/json/oscal_leveraged-example_ssp.json");

        // Deserialize the OSCAL doc
        let result = serde_json::from_str::<OscalDocumentType>(json);
        assert!(result.is_ok());

        let result: OscalDocumentType = result.unwrap();
        let OscalDocumentType::SystemSecurityPlan(ssp) = result else {
            panic!("Not a Component Definition");
        };
        let doc = OscalDocumentType::SystemSecurityPlan(ssp);
        let result = serde_json::to_string_pretty(&doc);
        assert!(result.is_ok());
    }

    #[test]
    fn test_oscal_content_ssp_ifa() {
        let json = include_str!("../../oscal-content/examples/ssp/json/ifa_ssp-example.json");

        // Deserialize the OSCAL doc
        let result = serde_json::from_str::<OscalDocumentType>(json);
        assert!(result.is_ok());

        let result: OscalDocumentType = result.unwrap();
        let OscalDocumentType::SystemSecurityPlan(ssp) = result else {
            panic!("Not a Component Definition");
        };
        let doc = OscalDocumentType::SystemSecurityPlan(ssp);
        let result = serde_json::to_string_pretty(&doc);
        assert!(result.is_ok());
    }
}
