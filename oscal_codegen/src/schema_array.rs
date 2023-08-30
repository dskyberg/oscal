/// Wraps a reference into a Vec.
/// If the schema defines a local object or enum, that
/// enum is added to the namespace and a reference is created for it.
use serde_json::Value;

use crate::{
    //
    try_str_from_map,
    Parse,
    ParserError,
    Property,
    PropertyType,
    SchemaEnum,
    SchemaId,
    SchemaObject,
    SchemaReference,
};

#[derive(Debug, Clone)]
pub struct SchemaArray {
    pub title: Option<String>,
    pub description: Option<String>,
    pub _ref: Option<SchemaId>,
    /// The type of value within the array.
    pub prop: Box<PropertyType>,
}

impl SchemaArray {
    pub fn prop_name(&self) -> String {
        self.prop.name()
    }
    pub fn id(&self) -> Option<&SchemaId> {
        self.prop.id()
    }

    pub fn minify_id(&mut self) {
        log::info!("Minifying array");
    }

    #[cfg(feature = "enums_as_enums")]
    fn handle_enum(
        value: &Value,
        ns: &mut crate::namespace::NameSpace,
        _parent_id: Option<&SchemaId>,
        _name: Option<&str>,
        child_id: &SchemaId,
    ) -> crate::error::Result<PropertyType> {
        // Add this object to the namespace
        let obj = ns.add_property(value, child_id, None).map_err(|e| {
            log::error!("Failed to add enum as a property");
            e
        })?;

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
        name: Option<&str>,
        _child_id: &SchemaId,
    ) -> crate::error::Result<PropertyType> {
        // Parse the Value as a SchhemaEnum, and then convert it to a SchemaReference
        let _enum = SchemaEnum::parse(value, ns, parent_id, name).map_err(|e| {
            log::error!("Failed to pase enum");
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

impl Property for SchemaArray {
    fn title(&self) -> Option<String> {
        self.title.clone()
    }
    fn description(&self) -> Option<String> {
        self.description.clone()
    }
    fn reference(&self) -> Option<String> {
        self._ref.as_ref().map(|r| r.raw.clone())
    }
    fn name(&self) -> String {
        format!("Vec<{}>", self.prop_name())
    }
    fn ref_name(&self) -> Option<String> {
        self.prop.ref_name()
    }
}

impl Parse for SchemaArray {
    fn parse(
        value: &Value,
        ns: &mut crate::namespace::NameSpace,
        parent_id: Option<&SchemaId>,
        name: Option<&str>,
    ) -> crate::error::Result<Self> {
        let obj = value.as_object().ok_or(ParserError::ObjectExpected)?;

        let title = try_str_from_map("title", obj)?.map(|s| s.to_string());
        let description = try_str_from_map("description", obj)?.map(|s| s.to_string());
        let ref_str = try_str_from_map("$ref", obj)?;
        let _ref = match ref_str {
            Some(s) => SchemaId::try_from(s).ok(),
            None => None,
        };

        if parent_id.is_none() {
            log::error!("Parent ID is required");
            return Err(ParserError::BadPropertyType.into());
        }

        if !obj.contains_key("items") {
            log::error!("Array does not contain 'items':{:?}", name);
            return Err(ParserError::BadPropertyType.into());
        }
        let items = obj.get("items").unwrap();

        // If the property is an object or an enum, it needs to be added to the
        // namespace, and converted into a reference
        if let Some(child_id) = SchemaObject::peek(items, parent_id)? {
            // Add this object to the namespace
            let obj = ns.add_property(items, &child_id, None)?;

            // Create a new Reference property, using the object's id
            // and return that, now that the object is loaded in the name space
            let prop_ref = SchemaReference::try_from(&obj).map_err(|e| {
                log::error!("Failed to convert SchemaObject to SchemaReference");
                e
            })?;
            let prop = PropertyType::Reference(prop_ref);
            return Ok(Self {
                title,
                description,
                _ref,
                prop: Box::new(prop),
            });
        }

        // For Enums' we can either treat it as a real enum, and generate a SchemaEnum,
        // or we can treat it as a reference - using SchemaEnum::enum_ref.  Let's add a
        // feature flag to toggle this.
        if let Some(child_id) = SchemaEnum::peek(items, parent_id)? {
            let prop = SchemaArray::handle_enum(items, ns, parent_id, name, &child_id)?;
            return Ok(Self {
                title,
                description,
                _ref,
                prop: Box::new(prop),
            });
        }

        // Otherwise, use the property directly
        let prop = PropertyType::parse(items, ns, parent_id, name)?;

        Ok(Self {
            title,
            description,
            _ref,
            prop: Box::new(prop),
        })
    }
}
