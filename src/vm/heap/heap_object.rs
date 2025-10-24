use enum_dispatch::enum_dispatch;

use crate::object::{object_trait::{Addable, Inspect, IntoOption}, str_val::StrVal};

#[enum_dispatch(IObject, Inspect, IntoOption<Addable>)]
#[derive(Debug, Clone)]
pub enum HeapVal {
    StrVal,
}

macro_rules! auto_impl {
    ($self:ident, $method:ident) => {
        match $self {
            HeapVal::StrVal(it) => it.$method(),
        }
    };
}

impl Inspect for HeapVal {
    fn inspect(&self) -> Box<str> {
        auto_impl!(self, inspect)
    }
}

impl IntoOption<Addable> for HeapVal {
    fn into_option(self) -> Option<Addable> {
        auto_impl!(self, into_option)
    }
}