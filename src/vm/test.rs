#[cfg(test)]
mod tests {
    use colored::Colorize;

    use crate::vm::{
        code::{
            make,
            op_code::{Op, OpCode},
        },
        vm::Vm,
    };

    fn expected_reg(
        code: &[Op],
        expected_reg_index: usize,
        expected_reg_value: i64
    ) {
        let mut vm = Vm::new(code);

        if let Err(err) = vm.run() {
            panic!("{}", format!("vm error: {err:#?}").red())
        }

        let reg = vm.reg[expected_reg_index];

        assert_eq!(reg, expected_reg_value);
        println!("REG{expected_reg_index} ({reg}) == {expected_reg_value}");
    }

    #[test]
    fn test_addi() {
        let code = &[
            make(OpCode::ADDI, &vec![0, 78]),
            make(OpCode::ADDI, &vec![0, 13]),
        ];

        expected_reg(code, 0, 91);
    }

    #[test]
    fn test_subi() {
        let code = &[
            make(OpCode::MOVRI, &vec![0, 3]),
            make(OpCode::SUBI, &vec![0, 7]),
        ];

        expected_reg(code, 0, -4);
    }

    #[test]
    fn test_muli() {
        let code = &[
            make(OpCode::MOVRI, &vec![0, 3]),
            make(OpCode::MULI, &vec![0, 3]),
        ];

        expected_reg(code, 0, 9);
    }

    #[test]
    fn test_divi() {
        let code = &[
            make(OpCode::MOVRI, &vec![0, 3]),
            make(OpCode::DIVI, &vec![0, 3]),
        ];

        expected_reg(code, 0, 1);
    }

    #[test]
    fn test_movrr() {
        let code = &[
            make(OpCode::ADDI, &vec![0, 91]),
            make(OpCode::MOVRR, &vec![1, 0]),
        ];

        expected_reg(code, 1, 91);
    }

    #[test]
    fn test_movri() {
        let code = &[
            make(OpCode::MOVRI, &vec![0, 91]),
        ];

        expected_reg(code, 0, 91);
    }

    #[test]
    fn test_simple_program() {
        let code = &[];

        let mut vm = Vm::new(code);

        if let Err(err) = vm.run() {
            panic!("{}", format!("vm error: {err:#?}").red())
        }
    }
}
