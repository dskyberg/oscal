use serde_json::Value;

use crate::{
    //
    file::{create_folder, gen_txt_file, read_file_to_string, remove_file},
    parse::Parse,
    Generate,
    ParserError,
    PropertyType,
    Referencable,
    Result,
    SchemaId,
};

/// NameSpace provides a hierarchical construct for mapping $id and $ref values
/// in objects, arrays, and the top level 'definitions' collection of the OSCAL schema.
/// Logically, only SchemaObject and SchemaEnum type can be held in the namespace.
/// Any other property type is ignored.
#[derive(Debug, Clone)]
pub enum NameSpace {
    Node {
        name: String,
        children: Vec<NameSpace>,
    },
    Leaf {
        name: String,
        child: PropertyType,
    },
}

impl NameSpace {
    pub fn new(name: &str) -> Self {
        // If a name was provided, use that as the root of the NameSpace.  Else use root
        NameSpace::Node {
            name: name.to_string(),
            children: Vec::new(),
        }
    }

    pub fn name(&self) -> &str {
        match self {
            NameSpace::Node { name, children: _ } => name.as_ref(),
            NameSpace::Leaf { name, child: _ } => name.as_ref(),
        }
    }

    pub fn new_leaf(name: &str, child: PropertyType) -> Result<NameSpace> {
        match child {
            PropertyType::Object(_)
            | PropertyType::Enum(_)
            | PropertyType::TypeRef(_)
            | PropertyType::ReferencableBoolean(_)
            | PropertyType::ReferencableInteger(_)
            | PropertyType::ReferencableString(_) => Ok(NameSpace::Leaf {
                name: name.to_string(),
                child,
            }),
            _ => {
                log::error!("Property {} cannot be a leaf: {:#?}", name, &child);
                Err(ParserError::WrongType.into())
            }
        }
    }

    pub fn is_leaf(&self) -> bool {
        matches!(self, NameSpace::Leaf { name: _, child: _ })
    }

    /// A terminal node is one that has only one leaf child, with identical name as
    /// this node
    pub fn is_terminal_node(&self) -> bool {
        match self {
            NameSpace::Leaf { name: _, child: _ } => false,
            NameSpace::Node { name, children } => {
                if children.len() != 1 {
                    return false;
                }
                if let Some(child) = children.get(0) {
                    if !child.is_leaf() {
                        return false; // this is weird
                    }
                    return name == child.name();
                }
                false
            }
        }
    }

    pub fn get_terminal_child(&self) -> Result<&NameSpace> {
        if !self.is_terminal_node() {
            return Err(ParserError::ExpectedTerminalNode.into());
        }
        // Since this is a terminal node, we know it has only 1 child, and the
        // child is a leaf.
        match self {
            NameSpace::Node { name: _, children } => {
                let child = children.get(0).unwrap();
                Ok(child)
            }
            _ => Err(ParserError::ExpectedTerminalNode.into()),
        }
    }

    pub fn contains(&self, key: &str) -> bool {
        match self {
            NameSpace::Leaf { name, child: _ } => name == key,
            NameSpace::Node { name: _, children } => {
                for child in children {
                    if child.is(key) {
                        return true;
                    }
                }
                false
            }
        }
    }

    pub fn get_mut(&mut self, key: &str) -> Option<&mut NameSpace> {
        match self {
            NameSpace::Leaf { name: _, child: _ } => None,
            NameSpace::Node { name: _, children } => {
                for child in children {
                    if child.is(key) {
                        return Some(child);
                    }
                }
                None
            }
        }
    }

    pub fn is(&self, target: &str) -> bool {
        match self {
            NameSpace::Leaf { name, child: _ } => name == target,
            NameSpace::Node { name, children: _ } => name == target,
        }
    }

    pub fn push(&mut self, target: NameSpace) {
        match self {
            NameSpace::Leaf { name: _, child: _ } => {}
            NameSpace::Node { name: _, children } => {
                children.push(target);
            }
        }
    }

    /// Walk down the path of SchemaId.  Create the namespace
    /// if it doesn't exist.  Return the namespace for the last
    /// part of the path
    pub fn upsert(&mut self, id: &SchemaId) -> &mut NameSpace {
        let mut ns: &mut NameSpace = self;

        // Walk the id path and ensure there is a namespace
        // for each part.
        for name in &id.path {
            if ns.contains(name.as_str()) {
                ns = ns.get_mut(name.as_str()).unwrap();
            } else {
                // New path
                let node = NameSpace::Node {
                    name: name.to_string(),
                    children: Vec::new(),
                };
                ns.push(node);
                ns = ns.get_mut(name).unwrap();
            }
        }
        ns
    }

    /// Called by an object that has properties.
    /// Called by [Schema::parse] to add the properties under "definitions".
    pub fn add_property(
        &mut self,
        value: &Value,
        id: &SchemaId,
        name: Option<&str>,
    ) -> Result<PropertyType> {
        //let ns = self.upsert(id);
        let ns = self;
        // Create the namespace for the value. and add it.
        let mut node = NameSpace::Node {
            name: id.name.to_string(),
            children: Vec::new(),
        };

        let prop = PropertyType::parse(value, &mut node, Some(id), name).map_err(|e| {
            log::error!("Property failed to parse");
            e
        })?;

        // Add the object to the namespace
        let leaf = NameSpace::new_leaf(id.name.as_str(), prop.clone()).map_err(|e| {
            log::error!("Failed to create a new leaf");
            e
        })?;
        node.push(leaf);

        // And add the new namespace to the parent
        ns.push(node);

        Ok(prop)
    }

    pub fn show(&self, depth: usize) {
        let pad = vec!["\t"; depth].join("");
        match self {
            NameSpace::Leaf { name: _, child } => match child {
                PropertyType::Object(obj) => println!("{}{}: Object", pad, obj.id().name),
                PropertyType::Enum(obj) => println!("{}{}: Enum", pad, obj.id().name),
                _ => {}
            },
            NameSpace::Node { name, children } => {
                if !self.is_terminal_node() {
                    println!("{}{}: Node", pad, name);
                }
                for child in children {
                    child.show(depth + 1);
                }
            }
        }
    }
}

trait Namable {
    fn name(&self) -> &str;
}

impl Namable for NameSpace {
    fn name(&self) -> &str {
        match self {
            NameSpace::Node { name, children: _ } => name,
            NameSpace::Leaf { name, child: _ } => name,
        }
    }
}

impl Generate for NameSpace {
    /// A node with no leaf children just needs a path,
    /// and then hand off to each child
    fn generate(&self, path: &str) -> Result<String> {
        match self {
            NameSpace::Leaf { name, child } => {
                // if this child isn't an object or enum, how did it get here?
                match child {
                    PropertyType::Object(obj) => obj.generate(path),
                    PropertyType::Enum(e) => e.generate(path),
                    PropertyType::TypeRef(type_ref) => type_ref.generate(path),
                    PropertyType::ReferencableString(val) => val.generate(path),
                    PropertyType::ReferencableBoolean(val) => val.generate(path),
                    PropertyType::ReferencableInteger(val) => val.generate(path),
                    _ => {
                        log::error!(
                            "Cannot generate this type of proprty: {} - {:#?}",
                            name,
                            child
                        );
                        Err(ParserError::CannotGenerate.into())
                    }
                }
            }
            NameSpace::Node { name, children } => {
                // If this is a terminal node, just act like a leaf
                if self.is_terminal_node() {
                    let child = self.get_terminal_child()?;
                    let name = child.generate(path)?;
                    return Ok(name);
                }

                let mut uses = String::new();
                let mut mods: String = String::new();
                let mut child_contents = String::new();

                let path = format!("{}/{}", path, name);
                create_folder(&path)?;

                for child in children {
                    let cname = child.generate(&path)?;
                    if &cname == "oscal" && name == "src" {
                        // Don't add `pub use oscal`
                        continue;
                    }

                    if child.name() == name {
                        let path = format!("{}/{}.rs", path, name);
                        child_contents = read_file_to_string(&path)?;
                        remove_file(&path)?;
                    } else {
                        uses.push_str(&format!("pub use {}::*;\n", &cname));
                        mods.push_str(&format!("pub mod {};\n", &cname));
                    }
                }

                let file_name = match name.as_ref() {
                    "src" => {
                        // Stuff that only pertains to the top level lib
                        mods.push_str("pub mod error;\n");
                        "lib.rs"
                    }
                    _ => "mod.rs",
                };

                let mut mod_file_inner = String::new();
                if file_name == "lib.rs" {
                    mod_file_inner.push_str(r##"#![doc = include_str!("../README.md")]"##);
                    mod_file_inner.push('\n');
                    mod_file_inner.push_str(r##"#![allow(ambiguous_glob_reexports)]"##);
                    mod_file_inner.push('\n');
                }
                mod_file_inner.push_str(&format!("{}\n\n{}\n{}", uses, mods, child_contents));

                // If there's a leaf with the same name as this, then generate it
                // right here in the mod file.

                gen_txt_file(&format!("{}/{}", &path, file_name), &mod_file_inner)?;
                Ok(name.to_owned())
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_it() {
        let pc = NameSpace::Leaf {
            name: "parameter_constraint".to_string(),
            child: PropertyType::String(crate::SchemaString {
                title: None,
                description: None,
                _ref: None,
                format: None,
                pattern: None,
            }),
        };
        assert!(pc.contains("parameter_constraint"));
        assert!(pc.is("parameter_constraint"));

        let occ = NameSpace::Node {
            name: "oscal_catalog_common".to_string(),
            children: vec![pc],
        };
        assert!(occ.contains("parameter_constraint"));
        assert!(occ.is("oscal_catalog_common"));

        let assembly = NameSpace::Node {
            name: "assembly".to_string(),
            children: vec![occ],
        };
        assert!(assembly.contains("oscal_catalog_common"));
        assert!(assembly.is("assembly"));

        let ns = NameSpace::Node {
            name: "root".to_string(),
            children: vec![assembly],
        };
        assert!(ns.contains("assembly"));
        assert!(ns.is("root"));
    }
}
