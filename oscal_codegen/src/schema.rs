use convert_case::{Case, Casing};
use serde_json::{Map, Value};

use crate::file::{append_txt_file, gen_txt_file, read_file_to_string, remove_file};
use crate::utils::{str_from_map, try_str_from_map};
use crate::{Generate, NameSpace, ParserError, Result, SchemaId};

#[derive(Debug)]
pub struct Schema {
    pub namespaces: NameSpace,
}

fn parse_definitions(namespaces: &mut NameSpace, schema: &Map<String, Value>) -> Result<()> {
    let schema_path_root = "#/definitions";

    let defs = schema
        .get("definitions")
        .ok_or(ParserError::MissingField("definitions".to_string()))?
        .as_object()
        .ok_or(ParserError::ObjectExpected)?;

    for (def_name, value) in defs {
        // An entry in definitions can be referenced by either its $id or its schema path
        let schema_path = format!("{}/{}", schema_path_root, def_name);

        let mut value = value.clone();

        // Every entry in the definitions map must also be a map
        let mut val_as_map = value
            .as_object()
            .ok_or_else(|| ParserError::ObjectExpected)?
            .clone();
        // There are 3 forms that may be present:
        // * type = object
        // * type = string
        // * No type, but has allOf array
        //   * allOf is used to either extend a omponent (includeds a $ref), or constrain the component (includeds enum)
        // * No type, but has a #ref

        if val_as_map.contains_key("allOf") {
            val_as_map = flatten_all_of(&val_as_map, defs);
        };

        let mut raw_id = try_str_from_map("$id", &val_as_map)?.map(|s| s.to_string());

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
    pub fn parse(schema: &Value) -> Result<Self> {
        let mut namespaces = NameSpace::new("src");
        let schema = schema.as_object().ok_or(ParserError::BadSchema)?;
        parse_schema_to_object(&mut namespaces, schema)?;

        parse_definitions(&mut namespaces, schema).map_err(|e| {
            log::error!("Schema failed to parse definitions");
            e
        })?;
        namespaces = namespaces.minify()?;
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
        generate_tests(&mut contents)?;

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
fn flatten_all_of(map: &Map<String, Value>, defs: &Map<String, Value>) -> Map<String, Value> {
    // The return value
    let mut new_map = Map::<String, Value>::new();

    // Safe to unwrap.  The calling function checked.
    let mut value = map.get("allOf").unwrap();

    // allOf as an array
    let array = value
        .as_array()
        .ok_or(ParserError::ArrayExpected)
        .map_err(|e| {
            log::error!("Array Expected");
            e
        })
        .unwrap();

    for mut array_val in array {
        let obj = array_val.as_object().unwrap();
        if obj.contains_key("$ref") {
            let lookup = obj.get("$ref").unwrap().as_str().unwrap();
            let mut ref_map = find_def_ref(lookup, defs)
                .unwrap()
                .as_object()
                .unwrap()
                .clone();
            new_map.append(&mut ref_map);
            continue;
        }
        let mut obj = obj.clone();
        new_map.append(&mut obj);
    }

    new_map
}

fn find_def_ref<'a>(lookup: &str, defs: &'a Map<String, Value>) -> Option<&'a Value> {
    if !lookup.starts_with("#/definitions/") {
        return None;
    }
    let re = regex::Regex::new("^#/definitions/(.*)$").unwrap();
    let result = re.captures(lookup);
    if result.is_none() {
        return None;
    }
    let result = result.unwrap();
    if result.len() != 2 {
        // This should actually be an error
        panic!("Malformed $ref");
    }

    let lookup = &result[1];
    if defs.contains_key(lookup) {
        return Some(defs.get(lookup).unwrap());
    }
    None
}

fn generate_tests(contents: &mut String) -> Result<()> {
    contents.push_str(r##"


#[cfg(test)]
mod tests {
use super::*;

#[test]
fn test_rev4_moderate() {
    let json =
        include_str!("../../tests/fedramp-automation/dist/content/rev4/baselines/json/FedRAMP_rev4_MODERATE-baseline-resolved-profile_catalog.json");
    let oscal = serde_json::from_str::<Oscal>(json);
    assert!(oscal.is_ok());
}

#[test]
fn test_rev4_high() {
    let json =
        include_str!("../../tests/fedramp-automation/dist/content/rev4/baselines/json/FedRAMP_rev4_HIGH-baseline-resolved-profile_catalog.json");
    let oscal = serde_json::from_str::<Oscal>(json);
    assert!(oscal.is_ok());
}


#[test]
fn test_rev5_moderate() {
    let json =
        include_str!("../../tests/fedramp-automation/dist/content/rev5/baselines/json/FedRAMP_rev5_MODERATE-baseline-resolved-profile_catalog.json");
    let oscal = serde_json::from_str::<Oscal>(json);
    assert!(oscal.is_ok());
}

#[test]
fn test_rev5_high() {
    let json =
        include_str!("../../tests/fedramp-automation/dist/content/rev5/baselines/json/FedRAMP_rev5_HIGH-baseline-resolved-profile_catalog.json");
    let oscal = serde_json::from_str::<Oscal>(json);
    assert!(oscal.is_ok());
}

#[test]
fn test_rev5_poam() {
    let json =
        include_str!("../../tests/fedramp-automation/dist/content/rev5/templates/poam/json/FedRAMP-POAM-OSCAL-Template.json");
    let oscal = serde_json::from_str::<Oscal>(json);
    assert!(oscal.is_ok());
}

#[test]
fn test_rev5_sap() {
    let json =
        include_str!("../../tests/fedramp-automation/dist/content/rev5/templates/sap/json/FedRAMP-SAP-OSCAL-Template.json");
    let oscal = serde_json::from_str::<Oscal>(json);
    assert!(oscal.is_ok());
}

#[test]
fn test_rev5_sar() {
    let json =
        include_str!("../../tests/fedramp-automation/dist/content/rev5/templates/sar/FedRAMP-SAR-OSCAL-Template.json");
    let oscal = serde_json::from_str::<Oscal>(json);
    assert!(oscal.is_ok());
}

#[test]
fn test_rev5_ssp() {
    let json =
        include_str!("../../tests/fedramp-automation/dist/content/rev5/templates/ssp/json/FedRAMP-SSP-OSCAL-Template.json");
    let oscal = serde_json::from_str::<Oscal>(json);
    assert!(oscal.is_ok());
}
}

"##,
     );
    Ok(())
}
