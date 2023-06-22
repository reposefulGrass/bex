
pub mod convert;


fn main() {
    println!("Hello, world!");
}

#[cfg(test)]
mod tests {
    use crate::convert::hex_to_bin;

    #[test]
    fn basic() {
        assert_eq!(
            hex_to_bin("01 23 45 67 89"),
            vec![0x01_u8, 0x23, 0x45, 0x67, 0x89]
        );
    }
}
