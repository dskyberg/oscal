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
pub struct SchemaStringRef {
    pub id: SchemaId,
    pub title: String,
    pub description: Option<String>,
    pub _ref: Option<String>,
    pub format: Option<String>,
    pub pattern: Option<String>,
}

impl Referencable for SchemaStringRef {
    fn id(&self) -> &SchemaId {
        &self.id
    }
}

impl Property for SchemaStringRef {
    fn title(&self) -> Option<String> {
        Some(self.title.clone())
    }
    fn description(&self) -> Option<String> {
        self.description.clone()
    }
    fn reference(&self) -> Option<String> {
        self._ref.clone()
    }

    fn name(&self) -> String {
        self.id.to_pascal_case()
    }
    fn ref_name(&self) -> Option<String> {
        Some(self.id.to_pascal_case())
    }
}

impl Parse for SchemaStringRef {
    fn parse(
        value: &Value,
        _ns: &mut crate::namespace::NameSpace,
        parent_id: Option<&SchemaId>,
        name: Option<&str>,
    ) -> crate::error::Result<Self> {
        let obj = value.as_object().ok_or(ParserError::ObjectExpected)?;

        let local_id = str_from_map("$id", obj).map_err(|e| {
            log::error!("Referenceable String  must have an $id: {:#?}", name);
            e
        })?;
        let _id = SchemaId::try_from(local_id.as_str()).map_err(|e| {
            log::error!("The $id is not valid: {}", &local_id);
            e
        })?;

        let title = try_str_from_map("title", obj).unwrap_or(_id.to_pascal_case());

        let id = merge_ids(parent_id, Some(local_id), &title)?;

        let description = try_str_from_map("description", obj);
        let _ref = try_str_from_map("$ref", obj);
        let format = try_str_from_map("format", obj);
        let pattern = try_str_from_map("pattern", obj);

        Ok(SchemaStringRef {
            id,
            title,
            description,
            _ref,
            format,
            pattern,
        })
    }
}

impl Generate for SchemaStringRef {
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
        contents.push_str("\n\tpub inner: String\n");
        contents.push_str("}\n");
        contents.push_str(&format!(
            r#"
impl {} {{
    pub fn new(value: &str) -> Self {{
        Self{{inner: value.to_string()}}
    }}

    /// String references are established so that the value can be
    /// validated against a pattern or other connstraint.
    pub fn validate(_value: &str) -> crate::error::Result<()> {{
        // todo!(); // Replace with appropriate validation method.
        Ok(())
    }}
}}

"#,
            &name
        ));

        contents.push_str(&format!(
            r#"
impl TryFrom<&str> for {} {{
    type Error = Box<dyn std::error::Error>;
    fn try_from(value: &str) -> Result<Self, Self::Error> {{
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
fn deserialize<'de, D>(deserializer: D) -> Result<String, D::Error>
where
    D: de::Deserializer<'de>,
{{
    let s: String = de::Deserialize::deserialize(deserializer)?;

    match {name}::validate(&s) {{
        Ok(()) => Ok(s),
        _ => Err(de::Error::custom("invalid {name}")),
    }}
}}

fn serialize<S>(value: &str, serializer: S) -> Result<S::Ok, S::Error>
where
    S: Serializer,
{{
    serializer.serialize_str(value)
}}
"#,
            name = &name
        ));
        gen_txt_file(&file_path, &contents)?;
        Ok(self.id().name.clone())
    }
}
