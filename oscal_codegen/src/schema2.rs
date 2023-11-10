use std::collections::HashMap;

use convert_case::{Case, Casing};
use serde::Serialize;
use serde_json::{Map, Value};

use crate::file::{append_txt_file, gen_txt_file, read_file_to_string, remove_file};
use crate::utils::{obj_from_map, str_from_map, try_obj_from_map, try_str_from_map};
use crate::{ParserError, Result};

#[derive(Debug, Serialize)]
pub struct Schema {
    pub objects: Vec<SchemaObject>,
    pub data_types: Vec<String>,
    pub ns: HashMap<String, String>,
}

impl Schema {
    pub fn new() -> Self {
        Self {
            objects: Vec::new(),
            data_types: Vec::new(),
            ns: HashMap::new(),
        }
    }

    pub fn parse(schema: &Value) -> Result<Self> {
        let mut this = Self::new();
        let schema = schema.as_object().ok_or(ParserError::BadSchema)?;

        this.parse_definitions(schema).map_err(|e| {
            log::error!("Schema failed to parse definitions");
            e
        })?;

        Ok(this)
    }

    pub fn show(&self) {
        println!("{}", self.objects.last().unwrap().generate());
    }

    pub fn generate(&self, path: &str) -> Result<String> {
        Ok("".to_string())
    }

    fn parse_definitions(&mut self, schema: &Map<String, Value>) -> Result<()> {
        let mut dict: Vec<String> = Vec::new();
        let mut collisions: Vec<String> = Vec::new();

        let schema_root = "#/definitions";
        let defs = obj_from_map("definitions", schema)
            .map_err(|_| ParserError::MissingField("definitions".to_string()))?;

        // First pass to look for name collisions
        for (def_name, value) in defs {
            // If the key contains a colon, get the part after the colon.
            let name = match def_name.contains(':') {
                true => def_name.split(':').collect::<Vec<&str>>()[1].to_case(Case::Snake),
                false => def_name.clone(),
            };
            if dict.contains(&name) {
                println!(" ***** BALLS!!!  {} is already used!!", &name);
                collisions.push(name.clone());
            } else {
                dict.push(name.clone());
            }
        }

        for (def_name, value) in defs {
            // for stuff withot an $id, use the relative schema path
            let schema_path = format!("{}/{}", &schema_root, def_name);

            // If the key contains a colon, get the part after the colon.
            let name = match def_name.contains(':') {
                true => def_name.split(':').collect::<Vec<&str>>()[1].to_case(Case::Snake),
                false => def_name.clone(),
            };

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

            if let Some(type_) = try_str_from_map("type", &val_as_map)? {
                match type_ {
                    "object" => {
                        let mut object = SchemaObject::from_map(
                            &schema_path,
                            &name,
                            collisions.contains(&name),
                            &val_as_map,
                        )?;
                        if object.id.is_some() {
                            // Make sure the nbame hasn't already been used.
                            let id = object.id.clone().unwrap();
                            self.ns.insert(id, name.clone());
                        } else {
                            self.ns.insert(schema_path.clone(), name.clone());
                        }
                        self.objects.push(object);
                    }
                    "string" => {
                        println!("String: {}", &schema_path);
                    }
                    _ => {}
                }
            } else {
                println!("No type: {}", &schema_path);
            }

            /*
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
            */
        }

        Ok(())
    }
}

#[derive(Debug, Serialize)]
pub struct SchemaObject {
    pub path: String,
    pub name: String, // snake_case name
    pub title: Option<String>,
    pub id: Option<String>,
    pub description: Option<String>,
    pub is_collision: bool,
}

impl SchemaObject {
    pub fn from_map(
        path: &str,
        name: &str,
        is_collision: bool,
        map: &Map<String, Value>,
    ) -> Result<Self> {
        let title = try_str_from_map("title", map)?.map(|s| s.to_string());
        let id = try_str_from_map("$id", map)?.map(|s| s.to_string());
        let description = try_str_from_map("description", map)?.map(|s| s.to_string());

        Ok(Self {
            path: path.to_string(),
            name: name.to_string(),
            title,
            id,
            description,
            is_collision,
        })
    }

    pub fn rust_name(&self) -> String {
        match self.is_collision {
            true => {
                // For instance: "#assembly_oscal-component-definition_statement"
                let parts = self.id.as_ref().unwrap().split('_').collect::<Vec<&str>>();
                let prefix = parts[1];
                format!("{}_{}", prefix, &self.name).to_case(Case::Pascal)
            }
            false => self.name.to_case(Case::Pascal),
        }
    }

    pub fn generate(&self) -> String {
        let name = self.rust_name();
        let snake_name = name.to_case(Case::Snake);
        let empty = String::from("");
        format!(
            r##"/// File name: {}.rs
/// pub use {}::*;
///
/// pub mod {};
use serde::{{Deserialize, Serialize}};
use serde_with::skip_serializing_none;

use crate::{{SchemaConstraint, StringDatatype, UUIDDatatype}};
#[skip_serializing_none]
#[derive(Debug, Clone, PartialEq, Deserialize, Serialize)]
#[serde(rename_all = "kebab-case")]
pub struct {} {{
}}

impl SchemaConstraint for {} {{
    fn constraint_title() -> &'static str {{
        "{}"
    }}
    fn constraint_description() -> &'static str {{
        "{}"
    }}
    fn constraint_id() -> &'static str {{
        "{}"
    }}
    fn schema_path() -> &'static str {{
        "{}"
    }}
}}
"##,
            &snake_name,
            &snake_name,
            &snake_name,
            &name,
            &name,
            self.title.as_ref().unwrap_or(&empty),
            self.description.as_ref().unwrap_or(&empty),
            self.id.as_ref().unwrap_or(&empty),
            &self.path
        )
    }
}
