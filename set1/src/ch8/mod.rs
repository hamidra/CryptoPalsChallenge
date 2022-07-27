use std::collections::HashMap;

pub fn detect_aes_ecb_cipher(ciphers: &Vec<Vec<u8>>) -> Option<(i32, &Vec<u8>)> {
    let mut result: Option<(i32, &Vec<u8>)> = None;
    for cipher in ciphers {
        let mut block_counter: HashMap<String, i32> = HashMap::new();
        cipher.chunks(16).for_each(|block| {
            let key = hex::encode(block);
            let count = block_counter.get(&key).unwrap_or(&0);
            block_counter.insert(key, count + 1);
        });
        let collision_count = block_counter.values().fold(0, |acc, val| acc + (val - 1));
        // println!("{:?}", block_counter);
        // println!("{}", collision_count);
        if let Some(r) = result {
            if r.0 < collision_count {
                result = Some((collision_count, &cipher));
            }
        } else {
            result = Some((collision_count, &cipher));
        }
    }
    result
}

#[cfg(test)]
mod test {
    use super::*;
    use crate::ch4::read_lines;
    const CIPHER_FILE: &str = "src/ch8/cipher.txt";
    const ECB_ENCODED_CIPHER: &str = "d880619740a8a19b7840a8a31c810a3d08649af70dc06f4fd5d2d69c744cd283e2dd052f6b641dbf9d11b0348542bb5708649af70dc06f4fd5d2d69c744cd2839475c9dfdbc1d46597949d9c7e82bf5a08649af70dc06f4fd5d2d69c744cd28397a93eab8d6aecd566489154789a6b0308649af70dc06f4fd5d2d69c744cd283d403180c98c8f6db1f2a3f9c4040deb0ab51b29933f2c123c58386b06fba186a";

    #[test]
    fn detect_aes_ecb_success() {
        let mut ciphers = Vec::new();
        let lines = read_lines(CIPHER_FILE).unwrap();
        for line in lines {
            let cipher_hex = line.unwrap();
            let cipher = hex::decode(cipher_hex).unwrap();
            ciphers.push(cipher);
        }
        let (_collision_count, aes_ecb) = detect_aes_ecb_cipher(&ciphers)
            .expect("could not find any ecb encoded ciphers in the cipher text.");
        assert_eq!(ECB_ENCODED_CIPHER, hex::encode(aes_ecb));
    }
}
