//! Reference https://bitvmx.org/knowledge/optimizing-algorithms-for-bitcoin-script

// Define the nibble type
pub type u4 = u8;

// shift table
// pre-calculate the table of results for shifting each nibble value from 0 to 15 to the right by 1 bit.
fn pre_compute_left_shift() -> Vec<u4> {
    let mut res = vec![];
    for x in (0..16).rev() {
        let y = x >> 1;
        res.push(y);
    }
    res
}
// As right shift will outer than 0xF, so need to bitand to keep the result in nibble.
fn pre_compute_right_shift() -> Vec<u4> {
    let mut res = vec![];
    for x in (0..16).rev() {
        let y = (x << 1) & 0xF;
        res.push(y);
    }
    res
}

// A<<round
fn nibble_left_shift(nibble: u4, round: u8) -> u4 {
    assert!(nibble < 16);
    assert!(round < 4);
    let table = pre_compute_left_shift();

    let mut cur = nibble;
    for _ in 0..round {
        let index = 15 - cur;
        cur = table[index as usize];
    }

    cur
}

// A>>round
fn nibble_right_shift(nibble: u4, round: u8) -> u8 {
    assert!(nibble < 16);
    assert!(round < 4);
    let table = pre_compute_right_shift();
    println!("table: {:?}", table);
    let mut cur = nibble;
    for _ in 0..round {
        println!("cur: {cur}");
        let index = 15 - cur;
        cur = table[index as usize];
    }

    cur
}

fn byte_to_nibbles(byte: u8) -> (u4, u4) {
    let high_nibble = (byte >> 4) & 0x0F;
    let low_nibble = byte & 0x0F;
    (high_nibble, low_nibble)
}

fn from_niblles_to_byte(high_nibble: u4, low_nibble: u4) -> u8 {
    (high_nibble << 4) + low_nibble
}

// shift bytes. As bytes can be split two nibbles.
// YX >> 1 == (Y << 3 + X>> 1) & 0xF
fn byte_right_shift(byte: u8) -> u8 {
    let (y, x) = byte_to_nibbles(byte);

    nibble_right_shift(y, 3) + nibble_left_shift(x, 1)
}

// If you want to calculate 0xF AND 0x1 = 0x1.
// Then you can shift left the 0xF with one table to convert it to 0xF0,
// then add the second nibble: 0xF0 + 0x1 = 0xF1. You can then use this value (0xF1=241) as the position in the table to obtain the result (0x01).
fn pre_compute_nibble_bitand_simple_table() -> Vec<u4> {
    let mut table = vec![];
    for y in (0..16).rev() {
        for x in (0..16).rev() {
            table.push(x & y)
        }
    }
    table
}

fn nibble_bit_and(x: u4, y: u4) -> u4 {
    // x<<1 nibble -> 0xX0
    let index = from_niblles_to_byte(x, y);

    let table = pre_compute_nibble_bitand_simple_table();
    table[(255 - index) as usize]
}

// (a xor b) = (a + b) - 2*(a and b)
fn nibble_bit_xor(x: u4, y: u4) -> u4 {
    let x_and_b = nibble_bit_and(x, y);

    x + y - 2 * x_and_b
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_nibble_bit_and() {
        let table = pre_compute_nibble_bitand_simple_table();
        println!("table: {:?}", table);

        let X = 0xF;
        let Y = 0x1;

        assert_eq!(nibble_bit_and(X, Y), X & Y, "nibble_bit_and failed")
    }
    #[test]
    fn test_nibble_bit_xor() {
        let table = pre_compute_nibble_bitand_simple_table();
        println!("table: {:?}", table);

        let X = 0xF;
        let Y = 0x1;

        assert_eq!(nibble_bit_xor(X, Y), X ^ Y, "nibble_bit_xor failed")
    }

    #[test]
    fn test_nibble_shift_with_table() {
        let a = 8;
        assert_eq!(nibble_left_shift(a, 1), a >> 1, "nibble_left_shift failed");
        assert_eq!(nibble_left_shift(a, 2), a >> 2, "nibble_left_shift failed");
        assert_eq!(nibble_left_shift(a, 3), a >> 3, "nibble_left_shift failed");

        assert_eq!(
            nibble_right_shift(a, 1),
            (a << 1) & 0xF,
            "nibble_right_shift failed"
        );
        assert_eq!(
            nibble_right_shift(a, 2),
            (a << 2) & 0xF,
            "nibble_right_shift failed"
        );
        assert_eq!(
            nibble_right_shift(a, 3),
            (a << 3) & 0xF,
            "nibble_right_shift failed"
        );
        println!("a<<3: {:?}", nibble_right_shift(3, 3));

        let bytes = 0x32;
        println!("hex: {:X}", bytes);
        assert_eq!(
            byte_right_shift(bytes),
            (bytes >> 1) & &0xF,
            "byte_right_shift failed"
        );
    }

    #[test]
    fn test_convert_between_byte_and_nibbles() {
        // let byte = 0xAB; // binary 10101011
        let byte = 0x32; // binary 10101011
        println!("byte: {:X}", byte);
        let (high_nibble, low_nibble) = byte_to_nibbles(byte);
        println!(
            "High Nibble: {:02X}, Low Nibble: {:02X}",
            high_nibble, low_nibble
        );

        let actual = from_niblles_to_byte(high_nibble, low_nibble);
        println!("actual: {:X}", actual);
        assert_eq!(byte, actual);
    }
}
