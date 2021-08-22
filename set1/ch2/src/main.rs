use hex::{encode, FromHex};
fn fix_len_xor(hex1: &str, hex2: &str) -> String {
    let vec1 = Vec::from_hex(hex1).expect("Invalid Hex String");
    let vec2 = Vec::from_hex(hex2).expect("Invalid Hex String");
    assert_eq!(vec1.len(), vec2.len());
    let vec3: Vec<u8> = vec1
        .iter()
        .zip(vec2.iter())
        .map(|(&b1, &b2)| b1 ^ b2)
        .collect();
    encode(vec3)
}

fn main() {
    let str1 = "1c0111001f010100061a024b53535009181c";
    let str2 = "686974207468652062756c6c277320657965";
    print!("{}", fix_len_xor(str1, str2));
}
