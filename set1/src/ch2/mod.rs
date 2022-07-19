use hex::{encode, FromHex};

pub fn fix_len_xor(hex1: &str, hex2: &str) -> Result<String, String> {
    let vec1 = Vec::from_hex(hex1).map_err(|err| err.to_string())?;
    let vec2 = Vec::from_hex(hex2).map_err(|err| err.to_string())?;
    if vec1.len() != vec2.len() {
        return Err("the hex valus have different length.".to_string());
    }
    let vec3: Vec<u8> = vec1
        .iter()
        .zip(vec2.iter())
        .map(|(&b1, &b2)| b1 ^ b2)
        .collect();
    Ok(encode(vec3))
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn fix_len_xor_sucess() {
        let str1 = "1c0111001f010100061a024b53535009181c";
        let str2 = "686974207468652062756c6c277320657965";
        let expected = "746865206b696420646f6e277420706c6179";
        let actual = fix_len_xor(str1, str2).unwrap();
        assert_eq!(expected, actual);
    }
}
