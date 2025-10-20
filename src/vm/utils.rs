use byteorder::{BigEndian, ByteOrder};

#[inline(always)]
pub fn read_i64(bytes: &[u8]) -> i64 {
    BigEndian::read_i64(bytes)
}