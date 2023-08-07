use serde_json::Value;

use crate::{
    //
    NameSpace,
    Result,
    SchemaId,
};

pub trait Parse: Sized {
    fn parse(
        value: &Value,
        ns: &mut NameSpace,
        parent: Option<&SchemaId>,
        name: Option<&str>,
    ) -> Result<Self>;
}
