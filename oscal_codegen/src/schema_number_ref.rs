use serde_json::Value;

use crate::{
    //
    gen_txt_file,
    merge_ids,
    str_from_map,
    try_number_from_map,
    try_str_from_map,
    Generate,
    Parse,
    ParserError,
    Property,
    Referencable,
    SchemaId,
};

#[derive(Debug, Clone)]
pub struct SchemaNumberRef {
    pub id: SchemaId,
    pub title: String,
    pub description: Option<String>,
    pub _ref: Option<String>,
    pub min: Option<isize>,
    pub max: Option<isize>,
}

impl Referencable for SchemaNumberRef {
    fn id(&self) -> &SchemaId {
        &self.id
    }
}

impl Property for SchemaNumberRef {
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

impl Parse for SchemaNumberRef {
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
        let _id = SchemaId::try_from(local_id.as_str()).map_err(|e| {
            log::error!("The $id is not valid: {}", &local_id);
            e
        })?;

        let title = try_str_from_map("title", obj).unwrap_or(_id.to_pascal_case());

        let id = merge_ids(parent_id, Some(local_id), &title)?;

        let description = try_str_from_map("description", obj);
        let _ref = try_str_from_map("$ref", obj);
        let min = try_number_from_map("minimum", obj);
        let max = try_number_from_map("maximum", obj);

        Ok(SchemaNumberRef {
            id,
            title,
            description,
            _ref,
            min,
            max,
        })
    }
}

impl Generate for SchemaNumberRef {
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
        contents.push_str("\n\tpub inner: i64\n");
        contents.push_str("}\n");
        contents.push_str(&format!(
            r#"
impl {} {{
    pub fn new(value: i64) -> Self {{
        Self{{inner: value}}
    }}

    /// String references are established so that the value can be
    /// validated against a pattern or other connstraint.
    pub fn validate(_value: i64) -> crate::error::Result<()> {{
        //todo!(); // Replace with appropriate validation method.
        Ok(())
    }}
}}

"#,
            &name
        ));

        contents.push_str(&format!(
            r#"
impl TryFrom<i64> for {} {{
    type Error = Box<dyn std::error::Error>;
    fn try_from(value: i64) -> Result<Self, Self::Error> {{
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
fn deserialize<'de, D>(deserializer: D) -> Result<i64, D::Error>
where
    D: de::Deserializer<'de>,
{{
    let s: i64 = de::Deserialize::deserialize(deserializer)?;

    match {name}::validate(s) {{
        Ok(()) => Ok(s),
        _ => Err(de::Error::custom("not a number"))
    }}
}}

fn serialize<S>(value: &i64, serializer: S) -> Result<S::Ok, S::Error>
where
    S: Serializer,
{{
    serializer.serialize_i64(*value)
}}
"#,
            name = &name
        ));
        gen_txt_file(&file_path, &contents)?;
        Ok(self.id().name.clone())
    }
}
