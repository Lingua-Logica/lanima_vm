use crate::{
    err::{Error, vm_err::VmError},
    object::object_trait::Inspect,
    vm::{builtins::BuiltinFunctionResult, vm::Vm},
};

pub fn builtin_print(vm: &mut Vm) -> BuiltinFunctionResult<()> {
    let o = vm
        .heap
        .get_ref(vm.reg[1] as usize)
        .map_err(|msg| Error::VmError(VmError::new(&msg)))?;

    println!("{}", o.inspect());

    Ok(())
}
