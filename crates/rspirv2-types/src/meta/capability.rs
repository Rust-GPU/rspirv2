use std::any::Any;
use std::fmt::Debug;

pub trait AnyCapability: Debug + Any {
    fn name(&self) -> String;
}
