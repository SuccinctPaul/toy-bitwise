fn dec_to_hex(dec_value: u8) -> String {
    format!("{:X}", dec_value)
}

#[test]
fn test_dec_to_hex() {
    let dec_value = 255;
    let hex_str = dec_to_hex(dec_value);
    println!("Hexadecimal value: {}", hex_str);
}
