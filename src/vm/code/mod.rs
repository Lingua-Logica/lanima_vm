use crate::vm::code::op_code::{Op, OpCode, CODEDEF_MAP};

pub mod op_code;

#[inline(always)]
pub fn make(op: OpCode, operands: &Vec<i64>) -> Op {
    let definition = CODEDEF_MAP.get(&op);

    match definition {
        Some(def) => {
            let operand_width_len = def.operand_widths.iter().sum::<i32>();

            let mut operand_bytes = vec![0u8; operand_width_len as usize];

            let mut offset = 0;

            for (i, operand) in operands.iter().enumerate() {
                let width = def.operand_widths[i];

                match width {
                    8 => {
                        // 处理 8 字节操作数 (大端序)
                        let bytes = operand.to_be_bytes();
                        operand_bytes[offset..offset + 8].copy_from_slice(&bytes);
                    }

                    4 => {
                        // 处理 4 字节操作数 (大端序)
                        let bytes = operand.to_be_bytes();
                        operand_bytes[offset..offset + 4].copy_from_slice(&bytes);
                    }

                    2 => {
                        // 处理 2 字节操作数 (大端序)
                        let bytes = operand.to_be_bytes();
                        operand_bytes[offset..offset + 2].copy_from_slice(&bytes);
                    }

                    1 => {
                        // 处理单字节操作数 (大端序)
                        operand_bytes[offset] = *operand as u8
                    }

                    // 添加其他宽度处理...
                    _ => panic!("unsupported operand width: {}", width),
                }

                offset += width as usize;
            }

            Op::new(op, operand_bytes.into())
        }
        
        None => panic!("cannot found opcode def: {op:#?}"),
    }
}