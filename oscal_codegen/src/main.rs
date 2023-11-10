use clap::{Parser, Subcommand};
use serde_json::Value;

use error::{ParserError, Result};
use extensible::Extensible;
use file::*;
use generate::*;
use namespace::NameSpace;
use parse::*;
use property::Property;
use property_type::*;
use referencable::Referencable;
//use schema::Schema;
use schema2::Schema;
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
mod extensible;
mod file;
mod generate;
mod namespace;
mod parse;
mod property;
mod property_type;
mod referencable;
mod schema;
mod schema2;
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

#[derive(Parser, Debug)]
#[command(author, version, about, long_about = None)]
#[command(propagate_version = true)]
struct Cli {
    #[command(subcommand)]
    command: Commands,
}

#[derive(Debug, Subcommand)]
enum Commands {
    /// Check for errors in the schema and extensions.
    Check {
        /// Source schema file to generate from
        #[arg(short, long, value_name = "FILE")]
        file: String,
    },
    /// Shows the resulting schema and extensions, without generating code
    Show {
        /// Source schema file to generate from
        #[arg(short, long, value_name = "FILE")]
        file: String,
    },
    /// Generates Rust code from the schema and extensions
    Generate {
        /// Source schema file to generate from
        #[arg(short, long, value_name = "FILE")]
        file: String,
        /// Where to generate the src tree
        #[arg(short, long, value_name = "Directory", default_value = "../oscal_lib")]
        path: String,
    },
}

fn main() -> Result<()> {
    // If no env var is set, default to "info"
    // This is a convenience step for testing.  Just flip this to
    // "trace", so yu don't have to set the RUST_LOG env.
    if std::env::var("RUST_LOG").is_err() {
        std::env::set_var("RUST_LOG", "trace");
    }
    pretty_env_logger::init();

    let args = Cli::parse();

    match args.command {
        Commands::Check { file } => {
            let _schema = read_schema(&file)?;
            println!("Schema validates");
        }
        Commands::Show { file } => {
            let schema = read_schema(&file)?;
            schema.show();
        }
        Commands::Generate { file, path } => {
            let schema = read_schema(&file)?;
            schema.generate(&path)?;
        }
    }

    Ok(())
}

/// Read the schema file and parse it
fn read_schema(file: &str) -> Result<Schema> {
    let json = read_file_to_string(file)?;
    let schema_val = serde_json::from_str::<Value>(&json)?;
    let schema = Schema::parse(&schema_val)?;
    Ok(schema)
}
