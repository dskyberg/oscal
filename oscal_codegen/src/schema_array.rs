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

        let title = try_str_from_map("title", obj);
        let description = try_str_from_map("description", obj);
        let ref_str = try_str_from_map("$ref", obj);
        let _ref = match ref_str {
            Some(s) => SchemaId::try_from(s.as_str()).ok(),
            None => None,
        };

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

        if let Some(child_id) = SchemaEnum::peek(items, parent_id)? {
            // Add this object to the namespace
            let obj = ns.add_property(items, &child_id, None)?;

            // Create a new Reference property, using the object's id
            // and return that, now that the object is loaded in the name space
            let prop_ref = SchemaReference::try_from(&obj).map_err(|e| {
                log::error!("Failed to convert SchemaEnum to SchemaReference");
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
