use openssl::{
    error::ErrorStack,
    symm::{encrypt, Cipher},
};
use rand::Rng;
use set1::ch8::detect_aes_ecb_cipher;

pub fn keygen(key_length: usize) -> Vec<u8> {
    let mut rng = rand::thread_rng();
    let key = (0..key_length).map(|_| rng.gen::<u8>()).collect();
    key
}

pub fn encrypt_cbc(key: &[u8], data: &[u8]) -> Result<Vec<u8>, ErrorStack> {
    // generate random iv to the size of key
    let mut rng = rand::thread_rng();
    let iv = (0..key.len()).map(|_| rng.gen::<u8>()).collect::<Vec<u8>>();

    let cipher = Cipher::aes_128_cbc();
    encrypt(cipher, key, Some(&iv[..]), data)
}

pub fn encrypt_ecb(key: &[u8], data: &[u8]) -> Result<Vec<u8>, ErrorStack> {
    let cipher = Cipher::aes_128_ecb();
    encrypt(cipher, key, None, data)
}

pub fn encryption_oracle(input: &str) -> (bool, Vec<u8>) {
    // create a random 16 byte key
    let key = keygen(16);

    let mut rng = rand::thread_rng();

    // append 5-10 bytes padding
    let padding_len = rng.gen_range(5..10);
    let padding = vec![0; padding_len];
    let mut padded: Vec<u8> = Vec::new();
    padded.extend(padding.clone());
    padded.extend(input.as_bytes());
    padded.extend(padding.clone());

    // pick EBC half time and CBC the other half
    let is_ecb: bool = rng.gen_ratio(1, 2);

    let result = if is_ecb {
        encrypt_ecb(&key[..], &padded[..])
    } else {
        encrypt_cbc(&key[..], &padded[..])
    };
    (is_ecb, result.unwrap())
}

#[cfg(test)]
mod tests {
    use itertools::equal;

    use super::*;
    #[test]
    fn create_random_key_sucess() {
        let key_16_1 = keygen(16);
        let key_16_2 = keygen(16);
        assert!(!equal(key_16_1, key_16_2));
    }

    #[test]
    fn encryption_oracle_sucess() {
        let test_text = "This is test text to be enctypted by oracle!!";
        let (is_ecb1, cipher_text1) = encryption_oracle(test_text);
        println!(
            "cipher text 1: {:?}, is_ecb?:{}",
            base64::encode(cipher_text1),
            is_ecb1
        );
        let (is_ecb2, cipher_text2) = encryption_oracle(test_text);
        println!(
            "cipher text 2: {:?}, is_ecb?:{}",
            base64::encode(cipher_text2),
            is_ecb2
        );
    }
    #[test]
    fn detect_ecb_success() {
        let oracle_text: String = vec!['A'; 4 * 16].into_iter().collect();
        for n in 1..1000 {
            let (is_ecb, cipher_text) = encryption_oracle(&oracle_text[..]);
            let cipher_texts = &vec![cipher_text];
            let mut ecb_detected = false;
            if let Some(ecb_detection) = detect_aes_ecb_cipher(cipher_texts) {
                ecb_detected = ecb_detection.0 > 1;
            }

            assert_eq!(is_ecb, ecb_detected);
        }
    }
}
