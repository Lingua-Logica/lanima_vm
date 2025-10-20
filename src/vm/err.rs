use std::rc::Rc;

#[derive(Debug, Clone)]
pub struct VmError {
    message: Rc<str>
}