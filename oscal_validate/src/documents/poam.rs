use anyhow::Result;
use std::{any::Any, sync::Arc};

use oscal_lib::{oscal_document::OscalDocumentType, poam::PlanOfActionAndMilestones};

use crate::{Engine, Error, ValidationResult};

trait Rulable {
    fn parse_json(json: &str) -> Result<Arc<dyn Any + Sync + Send>>;
    fn from_arc_any(any: Arc<dyn Any + Sync + Send>) -> Result<Arc<Self>>;
    fn to_arc_any(self) -> Arc<dyn Any>;
}

impl Rulable for PlanOfActionAndMilestones {
    fn from_arc_any(any: Arc<dyn Any + Sync + Send>) -> Result<Arc<Self>> {
        any.downcast::<PlanOfActionAndMilestones>()
            .map_err(|_| Error::FailedDowncast("POAM").into())
    }
    fn to_arc_any(self) -> Arc<dyn Any> {
        Arc::new(self)
    }
    fn parse_json(json: &str) -> Result<Arc<dyn Any + Sync + Send>> {
        let oscal_doc = serde_json::from_str::<OscalDocumentType>(json)?;
        match oscal_doc {
            OscalDocumentType::PlanOfActionAndMilestones(poam) => Ok(Arc::new(*poam)),
            _ => Err(Error::General("Not a POAM").into()),
        }
    }
}

pub fn rule_1(any: Arc<dyn Any + Sync + Send>) -> Result<ValidationResult> {
    let _poam = PlanOfActionAndMilestones::from_arc_any(any)?;
    println!("Rule 1 for POAM!");

    Ok(ValidationResult::ok())
}

pub fn rule_2(any: Arc<dyn Any + Sync + Send>) -> Result<ValidationResult> {
    let _poam = PlanOfActionAndMilestones::from_arc_any(any)?;
    println!("Rule 2 for POAM!");

    Ok(ValidationResult::ok())
}

pub fn configure_engine(engine: &mut Engine) {
    engine.with_rule(rule_1);
    engine.with_rule(rule_2);
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::Engine;

    #[test]
    fn test_rules() {
        //let poam = load_poam().expect("failed to load poam");
        let json = include_str!("../../../fedramp-automation/dist/content/rev5/templates/poam/json/FedRAMP-POAM-OSCAL-Template.json");
        let poam = PlanOfActionAndMilestones::parse_json(json).expect("oops");
        let mut engine = Engine::new();
        configure_engine(&mut engine);
        let result = engine.run(poam);
        dbg!(&result);
        assert!(result.is_ok());
    }
}
