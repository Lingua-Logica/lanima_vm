use crate::object::object_trait::{Addable, IObject, Inspect, IntoOption};

#[derive(Clone, Debug)]
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

impl IObject for StrVal {}