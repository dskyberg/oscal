pub(crate) mod address;
pub(crate) mod address_line;
pub(crate) mod back_matter;
pub(crate) mod document_id;
pub(crate) mod email_address;
pub(crate) mod hash;
pub(crate) mod last_modified;
pub(crate) mod link;
pub(crate) mod location;
pub(crate) mod location_uuid;
pub(crate) mod metadata;
pub(crate) mod oscal_version;
pub(crate) mod party;
pub(crate) mod party_uuid;
pub(crate) mod property;
pub(crate) mod published;
pub(crate) mod remarks;
pub(crate) mod responsible_party;
pub(crate) mod responsible_role;
pub(crate) mod revision;
pub(crate) mod role;
pub(crate) mod telephone_number;
pub(crate) mod version;

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_metadata() {
        let json = include_str!("../../../tests/metadata.json");
        let result =
            serde_json::from_str::<metadata::Metadata>(json).expect("Failed to Deserialize");
        dbg!(result);
    }
}
