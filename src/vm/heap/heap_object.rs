use std::rc::Rc;

use crate::vm::heap::str_val::StrVal;

pub trait Inspect {
    fn inspect(&self) -> Box<str>;
}

#[derive(Debug, Clone)]
pub enum HeapVal {
    Str(StrVal),
}

impl Inspect for HeapVal {
    fn inspect(&self) -> Box<str> {
        match self {
            HeapVal::Str(s) => s.clone().into()
        }
    }
}

pub enum Addable {
    Str(StrVal)
}

#[derive(Debug, Clone)]
pub struct HeapObject {
    val: HeapVal,
    add_func: Option<fn(Addable, Addable) -> HeapObject>,
}