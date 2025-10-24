use std::rc::Rc;

#[derive(Debug, Clone)]
pub struct VmError {
    message: Rc<str>
}

impl VmError {
    pub fn new(message: &str) -> Self {
        Self {
            message: message.into()
        }
    }
}

impl From<&str> for VmError {
    fn from(value: &str) -> Self {
        Self::new(value)
    }
}