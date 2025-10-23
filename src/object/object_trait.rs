use crate::vm::heap::heap_object::HeapVal;

use enum_dispatch::enum_dispatch;

use crate::object::str_val::StrVal;

#[derive(Clone, Debug)]
pub enum Addable {
    Str(StrVal)
}

pub trait IntoOption<T> {
    fn into_option(self) -> Option<T>;
}

pub trait Inspect {
    fn inspect(&self) -> Box<str>;
}

#[enum_dispatch]
pub trait IObject: Inspect + IntoOption<Addable> {}