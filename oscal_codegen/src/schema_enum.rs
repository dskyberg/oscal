use convert_case::{Case, Casing};
use serde_json::Value;

use crate::{
    //
    gen_txt_file,
    merge_ids,
    str_from_map,
    try_str_from_map,
    Generate,
    Parse,
    ParserError,
    Property,
    Referencable,
    SchemaId,
};

#[derive(Debug, Clone)]
pub struct SchemaEnum {
    pub id: SchemaId,
    pub title: String,
    pub description: Option<String>,
    pub _ref: Option<SchemaId>,
    pub enums: Vec<String>,
    pub enum_ref: Option<SchemaId>,
}

impl Referencable for SchemaEnum {
    fn id(&self) -> &SchemaId {
        &self.id
    }
}

impl Property for SchemaEnum {
    fn title(&self) -> Option<String> {
        Some(self.title.clone())
    }
    fn description(&self) -> Option<String> {
        self.description.clone()
    }
    fn reference(&self) -> Option<String> {
        self._ref.as_ref().map(|r| r.raw.clone())
    }
    fn name(&self) -> String {
        self.id.name.to_case(Case::Pascal)
    }
    fn ref_name(&self) -> Option<String> {
        Some(self.id.name.to_case(Case::Pascal))
    }
}

impl SchemaEnum {
    pub fn peek(
        value: &Value,
        parent_id: Option<&SchemaId>,
    ) -> crate::error::Result<Option<SchemaId>> {
        let obj = value.as_object().ok_or(ParserError::ObjectExpected)?;
        // Enums don't have an object type
        if obj.contains_key("type") {
            return Ok(None);
        }

        // Enums must have an allOf
        if !obj.contains_key("allOf") {
            return Ok(None);
        }

        let title = try_str_from_map("title", obj);
        if title.is_none() {
            return Ok(None);
        }
        let title = title.unwrap();

        let id_val = try_str_from_map("$id", obj);
        let id = merge_ids(parent_id, id_val, &title)?;
        Ok(Some(id))
    }
}

impl Parse for SchemaEnum {
    fn parse(
        value: &Value,
        _ns: &mut crate::namespace::NameSpace,
        parent_id: Option<&SchemaId>,
        name: Option<&str>,
    ) -> crate::error::Result<Self> {
        let obj = value.as_object().ok_or(ParserError::ObjectExpected)?;

        let title = str_from_map("title", obj).map_err(|e| {
            log::error!("Enum must have a title: {:#?}", name);
            e
        })?;

        let description = try_str_from_map("description", obj);
        let ref_str = try_str_from_map("$ref", obj);
        let _ref = match ref_str {
            Some(s) => SchemaId::try_from(s.as_str()).ok(),
            None => None,
        };
        let id = merge_ids(parent_id, None, &title)?;

        let allof = obj
            .get("allOf")
            .ok_or(ParserError::BadEnumeratedType)?
            .as_array()
            .ok_or(ParserError::BadEnumeratedType)?;

        if allof.len() < 2 {
            // Not sure what to do with this
            return Err(ParserError::BadEnumeratedType.into());
        }

        let mut enum_ref: Option<SchemaId> = None;
        let mut enums: Vec<String> = Vec::new();

        for entry in allof {
            let entry = entry.as_object().ok_or(ParserError::BadEnumeratedType)?;
            if entry.contains_key("$ref") {
                // This is the enum ref
                let enum_ref_val = entry
                    .get("$ref")
                    .ok_or(ParserError::BadEnumeratedType)?
                    .as_str()
                    .ok_or(ParserError::BadEnumeratedType)?;
                let id = SchemaId::try_from(enum_ref_val)?;
                enum_ref = Some(id);
            } else if entry.contains_key("enum") {
                let enum_array = entry
                    .get("enum")
                    .ok_or(ParserError::BadEnumeratedType)?
                    .as_array()
                    .ok_or(ParserError::BadEnumeratedType)?;

                for value in enum_array {
                    let e = value.as_str().ok_or(ParserError::BadEnumeratedType)?;
                    enums.push(e.to_string());
                }
            }
        }

        Ok(Self {
            id,
            title,
            description,
            _ref,
            enums,
            enum_ref,
        })
    }
}

impl Generate for SchemaEnum {
    fn generate(&self, path: &str) -> crate::error::Result<String> {
        let obj_name = &self.id.name;

        let file_path = format!("{}/{}.rs", path, obj_name);

        let mut contents = String::new();
        contents.push_str(&format!("/// {}\n", self.title));
        if let Some(desc) = &self.description {
            contents.push_str(&format!("/// {}\n", desc));
        }
        contents.push_str(&format!("/// $id: {}\n", &self.id.raw));
        contents.push_str("use serde::{Deserialize, Serialize};\n\n");

        contents.push_str("#[derive(Debug, Clone, Deserialize, Serialize)]\n");
        contents.push_str(r#"#[serde(rename_all ="kebab-case")]"#);
        contents.push('\n');
        contents.push_str(&format!("pub enum {} {{\n", self.id.to_pascal_case()));

        for val in &self.enums {
            contents.push_str(&format!("\t// orig: {}\n", &val));
            if val.starts_with("http://") || val.starts_with("https://") {
                let v = un_uri(val);
                contents.push_str(&format!(r##"#[serde(rename = "{}")]{}"##, val, "\n"));
                contents.push_str(&format!("\t{},\n", v.to_case(Case::Pascal)));
            } else {
                contents.push_str(&format!("\t{},\n", val.to_case(Case::Pascal)));
            }
        }
        contents.push_str("}\n");

        gen_txt_file(&file_path, &contents)?;

        Ok(obj_name.to_string())
    }
}

fn un_uri(val: &str) -> String {
    val.replace("://", "_").replace(['/', '.'], "_")
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_it() {
        let uri = "https://orcid.org/";

        let uri = uri.replace("://", "_").replace(['/', '.'], "_");
        let cased = uri.to_case(Case::Pascal);
        println!("{}", &cased);
    }
}
