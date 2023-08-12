use serde_json::Value;

use crate::{
    //
    try_str_from_map,
    Extensible,
    Generate,
    NameSpace,
    Parse,
    ParserError,
    Property,
    Referencable,
    Result,
    SchemaArray,
    SchemaBooleanRef,
    SchemaEnum,
    SchemaId,
    SchemaNumberRef,
    SchemaObject,
    SchemaReference,
    SchemaString,
    SchemaStringRef,
    SchemaTypeRef,
};

#[derive(Debug, Clone)]
pub enum PropertyType {
    ReferencableInteger(SchemaNumberRef),
    ReferencableBoolean(SchemaBooleanRef),
    ReferencableString(SchemaStringRef),
    TypeRef(SchemaTypeRef),
    Reference(SchemaReference),
    Object(SchemaObject),
    Enum(SchemaEnum),
    Array(SchemaArray),
    String(SchemaString),
}

impl PropertyType {
    pub fn id(&self) -> Option<&SchemaId> {
        match self {
            PropertyType::ReferencableInteger(val) => Some(val.id()),
            PropertyType::ReferencableBoolean(val) => Some(val.id()),
            PropertyType::ReferencableString(val) => Some(val.id()),
            PropertyType::TypeRef(val) => Some(val.id()),
            PropertyType::Reference(val) => Some(val.id()),
            PropertyType::Object(val) => Some(val.id()),
            PropertyType::Enum(val) => Some(val.id()),
            PropertyType::Array(array) => array.id(),
            PropertyType::String(_) => None,
        }
    }
}

impl Property for PropertyType {
    fn description(&self) -> Option<String> {
        match self {
            PropertyType::ReferencableInteger(val) => val.description(),
            PropertyType::ReferencableBoolean(val) => val.description(),
            PropertyType::ReferencableString(val) => val.description(),
            PropertyType::TypeRef(val) => val.description(),
            PropertyType::Reference(val) => val.description(),
            PropertyType::Object(val) => val.description(),
            PropertyType::Enum(val) => val.description(),
            PropertyType::Array(val) => val.description(),
            PropertyType::String(val) => val.description(),
        }
    }

    fn title(&self) -> Option<String> {
        match self {
            PropertyType::ReferencableInteger(val) => val.title(),
            PropertyType::ReferencableBoolean(val) => val.title(),
            PropertyType::ReferencableString(val) => val.title(),
            PropertyType::TypeRef(val) => val.title(),
            PropertyType::Reference(val) => val.title(),
            PropertyType::Object(val) => val.title(),
            PropertyType::Enum(val) => val.title(),
            PropertyType::Array(val) => val.title(),
            PropertyType::String(val) => val.title(),
        }
    }

    fn name(&self) -> String {
        match self {
            PropertyType::ReferencableInteger(val) => val.name(),
            PropertyType::ReferencableBoolean(val) => val.name(),
            PropertyType::ReferencableString(val) => val.name(),
            PropertyType::TypeRef(val) => val.name(),
            PropertyType::Reference(val) => val.name(),
            PropertyType::Object(val) => val.name(),
            PropertyType::Enum(val) => val.name(),
            PropertyType::Array(val) => val.name(),
            PropertyType::String(val) => val.name(),
        }
    }
    fn reference(&self) -> Option<String> {
        match self {
            PropertyType::ReferencableInteger(val) => val.reference(),
            PropertyType::ReferencableBoolean(val) => val.reference(),
            PropertyType::ReferencableString(val) => val.reference(),
            PropertyType::TypeRef(val) => val.reference(),
            PropertyType::Reference(val) => val.reference(),
            PropertyType::Object(val) => val.reference(),
            PropertyType::Enum(val) => val.reference(),
            PropertyType::Array(val) => val.reference(),
            PropertyType::String(val) => val.reference(),
        }
    }
    fn ref_name(&self) -> Option<String> {
        match self {
            PropertyType::ReferencableInteger(val) => val.ref_name(),
            PropertyType::ReferencableBoolean(val) => val.ref_name(),
            PropertyType::ReferencableString(val) => val.ref_name(),
            PropertyType::TypeRef(val) => val.ref_name(),
            PropertyType::Reference(val) => val.ref_name(),
            PropertyType::Object(val) => val.ref_name(),
            PropertyType::Enum(val) => val.ref_name(),
            PropertyType::Array(val) => val.ref_name(),
            PropertyType::String(val) => val.ref_name(),
        }
    }
}

impl Extensible for PropertyType {
    fn extend_schema(&mut self, value: &Value) -> Result<()> {
        match self {
            PropertyType::Enum(val) => val.extend_schema(value),
            _ => Err(ParserError::NotExstensible.into()),
        }
    }
}

impl Parse for PropertyType {
    fn parse(
        value: &Value,
        ns: &mut NameSpace,
        parent_id: Option<&SchemaId>,
        name: Option<&str>,
    ) -> Result<Self> {
        let obj = value.as_object().ok_or(ParserError::ObjectExpected)?;

        if obj.contains_key("$id") && obj.contains_key("$ref") && !obj.contains_key("type") {
            // This is a type reference
            let type_ref = SchemaTypeRef::parse(value, ns, parent_id, name).map_err(|e| {
                log::error!("Failed to parse ProppertyType::TypeRef");
                e
            })?;
            return Ok(PropertyType::TypeRef(type_ref));
        }

        // Is this Just a reference?
        if obj.contains_key("$ref") && !obj.contains_key("$id") {
            let _ref = SchemaReference::parse(value, ns, parent_id, name).map_err(|e| {
                log::error!("Failed to parse ProppertyType::Reference");
                e
            })?;
            return Ok(PropertyType::Reference(_ref));
        }

        let _type = try_str_from_map("type", obj)?;
        if _type.is_none() {
            // This can happen for enum types.
            if let Ok(enum_type) = SchemaEnum::parse(value, ns, parent_id, name) {
                return Ok(PropertyType::Enum(enum_type));
            }
            log::error!("No type, and not an enum");
            return Err(ParserError::BadPropertyType.into());
        }
        let _type = _type.unwrap();

        // If it's not a $ref or a string, then it's a type with items
        match _type {
            "integer" | "number" => {
                if obj.contains_key("$id") {
                    // This is a referencable string
                    let ref_str =
                        SchemaNumberRef::parse(value, ns, parent_id, name).map_err(|_| {
                            log::error!("Ref integer property failed to parse");
                            ParserError::BadPropertyType
                        })?;
                    return Ok(PropertyType::ReferencableInteger(ref_str));
                }
                log::error!("integer property is not supprted");
                Err(ParserError::BadPropertyType.into())
            }
            "boolean" => {
                if obj.contains_key("$id") {
                    // This is a referencable string
                    let ref_str =
                        SchemaBooleanRef::parse(value, ns, parent_id, name).map_err(|_| {
                            log::error!("Ref Boolean property failed to parse");
                            ParserError::BadPropertyType
                        })?;
                    return Ok(PropertyType::ReferencableBoolean(ref_str));
                }
                log::error!("boolean property is not supprted");
                Err(ParserError::BadPropertyType.into())
            }
            "string" => {
                if obj.contains_key("$id") {
                    // This is a referencable string
                    let ref_str =
                        SchemaStringRef::parse(value, ns, parent_id, name).map_err(|_| {
                            log::error!("Ref Str property failed to parse");
                            ParserError::BadPropertyType
                        })?;
                    return Ok(PropertyType::ReferencableString(ref_str));
                }
                let _str = SchemaString::parse(value, ns, parent_id, name).map_err(|_| {
                    log::error!("String property failed to parse");
                    ParserError::BadPropertyType
                })?;
                Ok(PropertyType::String(_str))
            }
            "object" => {
                let obj = SchemaObject::parse(value, ns, parent_id, name).map_err(|_| {
                    log::error!("Object property failed to parse");
                    ParserError::BadPropertyType
                })?;
                Ok(PropertyType::Object(obj))
            }
            "array" => {
                let array = SchemaArray::parse(value, ns, parent_id, name).map_err(|_| {
                    log::error!("Array property failed to parse");
                    ParserError::BadPropertyType
                })?;
                Ok(PropertyType::Array(array))
            }
            _ => {
                // Oops!  Unrecognized type
                log::error!("Unrecognize type: {}", _type);
                Err(ParserError::UnrecognizedType(_type.to_string()).into())
            }
        }
    }
}

impl Generate for PropertyType {
    fn generate(&self, path: &str) -> Result<String> {
        match self {
            PropertyType::Object(val) => val.generate(path),
            PropertyType::Enum(val) => val.generate(path),
            PropertyType::ReferencableString(val) => val.generate(path),
            _ => {
                // This should never get called
                Err(ParserError::CannotGenerate.into())
            }
        }
    }
}
