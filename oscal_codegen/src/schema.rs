use convert_case::{Case, Casing};
use serde_json::{Map, Value};

use crate::extensible::Extensible;
use crate::file::{append_txt_file, gen_txt_file, read_file_to_string, remove_file};
use crate::property_type::PropertyType;
use crate::utils::{str_from_map, try_str_from_map};
use crate::{Generate, NameSpace, ParserError, Result, SchemaId};

#[derive(Debug)]
pub struct Schema {
    pub namespaces: NameSpace,
}

fn parse_definitions(namespaces: &mut NameSpace, schema: &Map<String, Value>) -> Result<()> {
    let defs = schema
        .get("definitions")
        .ok_or(ParserError::MissingField("definitions".to_string()))?
        .as_object()
        .ok_or(ParserError::ObjectExpected)?;

    for (def_name, value) in defs {
        let mut value = value.clone();

        // This doesn't work recursively
        let val_as_map = value
            .as_object_mut()
            .ok_or_else(|| ParserError::ObjectExpected)?;

        if val_as_map.contains_key("allOf") {
            flatten_all_of(val_as_map)
        };

        let mut raw_id = try_str_from_map("$id", val_as_map)?.map(|s| s.to_string());

        if raw_id.is_none() {
            // No id.  Let's try to spoof it
            let _id = format!("#/definitions/{}", def_name.to_case(Case::Snake));
            val_as_map.insert("$id".to_string(), Value::String(_id.clone()));
            raw_id = Some(_id);
        }
        let raw_id = raw_id.unwrap();
        let id = SchemaId::try_from(raw_id.as_str())?;
        let ns = namespaces.upsert(&id);
        ns.add_property(&value, &id, Some(def_name)).map_err(|e| {
            log::error!("The property cannot be added to a namespace: {:#?}", value);
            e
        })?;
    }

    Ok(())
}

/// Generate a SchemaObject from the `oneOf` attribute
fn parse_schema_to_object(namespaces: &mut NameSpace, schema: &Map<String, Value>) -> Result<()> {
    let one_ofs = schema
        .get("oneOf")
        .ok_or(ParserError::MissingField("oneOf".to_string()))?
        .as_array()
        .ok_or(ParserError::ArrayExpected)?;

    let mut obj: Map<String, Value> = Map::new();

    let id = "#/oscal";
    obj.insert("$id".into(), id.into());

    let title = str_from_map("$schema", schema)?;
    obj.insert("title".into(), title.into());

    let description = str_from_map("$comment", schema)?;
    obj.insert("description".into(), description.into());
    obj.insert("type".into(), "object".into());

    let mut properties: Map<String, Value> = Map::new();

    for value in one_ofs {
        let mut value = value.clone();
        // This doesn't work recursively
        let val_as_map = value
            .as_object_mut()
            .ok_or_else(|| ParserError::ObjectExpected)?;
        if let Some(props) = val_as_map.get("properties") {
            let mut props = props
                .as_object()
                .ok_or_else(|| ParserError::ObjectExpected)?
                .clone();

            properties.append(&mut props);
        }
    }
    obj.insert("properties".into(), properties.into());

    let id = SchemaId::try_from(id)?;
    namespaces
        .add_property(&obj.into(), &id, None)
        .map_err(|e| {
            log::error!("The schema object cannot be added to a namespace");
            e
        })?;

    Ok(())
}

impl Schema {
    pub fn extend(&mut self, value: &Value) -> Result<()> {
        let obj = value.as_object().ok_or(ParserError::ObjectExpected)?;
        // Look for the "extensions" key
        let extensions_val = obj
            .get("extensions")
            .ok_or(ParserError::MissingField("extensions".to_string()))?;
        let extensions = extensions_val
            .as_array()
            .ok_or(ParserError::ArrayExpected)?;

        for extension in extensions {
            let obj = extension.as_object().ok_or(ParserError::ObjectExpected)?;

            let id_val = str_from_map("$id", obj).map_err(|e| {
                log::error!("Schema extensions must contain an $id");
                e
            })?;
            let id = SchemaId::try_from(id_val).map_err(|e| {
                log::error!("Failed to parse $id for schema extension");
                e
            })?;

            // Find the id in the namespace
            if let Some(prop) = self.find_mut(&id) {
                prop.extend_schema(extension)?;
            }
        }
        Ok(())
    }

    fn find_mut(&mut self, id: &SchemaId) -> Option<&mut PropertyType> {
        match &mut self.namespaces {
            NameSpace::Node { name: _, children } => {
                for child in children {
                    if let Some(id) = child.find_mut(id) {
                        return Some(id);
                    }
                }
                None
            }
            NameSpace::Leaf { name: _, child: _ } => None,
        }
    }

    pub fn parse(schema: &Value) -> Result<Self> {
        let mut namespaces = NameSpace::new("src");
        let schema = schema.as_object().ok_or(ParserError::BadSchema)?;
        parse_schema_to_object(&mut namespaces, schema)?;

        parse_definitions(&mut namespaces, schema).map_err(|e| {
            log::error!("Schema failed to parse definitions");
            e
        })?;
        Ok(Self { namespaces })
    }

    pub fn show(&self) {
        self.namespaces.show(0);
    }

    pub fn generate(&self, path: &str) -> Result<String> {
        self.namespaces.generate(path)?;
        self.generate_errors(path)?;
        self.fixup_lib(path)?;
        Ok(self.namespaces.name().to_string())
    }

    fn fixup_lib(&self, path: &str) -> Result<()> {
        // Read the contents of "oscal.rs"
        let mut contents = read_file_to_string(&format!("{}/src/oscal.rs", path))?;

        contents.push_str(r##"


#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_catalog() {
        let json =
            include_str!("../../tests/fedramp-automation-rev4-baselines/FedRAMP_rev4_HIGH-baseline-resolved-profile_catalog.json");
        let _oscal = serde_json::from_str::<Oscal>(json).expect("failed");
        //assert!(oscal.is_ok());
    }
}

"##,
         );

        append_txt_file(&format!("{}/src/lib.rs", path), &contents)?;
        remove_file(&format!("{}/src/oscal.rs", path))?;
        Ok(())
    }

    fn generate_errors(&self, path: &str) -> Result<()> {
        let contents = r##"use thiserror::Error;
/// Shorthand for standard Result
pub type Result<T> = std::result::Result<T, Box<dyn std::error::Error>>;

/// Crate errors
#[derive(Error, Debug, PartialEq, PartialOrd)]
pub enum OscalError {
    #[error("Token error: {0}")]
    BadNCName(String),
}
"##;
        gen_txt_file(&format!("{}/src/error.rs", path), contents)?;

        Ok(())
    }
}

/// Flatten  a map with an allOf array
/// So an array such as:
/// ```json
/// {
///   "allOf": [
///     {
///       "$ref": "#/definitions/StringDatatype"
///     },
///     {
///       "type": "string",
///       "format": "email",
///       "pattern": "^.+@.+$"
///     }
///   ]
/// }
/// ```
/// Becomes:
///
/// ```json
/// {
///   "$ref": "#/definitions/StringDatatype"
///   "type": "string",
///   "format": "email",
///   "pattern": "^.+@.+$"
/// }
/// ```
fn flatten_all_of(map: &mut Map<String, Value>) {
    if !map.contains_key("allOf") {
        return;
    }
    let mut value = map.remove("allOf").unwrap();

    let array = value
        .as_array_mut()
        .ok_or(ParserError::ArrayExpected)
        .map_err(|e| {
            log::error!("Array Expected");
            e
        })
        .unwrap()
        .clone();
    for mut array_val in array {
        let mut obj = array_val.as_object_mut().unwrap().clone();
        map.append(&mut obj);
    }
}
