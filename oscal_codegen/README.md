# Generate Rust code from the OSCAL schema

## Issues
The OSCAL schema has a loose interpretation of `enum`. In some instances, it is
better to ignore the fact that an item is an enum and simply treat it as the
base TokenDatatype.  Following are 2 examples where I needed to do this.

### #assembly_oscal-metadata_property
Convert the `name` property from enum to "#/definitions/TokenDatatype"

### #assembly_oscal-metadata_link
Convert the `rel` property from enum to "#/definitions/TokenDatatype"

### #field_oscal-metadata_document-id
Add `https://www.doi.org/` to the `scheme` enum property