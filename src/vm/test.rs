#[cfg(test)]
mod tests {
    use crate::vm::{code::{make, op_code::OpCode}, vm::Vm};

    #[test]
    fn test_addi() {
        let code = &[
            make(OpCode::ADDI, &vec![0, 78]),
            make(OpCode::ADDI, &vec![0, 13]),
            make(OpCode::PrintReg, &vec![0])
        ];

        let mut vm = Vm::new(code);

        if let Err(err) = vm.run() {
            println!("vm error: {err:#?}")
        }
    }

    #[test]
    fn test_movrr() {
        let code = &[
            make(OpCode::ADDI, &vec![0, 91]),
            make(OpCode::MOVRR, &vec![1, 0]),
            make(OpCode::PrintReg, &vec![0]),
            make(OpCode::PrintReg, &vec![1]),
        ];

        let mut vm = Vm::new(code);

        if let Err(err) = vm.run() {
            println!("vm error: {err:#?}")
        }
    }

    #[test]
    fn test_simple_program() {
        let code = &[];

        let mut vm = Vm::new(code);

        if let Err(err) = vm.run() {
            println!("vm error: {err:#?}")
        }
    }
}