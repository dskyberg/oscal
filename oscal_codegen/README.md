# Generate Rust code from the OSCAL schema

## Running

For help with the cli, run

```bash, ignore
> codegen -h
```

To generate code from schema, and extend, use:

```bash, ignore
> codegen generate -f ../schema/oscal_complete_schema.json -e ../schema/okta_extensions.json
```

## Handling Enums

The OSCAL schema defines extensible enums, that are defined with a base type (as a `$ref`) and 1 or more default variants.
The FedRAMP extensions define context specific variants for these enums.  This is just too brittle to manage as Rust enums.
So, you have the option to abandon enums, and treat them as the the base `$ref`.  Or you can continually update the code
to include additional variants, as they arise.

The default is to use refs.  To use enums,  turn off default features, and use the `enums_as_enums` feature.

```bash
cargo build --no-default-features --features "enums_as_enums"
```

# V2
Restructure the lib to align better with extensibility, constraints, and validation.
Top level `definitions` entries with `#id` values are what should be exposed.  These are the only things that legit should fulfill a '#ref'

This means that everything underneath the above structs are locally defined.

There are 127 total entries.