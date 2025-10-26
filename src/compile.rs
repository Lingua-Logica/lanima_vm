use std::{error::Error, fs, path::Path};

use bincode::{config, encode_to_vec};

use crate::{loader::{constants::{BYTECODE_HEAD2, VERSION}, Header}, vm::code::op_code::Op};

pub fn compile_to_file_from_bytecode(
    target: &Path,
    code: &[Op],
) -> Result<(), Box<dyn Error>> {
    let mut bytes = Vec::new();

    // 1. 追加 Header
    bytes.extend(
        encode_to_vec(
            &Header { head: BYTECODE_HEAD2, ver: VERSION },
            config::legacy().with_big_endian(),
        )?
    );

    // 2. 追加 code
    bytes.extend(
        encode_to_vec(code, config::legacy().with_big_endian())?
    );

    // 3. 落盘
    fs::write(target, bytes)?;

    Ok(())
}