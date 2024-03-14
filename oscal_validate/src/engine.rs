use rayon::prelude::*;
use std::{any::Any, sync::Arc};

use anyhow::Result;

use crate::ValidationResult;

pub type RuleFunction = fn(input: Arc<dyn Any + Sync + Send>) -> Result<ValidationResult>;

#[derive(Debug, Clone)]
pub struct Engine {
    rules: Vec<RuleFunction>,
}

impl Default for Engine {
    fn default() -> Self {
        Self::new()
    }
}

impl Engine {
    pub fn new() -> Self {
        Self { rules: vec![] }
    }

    pub fn run(&self, obj: Arc<dyn Any + Send + Sync>) -> Result<Vec<ValidationResult>> {
        let results: Result<Vec<ValidationResult>> = self
            .rules
            .par_iter()
            .map(|rule| rule(obj.clone()))
            .collect::<Vec<Result<ValidationResult>>>()
            .into_iter()
            .collect();
        results
    }

    pub fn with_rule(&mut self, rule: RuleFunction) -> &mut Self {
        self.rules.push(rule);
        self
    }
}
