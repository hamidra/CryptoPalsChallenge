use hex::FromHex;

pub fn hex_to_base64(hex_str: &str) -> Result<String, String> {
    let a: Vec<u8> = Vec::from_hex(hex_str).map_err(|err| err.to_string())?;
    Ok(base64::encode(a))
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn hex_to_base64_pass() {
        let test_case = "49276d206b696c6c696e6720796f757220627261696e206c696b65206120706f69736f6e6f7573206d757368726f6f6d";
        let expected = "SSdtIGtpbGxpbmcgeW91ciBicmFpbiBsaWtlIGEgcG9pc29ub3VzIG11c2hyb29t";
        let actual = hex_to_base64(test_case).unwrap();
        assert_eq!(expected, actual);
    }
}
