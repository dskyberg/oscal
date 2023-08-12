# Generate Rust code from the OSCAL schema

## Running

For help with the cli, run

```bash, ignore
> codegen -h
```

To generate code from schema, and extend, use:

```bash, ignore
> codegen generate -f  -f ../schema/oscal_complete_schema.json -e ../schema/okta_extensions.json
```