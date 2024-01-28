use serde::{Deserialize, Serialize};
use serde_with::skip_serializing_none;

use crate::oscal_complete_oscal_poam::plan_of_action_and_milestones::PlanOfActionAndMilestones;

#[skip_serializing_none]
#[derive(Debug, Clone, PartialEq, Deserialize, Serialize)]
#[serde(rename_all = "kebab-case")]
pub struct OscalDocument {
    pub plan_of_action_and_milestones: Option<PlanOfActionAndMilestones>,
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_poam() {
        let json = include_str!("../../tests/fedramp-automation/dist/content/rev5/templates/poam/json/FedRAMP-POAM-OSCAL-Template.json");
        let result = serde_json::from_str::<OscalDocument>(json).expect("oops");
        dbg!(result);
    }
}
