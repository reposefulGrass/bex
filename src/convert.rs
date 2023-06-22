
use nom::IResult;


fn parse_hex(hex_str: &str) -> IResult<&str, u8> {
    Ok(("", 0))
}

pub fn hex_to_bin (hex_str: &str) -> Vec<u8> {
    let mut bin: Vec<u8> = vec![];
    let mut hex_str = hex_str.to_owned();

    while hex_str.trim() != "" {
        let (temp_str, byte) = parse_hex(&hex_str).expect("Invalid Hex Byte `{}` at position `{}`.");
        hex_str = temp_str.to_string();
        bin.push(byte);
    }

    bin
}