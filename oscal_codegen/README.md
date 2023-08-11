# Generate Rust code from the OSCAL schema

## Issues
The OSCAL schema has a loose interpretation of `enum`. In some instances, it is
better to ignore the fact that an item is an enum and simply treat it as the
base TokenDatatype.  Following are 2 examples where I needed to do this.

### #assembly_oscal-metadata_property
Convert the `name` property from :

```json
"name": {
    "title": "Property Name",
    "description": "A textual label that uniquely identifies a specific attribute, characteristic, or quality of the property's containing object.",
    "allOf": [
        {
            "$ref": "#/definitions/TokenDatatype"
        },
        {
            "enum": [
                "marking"
            ]
        }
    ]
},
```

to:

```json
"name": {
    "title": "Property Name",
    "description": "A textual label that uniquely identifies a specific attribute, characteristic, or quality of the property's containing object.",
    "$ref": "#/definitions/TokenDatatype"
},
```

### #assembly_oscal-metadata_link
Convert the `rel` property from enum to "#/definitions/TokenDatatype"

```json
"rel": {
    "title": "Relation",
    "description": "Describes the type of relationship provided by the link. This can be an indicator of the link's purpose.",
    "allOf": [
        {
            "$ref": "#/definitions/TokenDatatype"
        },
        {
            "enum": [
                "reference"
            ]
        }
    ]
},
```
to:

```json
"rel": {
    "title": "Relation",
    "description": "Describes the type of relationship provided by the link. This can be an indicator of the link's purpose.",
    "$ref": "#/definitions/TokenDatatype"
},
```
### #field_oscal-metadata_document-id
Add `https://www.doi.org/` to the `scheme` enum property