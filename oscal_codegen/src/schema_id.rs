use convert_case::{Case, Casing};

use crate::ParserError;

#[derive(Debug, Clone)]
pub struct SchemaId {
    /// The ID is a local uri, such as `#/definitions/StringDataTyep`
    pub uri_id: bool,
    pub raw: String,
    pub path: Vec<String>,
    pub name: String,
}

impl TryFrom<&str> for SchemaId {
    type Error = ParserError;
    fn try_from(value: &str) -> Result<Self, Self::Error> {
        if value.starts_with("#/") {
            return SchemaId::from_uri_id(value);
        }
        SchemaId::from_embedded_id(value)
    }
}
use std::cmp::Ordering;

impl PartialOrd for SchemaId {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        match self.path.partial_cmp(&other.path) {
            Some(cmp) => match cmp {
                Ordering::Equal => self.name.partial_cmp(&other.name),
                _ => Some(cmp),
            },
            None => None,
        }
    }
}

impl PartialEq for SchemaId {
    fn eq(&self, other: &Self) -> bool {
        self.path == other.path && self.name == other.name
    }
}

impl SchemaId {
    pub fn to_pascal_case(&self) -> String {
        self.name.to_case(Case::Pascal)
    }

    pub fn is_descendant(&self, id: &SchemaId) -> bool {
        if self.path.len() >= id.path.len() {
            return false;
        }
        let a = &self.path[..];
        let b = &id.path[..self.path.len()];
        a == b
    }

    pub fn same_path_and_name(&self) -> bool {
        if self.path.is_empty() {
            return false;
        }
        let last = self.path.last().unwrap();
        &self.name == last
    }

    /// Thurn the path and name into a rust name space
    pub fn rustify(&self) -> String {
        match self.path.is_empty() {
            true => format!("crate::{}", self.to_pascal_case()),
            false => format!("crate::{}::{}", self.path.join("::"), self.to_pascal_case()),
        }
    }

    /// Elements like '#/definitions/<item>
    pub fn from_uri_id(value: &str) -> Result<Self, ParserError> {
        if !value.starts_with('#') {
            // This isn't an embedded id
            log::error!("Value doesn't start with '#': {}", value);
            return Err(ParserError::NotUriId(value.to_string()));
        }

        // Remove the leading '#'.  This is safe, since we've already verified the first
        // char is '#'
        let split_parts: Vec<&str> = value.split('/').collect();

        let mut path: Vec<String> = Vec::new();

        // Now, if there are any parts left...
        for (idx, part) in split_parts.iter().enumerate() {
            if idx == 0 {
                continue;
            }
            path.push(part.replace('-', "_"));
        }

        // First, remove the last path, and make it a name
        let name = path.remove(path.len() - 1);

        Ok(Self {
            uri_id: true,
            raw: value.to_string(),
            path: path.clone(),
            name,
        })
    }

    /// from_embedded
    /// This method converts long id's with embedded namespaces into
    /// a pathed id
    pub fn from_embedded_id(value: &str) -> Result<Self, ParserError> {
        if !value.starts_with('#') {
            // This isn't an embedded id
            log::error!("Value doesn't start with '#': {}", value);
            return Err(ParserError::NotEmbeddedId(value.to_string()));
        }

        // Remove the leading '#'.  This is safe, since we've already verified the first
        // char is '#'
        let line = value.get(1..).unwrap();

        let split_parts: Vec<&str> = line.split('_').collect();

        let mut path: Vec<String> = Vec::new();

        // If there are any parts left...
        for part in split_parts {
            path.push(part.replace('-', "_"));
        }

        // Remove the last path, and make it a name
        let name = path.remove(path.len() - 1);

        Ok(Self {
            uri_id: false,
            raw: value.to_string(),
            path: path.clone(),
            name,
        })
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_it() {
        let id = SchemaId::from_embedded_id("#assembly_oscal-catalog-common_parameter-constraint")
            .expect("oops");
        dbg!(&id);
    }

    #[test]
    fn test_is_descendant() {
        let left =
            SchemaId::try_from("#assembly_oscal-ssp_system-implementation").expect("left failed");
        let right =
            SchemaId::try_from("#assembly_oscal-ssp_system-implementation_leveraged-authorization_leveraged-authorization").expect("right faile");
        assert!(!left.same_path_and_name());
        assert!(left.is_descendant(&right));

        assert!(right.same_path_and_name());
        assert!(!right.is_descendant(&left));

        dbg!(left.rustify());
    }

    #[test]
    fn test_equality() {
        let left =
            SchemaId::try_from("#assembly_oscal-ssp_system-implementation").expect("left failed");
        let right =
            SchemaId::try_from("#assembly_oscal-ssp_system-implementation_leveraged-authorization_leveraged-authorization").expect("right faile");

        assert!(right > left);
    }
}
