use crate::u8::{u8_bit_and, u8_bit_or, u8_bit_xor};

// leverage power of u64
fn u64_bit_xor(a: u64, b: u64) -> u64 {
    let a_byte_be = a.to_be_bytes();
    let b_byte_be = b.to_be_bytes();

    let u8_xor_res = a_byte_be
        .into_iter()
        .zip(b_byte_be)
        .map(|(ai, bi)| u8_bit_xor(ai, bi))
        .collect::<Vec<u8>>();

    u64::from_be_bytes(u8_xor_res.try_into().unwrap())
}

fn u64_bit_or(a: u64, b: u64) -> u64 {
    let a_byte_be = a.to_be_bytes();
    let b_byte_be = b.to_be_bytes();

    let u8_xor_res = a_byte_be
        .into_iter()
        .zip(b_byte_be)
        .map(|(ai, bi)| u8_bit_or(ai, bi))
        .collect::<Vec<u8>>();

    u64::from_be_bytes(u8_xor_res.try_into().unwrap())
}

fn u64_bit_and(a: u64, b: u64) -> u64 {
    let a_byte_be = a.to_be_bytes();
    let b_byte_be = b.to_be_bytes();

    let u8_xor_res = a_byte_be
        .into_iter()
        .zip(b_byte_be)
        .map(|(ai, bi)| u8_bit_and(ai, bi))
        .collect::<Vec<u8>>();

    u64::from_be_bytes(u8_xor_res.try_into().unwrap())
}

#[cfg(test)]
mod test {

    use super::*;

    #[test]
    fn test_u64_bit_xor() {
        let a = 0x5553ff550a04d2c8;
        let b = 0xA1A2ffA3da3ff550;

        let actual = u64_bit_xor(a, b);
        assert_eq!(actual, a ^ b, "u64_bit_xor failed")
    }
    #[test]
    fn test_u64_bit_or() {
        let a = 0x5553ff550a04d2c8;
        let b = 0xA1A2ffA3da3ff550;

        let actual = u64_bit_or(a, b);
        assert_eq!(actual, a | b, "u64_bit_or failed");
    }

    #[test]
    fn test_u64_bit_and() {
        let a = 0x5553ff550a04d2c8;
        let b = 0xA1A2ffA3da3ff550;

        let actual = u64_bit_and(a, b);
        assert_eq!(actual, a & b, "u64_bit_and failed");
    }
}
