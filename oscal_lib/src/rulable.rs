use std::{any::Any, sync::Arc};

use crate::Error;

pub trait Rulable {
    fn parse_json(json: &str) -> Result<Arc<dyn Any + Sync + Send>, Error>;
    fn from_arc_any(any: Arc<dyn Any + Sync + Send>) -> Result<Arc<Self>, Error>;
    fn to_arc_any(self) -> Arc<dyn Any>;
}
