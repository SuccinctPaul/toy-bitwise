// Bitwise XOR for two u8 words, implemented with a lookup table for the helper function
// ```
//  f(x) = (x & 0b10101010) >> 1
// ```
// , which allows us in combination with OP_ADD and OP_SUB, to express bitwise XOR.
//
// Reference: https://github.com/BitVM/bitvm-js/blob/main/docs/opcodes/u8_xor.md

use crate::u4::u4;

fn helper(x: u8) -> u8 {
    (x & 0b10101010) >> 1
}

fn pre_compile_xor_table() -> Vec<u8> {
    let mut table = vec![];
    for i in (0..=u8::MAX).rev() {
        table.push(helper(i));
    }
    table
}

fn u8_xor(a: u8, b: u8) -> u8 {
    let f_a = helper(a);
    let a_even = f_a << 1;
    let a_odd = a - a_even;

    let f_b = helper(b);
    let b_even = f_b << 1;
    let b_odd = b - b_even;

    let a_andxor_b_even = f_a + f_b;
    let a_xor_b_even = a_andxor_b_even - (helper(a_andxor_b_even) << 1);

    let a_andxor_b_odd = a_odd + b_odd;
    let a_xor_b_odd = a_andxor_b_odd - (helper(a_andxor_b_odd) << 1);

    let a_xor_b = a_xor_b_odd + (a_xor_b_even << 1);
    a_xor_b
}
fn u8_or(a: u8, b: u8) -> u8 {
    let f_a = helper(a);
    let a_even = f_a << 1;
    let a_odd = a - a_even;

    let f_b = helper(b);
    let b_even = f_b << 1;
    let b_odd = b - b_even;

    let a_andxor_b_even = f_a + f_b;
    let a_xor_b_even = a_andxor_b_even - (helper(a_andxor_b_even));

    let a_andxor_b_odd = a_odd + b_odd;
    let a_xor_b_odd = a_andxor_b_odd - (helper(a_andxor_b_odd));

    let a_xor_b = a_xor_b_odd + (a_xor_b_even << 1);
    a_xor_b
}
fn u8_and(a: u8, b: u8) -> u8 {
    let f_a = helper(a);
    let a_even = f_a << 1;
    let a_odd = a - a_even;

    let f_b = helper(b);
    let b_even = f_b << 1;
    let b_odd = b - b_even;

    let a_andxor_b_even = f_a + f_b;
    let a_xor_b_even = helper(a_andxor_b_even);

    let a_andxor_b_odd = a_odd + b_odd;
    let a_xor_b_odd = helper(a_andxor_b_odd);

    let a_xor_b = a_xor_b_odd + (a_xor_b_even << 1);
    a_xor_b
}

pub fn u8_bit_xor(a: u8, b: u8) -> u8 {
    let table = pre_compile_xor_table();

    let index = u8::MAX - a;
    let f_a = table[index as usize];

    let a_even = f_a << 1;
    let a_odd = a - a_even;

    let index = u8::MAX - b;
    let f_b = table[index as usize];
    let b_even = f_b << 1;
    let b_odd = b - b_even;

    let a_andxor_b_even = f_a + f_b;
    let index = u8::MAX - a_andxor_b_even;
    let a_xor_b_even = a_andxor_b_even - (table[index as usize] << 1);

    let a_andxor_b_odd = a_odd + b_odd;
    let index = u8::MAX - a_andxor_b_odd;
    let a_xor_b_odd = a_andxor_b_odd - (table[index as usize] << 1);

    let a_xor_b = a_xor_b_odd + (a_xor_b_even << 1);
    a_xor_b
}

pub fn u8_bit_or(a: u8, b: u8) -> u8 {
    let table = pre_compile_xor_table();

    let index = u8::MAX - a;
    let f_a = table[index as usize];

    let a_even = f_a << 1;
    let a_odd = a - a_even;

    let index = u8::MAX - b;
    let f_b = table[index as usize];
    let b_even = f_b << 1;
    let b_odd = b - b_even;

    let a_andxor_b_even = f_a + f_b;
    let index = u8::MAX - a_andxor_b_even;
    let a_xor_b_even = a_andxor_b_even - (table[index as usize]);

    let a_andxor_b_odd = a_odd + b_odd;
    let index = u8::MAX - a_andxor_b_odd;
    let a_xor_b_odd = a_andxor_b_odd - (table[index as usize]);

    let a_xor_b = a_xor_b_odd + (a_xor_b_even << 1);
    a_xor_b
}

pub fn u8_bit_and(a: u8, b: u8) -> u8 {
    let table = pre_compile_xor_table();

    let f_a = table[(u8::MAX - a) as usize];
    let a_even = f_a << 1;
    let a_odd = a - a_even;

    let f_b = table[(u8::MAX - b) as usize];
    let b_even = f_b << 1;
    let b_odd = b - b_even;

    let a_andxor_b_even = f_a + f_b;
    let a_and_b_even = table[(u8::MAX - a_andxor_b_even) as usize];

    let a_andxor_b_odd = a_odd + b_odd;
    let index = u8::MAX - a_andxor_b_odd;
    let a_and_b_odd = table[index as usize];

    let a_xor_b = a_and_b_odd + (a_and_b_even << 1);
    a_xor_b
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_u8_xor() {
        let A = 0b00101010;
        let B = 0b10100100;

        let actual = u8_xor(A, B);
        let expect = A ^ B;
        assert_eq!(expect, actual);

        let A = 0xd8;
        let B = 0xb6;

        let actual = u8_xor(A, B);
        let expect = A ^ B;
        assert_eq!(expect, actual);
        let actual = u8_bit_xor(A, B);
        assert_eq!(expect, actual);
    }

    #[test]
    fn test_u8_or() {
        let A = 0xd8;
        let B = 0xb6;

        // let actual = u8_bit_or(A, B);
        let actual = u8_or(A, B);
        let expect = A | B;
        assert_eq!(expect, actual);

        let actual = u8_bit_or(A, B);
        let expect = A | B;
        assert_eq!(expect, actual);
    }

    #[test]
    fn test_u8_and() {
        let A = 0xd8;
        let B = 0xb6;

        // let actual = u8_bit_or(A, B);
        let actual = u8_and(A, B);
        let expect = A & B;
        assert_eq!(expect, actual);

        let actual = u8_bit_and(A, B);
        let expect = A & B;
        assert_eq!(expect, actual);
    }

    #[test]
    fn test_u8_xor_table() {
        for (i, actual) in (0..=u8::MAX).rev().zip(pre_compile_xor_table()) {
            let expect = helper(i);
            println!("f({i})={:02X}", expect);
            assert_eq!(expect, actual);
        }
    }
}
