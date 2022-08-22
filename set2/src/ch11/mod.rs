use openssl::symm::{cipher, encrypt};
use rand::Rng;
pub fn keygen(key_length: usize) -> Vec<u8> {
    let mut rng = rand::thread_rng();
    let key = (1..key_length).map(|_| rng.gen::<u8>()).collect();
    key
}

pub fn encrypt_oracle(input: &str) -> Vec<u8> {
    // create a random 16 byte key
    let key = keygen(16);
    let mut rng = rand::thread_rng();
    let alg: bool = rng.gen_ratio(1, 2);
    // append
}

#[cfg(test)]
mod tests {
    use itertools::equal;

    use super::*;
    #[test]
    fn create_random_key_sucess() {
        let key_16_1 = keygen(16);
        let key_16_2 = keygen(16);
        assert!(equal(key_16_1, key_16_2));
    }
}
