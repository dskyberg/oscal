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
pub struct SchemaTypeRef {
    pub id: SchemaId,
    pub _ref: SchemaId,
    pub title: String,
    pub description: Option<String>,
}

impl Referencable for SchemaTypeRef {
    fn id(&self) -> &SchemaId {
        &self.id
    }
}

impl Property for SchemaTypeRef {
    fn title(&self) -> Option<String> {
        Some(self.title.clone())
    }
    fn description(&self) -> Option<String> {
        self.description.clone()
    }
    fn reference(&self) -> Option<String> {
        Some(self._ref.raw.clone())
    }
    fn name(&self) -> String {
        self.id.to_pascal_case()
    }
    fn ref_name(&self) -> Option<String> {
        Some(self._ref.to_pascal_case())
    }
}

impl Parse for SchemaTypeRef {
    fn parse(
        value: &Value,
        _ns: &mut crate::namespace::NameSpace,
        parent_id: Option<&SchemaId>,
        name: Option<&str>,
    ) -> crate::error::Result<Self> {
        let obj = value.as_object().ok_or(ParserError::ObjectExpected)?;

        let title = str_from_map("title", obj).map_err(|e| {
            log::error!("Type must have a title: {:#?}", name);
            e
        })?;

        let description = try_str_from_map("description", obj)?.map(|s| s.to_string());

        let local_id = str_from_map("$id", obj).map_err(|e| {
            log::error!("A TypeRef must have an $id: {:#?}", name);
            e
        })?;
        let id = merge_ids(parent_id, Some(local_id), title)?;

        let ref_val = str_from_map("$ref", obj).map_err(|e| {
            log::error!("A TypeRef must have a $ref: {:#?}", name);
            e
        })?;
        let _ref = SchemaId::try_from(ref_val).map_err(|e| {
            log::error!("TypeRef $ref failed to parse");
            e
        })?;

        Ok(SchemaTypeRef {
            id,
            _ref,
            title: title.to_string(),
            description,
        })
    }
}

impl Generate for SchemaTypeRef {
    fn generate(&self, path: &str) -> crate::error::Result<String> {
        let obj_name = &self.id.name;

        let file_path = format!("{}/{}.rs", path, obj_name);

        let mut contents = String::new();
        contents.push_str(&format!("/// {}\n", self.title));
        if let Some(desc) = &self.description {
            contents.push_str(&format!("/// {}\n", desc));
        }
        contents.push_str(&format!("/// $id: {}\n", &self.id.raw));
        if self.ref_name().is_none() {
            log::error!("The reference for type_ref cannot be none");
            return Err(ParserError::CannotGenerate.into());
        }
        let ref_name = self.ref_name().unwrap();

        contents.push_str(&format!("use crate::{};\n\n", &ref_name));
        contents.push_str(&format!(
            "pub type {} = {};\n",
            self.id.to_pascal_case(),
            &ref_name
        ));

        gen_txt_file(&file_path, &contents)?;
        Ok(obj_name.to_string())
    }
}
