pub use origin::*;
pub use related_observation::*;
pub use associated_risk::*;


pub mod origin;
pub mod related_observation;
pub mod associated_risk;

/// POA&M Item
/// Describes an individual POA&M item.
/// $id: #assembly_oscal-poam_poam-item
use serde::{Deserialize, Serialize};
use serde_with::skip_serializing_none;

use crate::assembly::oscal_metadata::Link;
use crate::assembly::oscal_metadata::Property;
use crate::definitions::UuidDatatype;
use crate::field::oscal_metadata::Remarks;

#[skip_serializing_none]
#[derive(Debug, Clone, Deserialize, Serialize)]
#[serde(rename_all ="kebab-case")]
pub struct PoamItem {
	pub links: Option<Vec<Link>>,
	pub props: Option<Vec<Property>>,
	pub related_risks: Option<Vec<AssociatedRisk>>,
	/// POA&M Item Universally Unique Identifier
	/// A machine-oriented, globally unique identifier with instance scope that can be used to reference this POA&M item entry in this OSCAL instance. This UUID should be assigned per-subject, which means it should be consistently used to identify the same subject across revisions of the document.
	pub uuid: Option<UuidDatatype>,
	pub remarks: Option<Remarks>,
	pub origins: Option<Vec<Origin>>,
	pub related_observations: Option<Vec<RelatedObservation>>,
	/// POA&M Item Description
	/// A human-readable description of POA&M item.
	pub description: String,
	/// POA&M Item Title
	/// The title or name for this POA&M item .
	pub title: String,
}
