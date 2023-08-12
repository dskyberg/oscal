use convert_case::{Case, Casing};

use crate::{ParserError, Result, SchemaId};

/// Determine the ID for an object.
/// If there is a parent ID:
/// - If there is no local id, add it to the parent id
/// - If not, and there is a title, convert it to an id, and add to the parent
pub fn merge_ids(
    parent_id: Option<&SchemaId>,
    id_val: Option<&str>,
    title: &str,
) -> Result<SchemaId> {
    let id = match (id_val, parent_id) {
        // Use the local $id value
        (Some(local), None) => SchemaId::try_from(local).map_err(|e| {
            log::error!("Object $id failed to parse");
            e
        })?,
        // No local id.  Extend from parent_id
        (None, Some(parent_id)) => {
            // turn the title into a snake case string
            let name = title.to_case(Case::Kebab);
            //
            let raw = format!("{}_{}", parent_id.raw, name);
            SchemaId::try_from(raw.as_str()).map_err(|e| {
                log::error!("Object $id failed to parse");
                e
            })?
        }

        // There's a parent_id, and a local string.  This is odd.
        (Some(local), Some(pid)) =>
        // It's likely that this is a dup, from the schema.
        {
            match local == pid.raw {
                true => pid.clone(), // It's the parent id.  Just return it.
                false => {
                    // This is probably an error.
                    log::error!(
                        "Both a local id and a parent id were provided. Using local: {} - {:?}",
                        &local,
                        pid
                    );
                    return Err(ParserError::MergeFailed.into());
                }
            }
        }

        (None, None) => {
            log::debug!("Object with no $id field and no parent id");
            return Err(ParserError::MissingField("$id".to_string()).into());
        }
    };
    Ok(id)
}
