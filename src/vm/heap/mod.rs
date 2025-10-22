pub mod str_val;
use std::{cell::RefCell, rc::Rc};

use crate::vm::heap::heap_object::HeapObject;

pub mod heap_object;

#[derive(Clone, Debug)]
pub struct Heap(Vec<Rc<RefCell<HeapObject>>>);

impl Heap {
    pub fn get(&self, i: usize) -> Rc<RefCell<HeapObject>> {
        self.0[i].clone()
    }

    pub fn get_ref(&self, i: usize) -> &Rc<RefCell<HeapObject>> {
        &self.0[i]
    }
}