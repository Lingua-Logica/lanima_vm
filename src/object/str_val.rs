use std::ops::Add;

use crate::object::object_trait::{Addable, IObject, Inspect, IntoOption};

#[derive(Clone, Debug, PartialEq, Eq, PartialOrd, Ord)]
pub struct StrVal(pub String);

impl Into<Box<str>> for StrVal {
    fn into(self) -> Box<str> {
        self.0.into()
    }
}

impl IntoOption<Addable> for StrVal {
    fn into_option(self) -> Option<Addable> {
        Some(Addable::Str(self))
    }
}

impl Inspect for StrVal {
    fn inspect(&self) -> Box<str> {
        self.0.clone().into()
    }
}

impl Add for StrVal {
    type Output = StrVal;

    fn add(self, rhs: Self) -> Self::Output {
        StrVal(self.0 + &rhs.0)
    }
}

impl IObject for StrVal {}