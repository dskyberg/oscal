/// Party Reference
/// A machine-oriented identifier reference to another party defined in metadata. The UUID of the party in the source OSCAL instance is sufficient to reference the data item locally or globally (e.g., in an imported OSCAL instance).
/// $id: #field_oscal-metadata_party-uuid
use crate::definitions::UuidDatatype;

pub type PartyUuid = UuidDatatype;
