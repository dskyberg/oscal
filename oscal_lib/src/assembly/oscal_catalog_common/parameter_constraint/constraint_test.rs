/// Constraint Test
/// A test expression which is expected to be evaluated by a tool.
/// $id: #assembly_oscal-catalog-common_parameter-constraint_constraint-test_constraint-test
use serde::{Deserialize, Serialize};
use serde_with::skip_serializing_none;

use crate::definitions::StringDatatype;
use crate::field::oscal_metadata::Remarks;

#[skip_serializing_none]
#[derive(Debug, Clone, Deserialize, Serialize)]
#[serde(rename_all ="kebab-case")]
pub struct ConstraintTest {
	pub remarks: Option<Remarks>,
	/// Constraint test
	/// A formal (executable) expression of a constraint
	pub expression: StringDatatype,
}
