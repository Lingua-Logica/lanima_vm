use clap::Parser;
use colored::Colorize;

use crate::{args::Args, run::run_bytecode_file};

mod args;
mod compile;
mod err;
mod loader;
mod object;
mod run;
mod vm;

fn main() {
    let args = Args::parse();

    if let Some(bytecode_file) = args.bytecode {
        let result = run_bytecode_file(bytecode_file);

        if let Err(err) = result {
            eprintln!("{}", format!("{err}").red())
        }
    }

    // if let Err(err)  = crate::compile::compile_to_file_from_bytecode(
    //     std::path::Path::new("./test.lmc"),
    //     &[
    //         vm::code::make(vm::code::op_code::OpCode::MOVRI, &vec![0, 91]),
    //         vm::code::make(vm::code::op_code::OpCode::PrintReg, &vec![0])
    //     ]
    // ) {
    //     eprintln!("{}", format!("{err}").red())
    // }
}
