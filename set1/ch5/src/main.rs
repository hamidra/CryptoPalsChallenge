use hex::encode;
fn encrypt_xor(text: &str, key: &str) -> String {
    let text_bytes = text.as_bytes();
    let key_bytes = key.as_bytes();
    let mut encrypted: Vec<u8> = Vec::new();
    let mut i = 0;
    for byte in text_bytes {
        encrypted.push(byte ^ key_bytes[i]);
        i = (i + 1) % key_bytes.len();
    }
    encode(encrypted)
}

fn main() {
    let cipher = encrypt_xor(
        "Burning 'em, if you ain't quick and nimble 
        I go crazy when I hear a cymbal",
        "ICE",
    );
    println!("{}", cipher);
}
