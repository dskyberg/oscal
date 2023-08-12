use serde_json::Value;

use crate::{
    //
    try_str_from_map,
    Parse,
    ParserError,
    Property,
    Result,
    SchemaId,
};

#[derive(Debug, Clone)]
pub struct SchemaString {
    pub title: Option<String>,
    pub description: Option<String>,
    pub _ref: Option<String>,
    pub format: Option<String>,
    pub pattern: Option<String>,
}

impl Property for SchemaString {
    fn title(&self) -> Option<String> {
        self.title.clone()
    }
    fn description(&self) -> Option<String> {
        self.description.clone()
    }
    fn reference(&self) -> Option<String> {
        None
    }

    fn name(&self) -> String {
        "String".to_string()
    }
    fn ref_name(&self) -> Option<String> {
        None
    }
}

impl Parse for SchemaString {
    fn parse(
        value: &Value,
        _ns: &mut crate::namespace::NameSpace,
        _parent_id: Option<&SchemaId>,
        _name: Option<&str>,
    ) -> Result<SchemaString> {
        let obj = value.as_object().ok_or(ParserError::ObjectExpected)?;

        let title = try_str_from_map("title", obj)?.map(|s| s.to_string());
        let description = try_str_from_map("description", obj)?.map(|s| s.to_string());
        let format = try_str_from_map("format", obj)?.map(|s| s.to_string());
        let pattern = try_str_from_map("pattern", obj)?.map(|s| s.to_string());
        let _ref = try_str_from_map("$ref", obj)?.map(|s| s.to_string());

        Ok(SchemaString {
            title,
            description,
            _ref,
            format,
            pattern,
        })
    }
}
