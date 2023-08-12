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
pub struct SchemaBooleanRef {
    pub id: SchemaId,
    pub title: String,
    pub description: Option<String>,
    pub _ref: Option<String>,
}

impl Referencable for SchemaBooleanRef {
    fn id(&self) -> &SchemaId {
        &self.id
    }
}

impl Property for SchemaBooleanRef {
    fn title(&self) -> Option<String> {
        Some(self.title.clone())
    }
    fn description(&self) -> Option<String> {
        self.description.clone()
    }
    fn reference(&self) -> Option<String> {
        None
    }

    fn name(&self) -> String {
        self.id.to_pascal_case()
    }
    fn ref_name(&self) -> Option<String> {
        Some(self.id.to_pascal_case())
    }
}

impl Parse for SchemaBooleanRef {
    fn parse(
        value: &Value,
        _ns: &mut crate::namespace::NameSpace,
        parent_id: Option<&SchemaId>,
        name: Option<&str>,
    ) -> crate::error::Result<Self> {
        let obj = value.as_object().ok_or(ParserError::ObjectExpected)?;

        let local_id = str_from_map("$id", obj).map_err(|e| {
            log::error!("Referenceable  bool  must have an $id: {:#?}", name);
            e
        })?;
        let _id = SchemaId::try_from(local_id).map_err(|e| {
            log::error!("The $id is not valid: {}", local_id);
            e
        })?;

        let title = try_str_from_map("title", obj)?
            .map(|s| s.to_string())
            .unwrap_or(_id.to_pascal_case());

        let id = merge_ids(parent_id, Some(local_id), &title)?;

        let description = try_str_from_map("description", obj)?.map(|s| s.to_string());
        let _ref = try_str_from_map("$ref", obj)?.map(|s| s.to_string());

        Ok(SchemaBooleanRef {
            id,
            title,
            description,
            _ref,
        })
    }
}

impl Generate for SchemaBooleanRef {
    fn generate(&self, path: &str) -> crate::error::Result<String> {
        let obj_name = &self.id.name;
        let file_path = format!("{}/{}.rs", path, obj_name);
        let name = self.id.to_pascal_case();
        let mut contents = String::new();

        contents.push_str(&format!(
            r#"/// {title}
/// {description}
/// $id: {id}

"#,
            title = self.title,
            description = self.description.clone().unwrap_or("".to_string()),
            id = &self.id.raw
        ));
        // Standard 'use' statements
        contents.push_str("use serde::{de, Deserialize, Serialize, Serializer};\n\n");

        contents.push_str("#[derive(Debug, Clone, Deserialize, Serialize)]\n");
        contents.push_str(r#"#[serde(rename_all ="kebab-case")]"#);
        contents.push_str("\n#[serde(transparent)]\n");

        contents.push_str(&format!("pub struct {} {{\n", &name));
        contents.push_str(
            r##"#[serde(serialize_with = "serialize", deserialize_with = "deserialize")]"##,
        );
        contents.push_str("\n\tpub inner: bool\n");
        contents.push_str("}\n");
        contents.push_str(&format!(
            r#"
impl {} {{
    pub fn new(value: bool) -> Self {{
        Self{{inner: value}}
    }}

    /// String references are established so that the value can be
    /// validated against a pattern or other connstraint.
    pub fn validate(_value: bool) -> crate::error::Result<()> {{
        // todo!(); // Replace with appropriate validation method.
        Ok(())
    }}
}}

"#,
            &name
        ));

        contents.push_str(&format!(
            r#"
impl TryFrom<bool> for {} {{
    type Error = Box<dyn std::error::Error>;
    fn try_from(value: bool) -> Result<Self, Self::Error> {{
        match Self::validate(value) {{
            Ok(()) => Ok(Self::new(value)),
            Err(e) => Err(e),
        }}
    }}
}}
"#,
            &name
        ));

        contents.push_str(&format!(
            r#"
fn deserialize<'de, D>(deserializer: D) -> Result<bool, D::Error>
where
    D: de::Deserializer<'de>,
{{
    let s: bool = de::Deserialize::deserialize(deserializer)?;

    match {name}::validate(s) {{
        Ok(()) => Ok(s),
        _ => Err(de::Error::custom("not a boolean"))
    }}
}}

fn serialize<S>(value: &bool, serializer: S) -> Result<S::Ok, S::Error>
where
    S: Serializer,
{{
    serializer.serialize_bool(*value)
}}
"#,
            name = &name
        ));
        gen_txt_file(&file_path, &contents)?;
        Ok(self.id().name.clone())
    }
}
