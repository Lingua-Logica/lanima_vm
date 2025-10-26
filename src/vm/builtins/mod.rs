pub mod builtin_functions;

use crate::{err::Error, vm::{builtins::builtin_functions::builtin_print, vm::Vm}};

pub type BuiltinFunctionResult<T> = Result<T, Error>;

pub static BUILTINS: &'static [fn(&mut Vm) -> BuiltinFunctionResult<()>] = &[
    builtin_print
];