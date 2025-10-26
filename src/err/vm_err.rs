use std::{error::Error, fmt::Display, rc::Rc};

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

impl Display for VmError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "vm error: {}", self.message)
    }
}

impl Error for VmError {}