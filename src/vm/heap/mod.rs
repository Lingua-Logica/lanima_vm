use std::{cell::RefCell, rc::Rc};

use crate::vm::heap::heap_object::HeapVal;

pub mod heap_object;

#[derive(Clone, Debug)]
pub struct Heap(Vec<Rc<RefCell<HeapVal>>>);

impl Heap {
    pub fn get(&self, i: usize) -> Rc<RefCell<HeapVal>> {
        self.0[i].clone()
    }

    pub fn get_ref(&self, i: usize) -> &Rc<RefCell<HeapVal>> {
        &self.0[i]
    }
}