use openssl::{
    error::ErrorStack,
    symm::{encrypt, Cipher},
};
use rand::Rng;
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

pub fn encryption_oracle(input: &str) -> Vec<u8> {
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
    let do_ecb: bool = rng.gen_ratio(1, 2);

    let result = if do_ecb {
        encrypt_ecb(&key[..], &padded[..])
    } else {
        encrypt_cbc(&key[..], &padded[..])
    };
    result.unwrap()
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
        let cipher_text1 = encryption_oracle(test_text);
        println!("cipher text 1: {:?}", cipher_text1);
        let cipher_text2 = encryption_oracle(test_text);
        println!("cipher text 2: {:?}", cipher_text2);
    }
}
