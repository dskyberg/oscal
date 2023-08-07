/// Location Reference
/// A machine-oriented identifier reference to a location defined in the metadata section of this or another OSCAL instance. The UUID of the location in the source OSCAL instance is sufficient to reference the data item locally or globally (e.g., in an imported OSCAL instance).
/// $id: #field_oscal-metadata_location-uuid
use crate::definitions::UuidDatatype;

pub type LocationUuid = UuidDatatype;
