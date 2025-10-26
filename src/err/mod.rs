use std::fmt::Display;

use crate::err::{type_err::TypeError, vm_err::VmError};

pub mod type_err;
pub mod vm_err;

#[derive(Debug, Clone)]
pub enum Error {
    TypeError(TypeError),
    VmError(VmError)
}

impl Display for Error {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Error::TypeError(err) => err.fmt(f),
            Error::VmError(err) => err.fmt(f),
        }
    }
}