use base64::encode;
use hex::FromHex;
fn hex_to_base64(hex: &str) {
    let a: Vec<u8> = Vec::from_hex(hex).expect("Invalid Hex String");
    let b46: String = encode(a);
    print!("{:?}", b46);
}

fn main() {
    hex_to_base64("49276d206b696c6c696e6720796f757220627261696e206c696b65206120706f69736f6e6f7573206d757368726f6f6d");
}
