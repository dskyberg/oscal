use serde_json::Value;

use error::{ParserError, Result};
use file::*;
use generate::*;
use namespace::NameSpace;
use parse::*;
use property::Property;
use property_type::*;
use referencable::Referencable;
use schema::Schema;
use schema_array::SchemaArray;
use schema_boolean_ref::SchemaBooleanRef;
use schema_enum::SchemaEnum;
use schema_id::SchemaId;
use schema_number_ref::SchemaNumberRef;
use schema_object::SchemaObject;
use schema_ref::SchemaReference;
use schema_string::SchemaString;
use schema_string_ref::SchemaStringRef;
use schema_type_ref::SchemaTypeRef;
use utils::*;

mod error;
mod file;
mod generate;
mod namespace;
mod parse;
mod property;
mod property_type;
mod referencable;
mod schema;
mod schema_array;
mod schema_boolean_ref;
mod schema_enum;
mod schema_id;
mod schema_number_ref;
mod schema_object;
mod schema_ref;
mod schema_string;
mod schema_string_ref;
mod schema_type_ref;
mod utils;

fn read_schema(file: &str) -> Result<Schema> {
    let json = read_file_to_string(file)?;
    let schema_val = serde_json::from_str::<Value>(&json)?;
    let schema = Schema::parse(&schema_val)?;
    Ok(schema)
}

fn help() {
    log::info!("id_parser <show | generate> <file name>");
}
fn main() -> Result<()> {
    std::env::set_var("RUST_LOG", "trace");
    env_logger::init();
    let args: Vec<String> = std::env::args().collect();
    if args.len() < 3 {
        help();
        return Ok(());
    }
    let command = args.get(1).unwrap();
    let file_name = args.get(2).unwrap();
    let schema = read_schema(file_name)?;

    match command.as_str() {
        "show" => schema.show(),
        "generate" => {
            let _ = schema.generate("../oscal_lib")?;
        }
        _ => {
            help();
        }
    };

    Ok(())
}
