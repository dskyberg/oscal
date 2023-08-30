use convert_case::{Case, Casing};
use std::collections::HashMap;

use serde_json::{Map, Value};

use crate::{
    //
    gen_txt_file,
    merge_ids,
    reserved_word,
    str_from_map,
    try_str_from_map,
    Generate,
    NameSpace,
    Parse,
    ParserError,
    Property,
    PropertyType,
    Referencable,
    Result,
    SchemaEnum,
    SchemaId,
    SchemaReference,
};

#[derive(Debug, Clone)]
pub struct SchemaObject {
    pub title: String,
    pub description: String,
    pub id: SchemaId,
    pub properties: Option<HashMap<String, PropertyType>>,
    pub required: Vec<String>,
}

impl Property for SchemaObject {
    fn title(&self) -> Option<String> {
        Some(self.title.clone())
    }
    fn description(&self) -> Option<String> {
        Some(self.description.clone())
    }
    fn reference(&self) -> Option<String> {
        Some(self.id.raw.clone())
    }
    fn name(&self) -> String {
        self.id.name.to_case(Case::Pascal)
    }
    fn ref_name(&self) -> Option<String> {
        Some(self.id.name.to_case(Case::Pascal))
    }
}

impl Referencable for SchemaObject {
    fn id(&self) -> &SchemaId {
        &self.id
    }
}

impl SchemaObject {
    pub fn peek(value: &Value, parent_id: Option<&SchemaId>) -> Result<Option<SchemaId>> {
        let obj = value.as_object().ok_or(ParserError::ObjectExpected)?;
        let obj_type = try_str_from_map("type", obj)?;
        if obj_type.is_none() {
            return Ok(None);
        }
        let obj_type = obj_type.unwrap();
        if obj_type != "object" {
            return Ok(None);
        }
        // This appears to be a typical object def that we can map to a struct.
        let title = str_from_map("title", obj)?;
        let id_val = try_str_from_map("$id", obj)?;
        let id = merge_ids(parent_id, id_val, title)?;
        Ok(Some(id))
    }

    pub fn minify_id(&mut self) {
        let last = self.id.path.last();
        if let Some(last_name) = last {
            if last_name.as_str() == self.id.name {
                // Pop the last.
                self.id.path.pop();
            }
        }
    }
}

impl Parse for SchemaObject {
    fn parse(
        value: &Value,
        ns: &mut NameSpace,
        parent_id: Option<&SchemaId>,
        name: Option<&str>,
    ) -> Result<Self> {
        let obj = value.as_object().ok_or(ParserError::ObjectExpected)?;

        let obj_type = str_from_map("type", obj).map_err(|e| {
            log::error!("Object has no type identifier");
            e
        })?;

        if obj_type != "object" {
            log::error!("Type should be object, but is {}", &obj_type);
            return Err(ParserError::WrongObjectType(obj_type.to_string()).into());
        }

        let title = str_from_map("title", obj)?;

        // This appears to be a typical object def that we can map to a struct.
        let id_val = try_str_from_map("$id", obj)?;
        let id = merge_ids(parent_id, id_val, title)?;

        let description = obj
            .get("description")
            .ok_or(ParserError::MissingField("Description".to_string()))?;
        let description = description.as_str().ok_or(ParserError::StringExpected)?;

        if !obj.contains_key("properties") {
            // Empty object.
            return Ok(Self {
                title: title.to_string(),
                description: description.to_string(),
                id,
                properties: None,
                required: Vec::new(),
            });
        }
        let properties_val = obj.get("properties").unwrap();
        let properties = parse_props(properties_val, ns, &id).map_err(|e| {
            log::error!("Failed parsing the set of properties for {:?}", name);
            e
        })?;
        if properties.is_none() {
            log::debug!("No properties for {:#?}", &id);
        }

        let required = parse_required(obj).map_err(ParserError::StackedError)?;

        Ok(Self {
            title: title.to_string(),
            description: description.to_string(),
            id,
            properties,
            required,
        })
    }
}

impl Generate for SchemaObject {
    fn generate(&self, path: &str) -> Result<String> {
        let mut crate_uses: Vec<String> = Vec::new();
        let mut prop_contents = String::from("");
        let obj_name = &self.id.name;
        let file_path = format!("{}/{}.rs", path, obj_name);

        // If this object has properties, dump them
        if let Some(props) = &self.properties {
            for (prop_name, prop) in props {
                let (name, is_reserved) = reserved_word(prop_name);
                // For recursive references, skip adding to crate_uses
                if let Some(ref_name) = prop.ref_name() {
                    // There are 2 cases where we don't want t add to the crate_uses
                    // - when the names are the same
                    // - when the namespace is a child of this namespace
                    let is_descendant = match prop.id() {
                        Some(id) => self.id().is_descendant(id),
                        None => false,
                    };

                    if ref_name != obj_name.to_case(Case::Pascal) && !is_descendant {
                        // crate_uses.push(ref_name);
                        if let Some(id) = prop.id() {
                            crate_uses.push(id.rustify());
                        }
                    }
                }

                if let Some(val) = prop.title() {
                    prop_contents.push_str(&format!("\t/// {}\n", val));
                }

                if let Some(val) = prop.description() {
                    prop_contents.push_str(&format!("\t/// {}\n", val));
                }
                if let Some(val) = prop.reference() {
                    prop_contents.push_str(&format!("\t/// $ref: {}\n", val));
                }
                if is_reserved {
                    prop_contents.push('\t');
                    prop_contents.push_str(&format!(r##"#[serde(rename = "{}")]"##, prop_name));
                    prop_contents.push_str(&format!("\n\tpub {}: ", &name));
                } else {
                    prop_contents.push_str(&format!("\tpub {}: ", &name.to_case(Case::Snake)));
                }

                match self.required.contains(prop_name) {
                    true => prop_contents.push_str(&format!("{},", prop.name())),
                    false => prop_contents.push_str(&format!("Option<{}>,", prop.name())),
                }
                prop_contents.push('\n');
            }
        }

        let mut contents = String::new();
        contents.push_str(&format!("/// {}\n", self.title));
        contents.push_str(&format!("/// {}\n", self.description));
        contents.push_str(&format!("/// $id: {}\n", &self.id.raw));
        contents.push_str("use serde::{Deserialize, Serialize};\n");
        contents.push_str("use serde_with::skip_serializing_none;\n\n");

        // add all the 'use crate::...' statements
        crate_uses.sort();
        crate_uses.dedup();
        for uses in crate_uses {
            contents.push_str(&format!("use {};\n", uses));
        }
        contents.push('\n');

        contents.push_str("#[skip_serializing_none]\n");
        contents.push_str("#[derive(Debug, Clone, Deserialize, Serialize)]\n");
        contents.push_str(r#"#[serde(rename_all ="kebab-case")]"#);
        contents.push('\n');
        contents.push_str(&format!("pub struct {} {{\n", self.id.to_pascal_case()));

        contents.push_str(&prop_contents);
        contents.push_str("}\n");
        gen_txt_file(&file_path, &contents)?;

        Ok(self.id.name.clone())
    }
}

fn parse_required(obj: &Map<String, Value>) -> Result<Vec<String>> {
    if !obj.contains_key("required") {
        return Ok(Vec::new());
    }
    let req_array = obj.get("required").unwrap().as_array().unwrap();
    let required = req_array
        .iter()
        .map(|v| v.as_str().unwrap().to_string())
        .collect();
    Ok(required)
}

fn parse_props(
    value: &Value,
    ns: &mut NameSpace,
    obj_id: &SchemaId,
) -> Result<Option<HashMap<String, PropertyType>>> {
    let properties = value.as_object().ok_or(ParserError::ObjectExpected)?;

    let mut result = HashMap::<String, PropertyType>::new();
    for (prop_name, prop_val) in properties {
        // If the property is an object or an enum, it needs to be added to the
        // namespace, and converted into a reference
        if let Some(child_id) = SchemaObject::peek(prop_val, Some(obj_id))? {
            // Add this object to the namespace
            let obj = ns.add_property(prop_val, &child_id, Some(prop_name))?;

            // Create a new Reference property, using the object's id
            // and return that, now that the object is loaded in the name space
            let prop_ref = SchemaReference::try_from(&obj).map_err(|e| {
                log::error!("Failed to convert SchemaObject to SchemaReference");
                e
            })?;
            let prop = PropertyType::Reference(prop_ref);
            result.insert(prop_name.to_owned(), prop);
        } else if let Some(child_id) = SchemaEnum::peek(prop_val, Some(obj_id))? {
            let prop = SchemaObject::handle_enum(prop_val, ns, Some(obj_id), prop_name, child_id)?;
            result.insert(prop_name.to_owned(), prop);
        } else {
            let prop =
                PropertyType::parse(prop_val, ns, Some(obj_id), Some(prop_name)).map_err(|e| {
                    log::error!("Failed parsing property: {}", prop_name);
                    e
                })?;

            result.insert(prop_name.to_owned(), prop);
        }
    }

    Ok(Some(result))
}

impl SchemaObject {
    #[cfg(feature = "enums_as_enums")]
    fn handle_enum(
        value: &Value,
        ns: &mut crate::namespace::NameSpace,
        _parent_id: Option<&SchemaId>,
        name: &str,
        id: SchemaId,
    ) -> crate::error::Result<PropertyType> {
        // Add this object to the namespace
        let obj = ns.add_property(value, &id, Some(name))?;

        // Create a new Reference property, using the object's id
        // and return that, now that the object is loaded in the name space
        let prop_ref = SchemaReference::try_from(&obj).map_err(|e| {
            log::error!("Failed to convert SchemaEnum to SchemaReference");
            e
        })?;

        Ok(PropertyType::Reference(prop_ref))
    }

    #[cfg(feature = "enums_as_refs")]
    fn handle_enum(
        value: &Value,
        ns: &mut crate::namespace::NameSpace,
        parent_id: Option<&SchemaId>,
        name: &str,
        _id: SchemaId,
    ) -> crate::error::Result<PropertyType> {
        // Parse the Value as a SchhemaEnum, and then convert it to a SchemaReference
        let _enum = SchemaEnum::parse(value, ns, parent_id, Some(name)).map_err(|e| {
            log::error!("Attempt to parse enum failed");
            e
        })?;
        let prop_ref = SchemaReference {
            title: Some(_enum.title),
            description: _enum.description,
            id: _enum.enum_ref,
        };
        Ok(PropertyType::Reference(prop_ref))
    }
}
