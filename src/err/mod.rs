use crate::err::type_err::TypeError;

pub mod type_err;
pub mod vm_err;

pub enum Error {
    TypeError(TypeError),
}