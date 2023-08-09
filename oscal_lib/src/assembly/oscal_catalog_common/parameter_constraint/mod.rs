pub use constraint_test::*;


pub mod constraint_test;

/// Constraint
/// A formal or informal expression of a constraint or test
/// $id: #assembly_oscal-catalog-common_parameter-constraint
use serde::{Deserialize, Serialize};
use serde_with::skip_serializing_none;


#[skip_serializing_none]
#[derive(Debug, Clone, Deserialize, Serialize)]
#[serde(rename_all ="kebab-case")]
pub struct ParameterConstraint {
	/// Constraint Description
	/// A textual summary of the constraint to be applied.
	pub description: Option<String>,
	pub tests: Option<Vec<ConstraintTest>>,
}
