use std::error::Error;

use crate::{loader::init_loader, vm::vm::Vm};

pub fn run_bytecode_file(file_path: String) -> Result<(), Box<dyn Error>> {
    let loader = init_loader(&file_path)?;
    let bytecode = loader.load()?;

    let mut vm = Vm::new(&bytecode);

    vm.run()?;

    Ok(())
}