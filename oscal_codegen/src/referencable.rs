use crate::SchemaId;
use core::fmt::Debug;

pub trait Referencable: Debug + Clone {
    fn id(&self) -> &SchemaId;
}
