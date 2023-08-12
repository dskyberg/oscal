use convert_case::{Case, Casing};
use serde_json::Value;

use crate::{
    str_from_map,
    //
    try_str_from_map,
    Parse,
    ParserError,
    Property,
    PropertyType,
    Referencable,
    Result,
    SchemaEnum,
    SchemaId,
    SchemaObject,
};

#[derive(Debug, Clone)]
pub struct SchemaReference {
    pub id: SchemaId,
    pub title: Option<String>,
    pub description: Option<String>,
}
impl Referencable for SchemaReference {
    fn id(&self) -> &SchemaId {
        &self.id
    }
}

impl Property for SchemaReference {
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
        self.id.name.to_case(Case::Pascal)
    }
    fn ref_name(&self) -> Option<String> {
        Some(self.id.name.to_case(Case::Pascal))
    }
}

impl Parse for SchemaReference {
    fn parse(
        value: &Value,
        _ns: &mut crate::namespace::NameSpace,
        _parent_id: Option<&SchemaId>,
        _name: Option<&str>,
    ) -> Result<SchemaReference> {
        let obj = value.as_object().ok_or(ParserError::ObjectExpected)?;

        let title = try_str_from_map("title", obj)?.map(|s| s.to_string());
        let description = try_str_from_map("description", obj)?.map(|s| s.to_string());
        let id_val = str_from_map("$ref", obj).map_err(|e| {
            log::error!("Reference has no $ref");
            e
        })?;
        let id = SchemaId::try_from(id_val)?;

        Ok(SchemaReference {
            id,
            title,
            description,
        })
    }
}

impl TryFrom<&PropertyType> for SchemaReference {
    type Error = ParserError;
    fn try_from(value: &PropertyType) -> std::result::Result<Self, Self::Error> {
        match value {
            PropertyType::Enum(e) => Self::try_from(e),
            PropertyType::Object(o) => Self::try_from(o),
            _ => {
                log::error!("Cannot convert type to SchemaReference");
                Err(ParserError::BadPropertyType)
            }
        }
    }
}

impl TryFrom<&SchemaObject> for SchemaReference {
    type Error = ParserError;
    fn try_from(value: &SchemaObject) -> std::result::Result<Self, Self::Error> {
        let title = value.title();
        let description = value.description();
        let id = value.id.clone();
        Ok(SchemaReference {
            title,
            description,
            id,
        })
    }
}

impl TryFrom<&SchemaEnum> for SchemaReference {
    type Error = ParserError;
    fn try_from(value: &SchemaEnum) -> std::result::Result<Self, Self::Error> {
        let title = value.title();
        let description = value.description();
        let id = value.id.clone();
        Ok(SchemaReference {
            title,
            description,
            id,
        })
    }
}
