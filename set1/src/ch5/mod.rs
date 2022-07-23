use hex::encode;
pub fn encrypt_xor(text: &str, key: &str) -> String {
    let mut encrypted_bytes: Vec<u8> = Vec::new();
    let key_bytes = key.as_bytes();
    let text_bytes = text.as_bytes();
    for i in 0..text_bytes.len() {
        let k = i % key_bytes.len();
        encrypted_bytes.push(text_bytes[i] ^ key_bytes[k]);
    }
    encode(encrypted_bytes)
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn encrypt_with_xor_key_success() {
        let text = "Burning 'em, if you ain't quick and nimble\n\
                    I go crazy when I hear a cymbal";
        let key = "ICE";
        let expected =
            "0b3637272a2b2e63622c2e69692a23693a2a3c6324202d623d63343c2a26226324272765272a282b2f20430a652e2c652a3124333a653e2b2027630c692b20283165286326302e27282f";

        let actual = encrypt_xor(&text, &key);
        assert_eq!(expected, actual);
    }
}
