/// SchemaEnum creates verifiable data
/// There are two parts to a SchemaEnum
///
/// - A reference to an OSCAL data type, such as `TokenDatatype`
/// - An enum with a core set of valid values
///
/// Objects that include the enum, should allow any value of the referenced data type.
///
/// A separate extensible collection will be created that can be used to validate the data
/// programmatically.  This can't be done via the serde crate because there is no way to provide a serialize/deserialize
/// context to Serde.
///
use convert_case::{Case, Casing};
use serde::Serialize;
use serde_json::Value;

use crate::{
    //
    array_from_map,
    gen_txt_file,
    merge_ids,
    str_from_map,
    str_from_value,
    try_str_from_map,
    Extensible,
    Generate,
    Parse,
    ParserError,
    Property,
    Referencable,
    SchemaId,
};

#[derive(Debug, Clone, Serialize)]
pub struct SchemaEnum {
    pub id: SchemaId,
    pub title: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub _ref: Option<SchemaId>,
    pub enums: Vec<String>,
    pub enum_ref: SchemaId,
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

        let title = try_str_from_map("title", obj)?;
        if title.is_none() {
            return Ok(None);
        }
        let title = title.unwrap();

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

impl Extensible for SchemaEnum {
    fn extend_schema(&mut self, value: &Value) -> crate::error::Result<()> {
        let obj = value.as_object().ok_or(ParserError::ObjectExpected)?;

        let allof = array_from_map("allOf", obj)?;

        for entry in allof {
            let obj = entry.as_object().ok_or(ParserError::ObjectExpected)?;

            if !obj.contains_key("enum") {
                continue;
            }
            let enum_array = array_from_map("enum", obj)?;

            for value in enum_array {
                let e = value.as_str().ok_or(ParserError::BadEnumeratedType)?;
                log::trace!("Adding {} to {}", e, self.name());
                self.enums.push(e.to_string());
            }
            return Ok(());
        }
        log::error!("Enum extension with no 'enum' element");
        Err(ParserError::BadExtension("Enum extension with no 'enum' element".to_string()).into())
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

        let title = str_from_map("title", obj)
            .map_err(|e| {
                log::error!("Enum must have a title: {:#?}", name);
                e
            })?
            .to_string();

        let description = try_str_from_map("description", obj)?.map(|s| s.to_string());
        let ref_str = try_str_from_map("$ref", obj)?;
        let _ref = match ref_str {
            Some(s) => SchemaId::try_from(s).ok(),
            None => None,
        };
        let id = merge_ids(parent_id, None, &title)?;

        let allof = array_from_map("allOf", obj)?;
        if allof.len() != 2 {
            // Not sure what to do with this
            return Err(ParserError::BadEnumeratedType.into());
        }

        // Safe unwrap, since we know the len is 2
        let entry = allof
            .get(0)
            .unwrap()
            .as_object()
            .ok_or(ParserError::BadEnumeratedType)?;
        let enum_ref_val = str_from_map("$ref", entry).map_err(|_| {
            log::error!("Enum does not contain a $ref");
            ParserError::BadEnumeratedType
        })?;
        // This is the enum ref
        let enum_ref = SchemaId::try_from(enum_ref_val).map_err(|e| {
            log::error!("$ref is not a valid id: {}", enum_ref_val);
            e
        })?;

        let mut enums: Vec<String> = Vec::new();
        let entry = allof
            .get(1)
            .unwrap()
            .as_object()
            .ok_or(ParserError::BadEnumeratedType)?;

        let enum_array = array_from_map("enum", entry).map_err(|_| {
            log::error!("Enum does not contain enum");
            ParserError::BadEnumeratedType
        })?;
        for value in enum_array {
            let e = str_from_value(value)?;
            enums.push(e.to_string());
        }

        /*
               for entry in allof {
                   let entry = entry.as_object().ok_or(ParserError::BadEnumeratedType)?;
                   if let Some(enum_ref_val) = try_str_from_map("$ref", obj)? {
                       // This is the enum ref
                       let id = SchemaId::try_from(enum_ref_val)?;
                       enum_ref = Some(id);
                   } else if let Some(enum_array) = try_array_from_map("enum", entry)? {
                       for value in enum_array {
                           let e = str_from_value(value)?;
                           enums.push(e.to_string());
                       }
                   }
               }

        */
        let result = Self {
            id,
            title,
            description,
            _ref,
            enums,
            enum_ref,
        };
        let json = serde_json::to_string_pretty(&result).unwrap();
        println!("{},", json);
        Ok(result)
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
            // If the value is a url, , remove the ugly parts
            if val.starts_with("http://") || val.starts_with("https://") {
                let v = un_uri(val);
                contents.push_str(&format!(r##"#[serde(rename = "{}")]{}"##, val, "\n"));
                contents.push_str(&format!("\t{},\n", v.to_case(Case::Pascal)));
            } else if val == &val.to_case(Case::Upper) {
                // If this is one of those weird all upper case words...
                contents.push('\t');
                contents.push_str(&format!(r##"#[serde(rename = "{}")]{}"##, val, "\n"));
                contents.push_str(&format!("\t{},\n", val.to_case(Case::Pascal)));
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
