use std::rc::Rc;

pub trait Inspect {
    fn inspect(&self) -> Box<str>;
}

#[derive(Debug, Clone)]
pub enum HeapObject {
    Str(String)
}

impl Inspect for HeapObject {
    fn inspect(&self) -> Box<str> {
        match self {
            HeapObject::Str(s) => s.clone().into()
        }
    }
}