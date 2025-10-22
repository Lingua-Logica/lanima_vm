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
        println!("{}", format!("REG{expected_reg_index} ({reg}) == {expected_reg_value}").green());
    }

    #[test]
    fn test_add() {
        let code = &[
            make(OpCode::ADDI, &vec![0, 78]),
            make(OpCode::ADDI, &vec![0, 13]),
            make(OpCode::ADDR, &vec![1, 0]),
        ];

        let expected = [
            (0, 91),
            (1, 91),
        ];

        for (reg, expected_value) in expected {
            expected_reg(code, reg, expected_value);
        }
    }

    #[test]
    fn test_sub() {
        let code = &[
            make(OpCode::MOVRI, &vec![0, 3]),
            make(OpCode::SUBI, &vec![0, 7]),
            make(OpCode::SUBR, &vec![1, 0]),
        ];

        let expected = [
            (0, -4),
            (1, 4),
        ];

        for (reg, expected_value) in expected {
            expected_reg(code, reg, expected_value);
        }
    }

    #[test]
    fn test_mul() {
        let code = &[
            make(OpCode::MOVRI, &vec![0, 3]),
            make(OpCode::MULI, &vec![0, 3]),
            make(OpCode::MOVRI, &vec![1, 10]),
            make(OpCode::MULR, &vec![1, 0]),
        ];
        
        let expected = [
            (0, 9),
            (1, 90),
        ];

        for (reg, expected_value) in expected {
            expected_reg(code, reg, expected_value);
        }
    }

    #[test]
    fn test_div() {
        let code = &[
            make(OpCode::MOVRI, &vec![0, 3]),
            make(OpCode::DIVI, &vec![0, 3]),
            make(OpCode::MOVRI, &vec![1, 3]),
            make(OpCode::DIVR, &vec![1, 0]),
        ];

        let expected = [
            (0, 1),
            (1, 3),
        ];

        for (reg, expected_value) in expected {
            expected_reg(code, reg, expected_value);
        }
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
