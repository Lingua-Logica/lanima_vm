use std::ops::Add;

use crate::{err::Error, vm::heap::heap_object::HeapVal};

use enum_dispatch::enum_dispatch;

use crate::object::str_val::StrVal;

#[derive(Clone, Debug)]
pub enum Addable {
    Str(StrVal)
}

impl Add for Addable {
    type Output = Result<HeapVal, Error>;

    fn add(self, rhs: Self) -> Self::Output {
        match (self, rhs) {
            (Addable::Str(l), Addable::Str(r)) => Ok(HeapVal::StrVal(l + r))
        }
    }
}

pub trait IntoOption<T> {
    fn into_option(self) -> Option<T>;
}

pub trait Inspect {
    fn inspect(&self) -> Box<str>;
}

#[enum_dispatch]
pub trait IObject: Inspect + IntoOption<Addable> {}