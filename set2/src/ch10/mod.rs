use aes::cipher::{generic_array::GenericArray, BlockDecrypt, KeyInit};
use aes::Aes128;
use std::error::Error;
pub fn decrypt_aes_128(cipher_bytes: &[u8], key: &[u8]) -> Result<Vec<u8>, Box<dyn Error>> {
    let fixed_sized_key = GenericArray::clone_from_slice(key);
    let cipher = Aes128::new(&fixed_sized_key);
    let mut block = GenericArray::clone_from_slice(cipher_bytes);
    cipher.decrypt_block(&mut block);
    Ok(block.to_vec())
}

pub fn byte_vec_xor(vec1: &[u8], vec2: &[u8]) -> Vec<u8> {
    vec1.iter()
        .zip(vec2.iter())
        .map(|(b1, b2)| b1 ^ b2)
        .collect()
}

pub fn decrypt_aes_128_cbc(
    cipher_bytes: &Vec<u8>,
    key: &[u8],
    iv: &[u8],
) -> Result<Vec<u8>, Box<dyn Error>> {
    let mut decrypted = Vec::new();
    let mut prev = iv.to_vec();
    cipher_bytes.chunks(key.len()).for_each(|block| {
        let cbc_block = decrypt_aes_128(block, key).unwrap();
        let cbc_decrypted_block = byte_vec_xor(&cbc_block[..], &prev[..]);
        prev = block.to_vec();
        decrypted.extend(cbc_decrypted_block);
    });
    let padding_len = decrypted.pop().unwrap_or(0);
    let unpadded_len = decrypted.len() - padding_len as usize;
    decrypted = decrypted.into_iter().take(unpadded_len).collect();
    Ok(decrypted)
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::str;
    use std::{fs::File, io::Read};
    const CIPHER_FILE: &str = "src/ch10/encrypted.txt";
    const DECRYPTED:&str = "I'm back and I'm ringin' the bell \nA rockin' on the mike while the fly girls yell \nIn ecstasy in the back of me \nWell that's my DJ Deshay cuttin' all them Z's \nHittin' hard and the girlies goin' crazy \nVanilla's on the mike, man I'm not lazy. \n\nI'm lettin' my drug kick in \nIt controls my mouth and I begin \nTo just let it flow, let my concepts go \nMy posse's to the side yellin', Go Vanilla Go! \n\nSmooth 'cause that's the way I will be \nAnd if you don't give a damn, then \nWhy you starin' at me \nSo get off 'cause I control the stage \nThere's no dissin' allowed \nI'm in my own phase \nThe girlies sa y they love me and that is ok \nAnd I can dance better than any kid n' play \n\nStage 2 -- Yea the one ya' wanna listen to \nIt's off my head so let the beat play through \nSo I can funk it up and make it sound good \n1-2-3 Yo -- Knock on some wood \nFor good luck, I like my rhymes atrocious \nSupercalafragilisticexpialidocious \nI'm an effect and that you can bet \nI can take a fly girl and make her wet. \n\nI'm like Samson -- Samson to Delilah \nThere's no denyin', You can try to hang \nBut you'll keep tryin' to get my style \nOver and over, practice makes perfect \nBut not if you're a loafer. \n\nYou'll get nowhere, no place, no time, no girls \nSoon -- Oh my God, homebody, you probably eat \nSpaghetti with a spoon! Come on and say it! \n\nVIP. Vanilla Ice yep, yep, I'm comin' hard like a rhino \nIntoxicating so you stagger like a wino \nSo punks stop trying and girl stop cryin' \nVanilla Ice is sellin' and you people are buyin' \n'Cause why the freaks are jockin' like Crazy Glue \nMovin' and groovin' trying to sing along \nAll through the ghetto groovin' this here song \nNow you're amazed by the VIP posse. \n\nSteppin' so hard like a German Nazi \nStartled by the bases hittin' ground \nThere's no trippin' on mine, I'm just gettin' down \nSparkamatic, I'm hangin' tight like a fanatic \nYou trapped me once and I thought that \nYou might have it \nSo step down and lend me your ear \n'89 in my time! You, '90 is my year. \n\nYou're weakenin' fast, YO! and I can tell it \nYour body's gettin' hot, so, so I can smell it \nSo don't be mad and don't be sad \n'Cause the lyrics belong to ICE, You can call me Dad \nYou're pitchin' a fit, so step back and endure \nLet the witch doctor, Ice, do the dance to cure \nSo come up close and don't be square \nYou wanna battle me -- Anytime, anywhere \n\nYou thought that I was weak, Boy, you're dead wrong \nSo come on, everybody and sing this song \n\nSay -- Play that funky music Say, go white boy, go white boy go \nplay that funky music Go white boy, go white boy, go \nLay down and boogie and play that funky music till you die. \n\nPlay that funky music Come on, Come on, let me hear \nPlay that funky music white boy you say it, say it \nPlay that funky music A little louder now \nPlay that funky music, white boy Come on, Come on, Come on \nPlay that funky music ";
    const KEY: &[u8] = b"YELLOW SUBMARINE";
    const IV: &[u8] = b"\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00";
    #[test]
    fn encrypt_cbc_success() {
        let mut file = File::open(CIPHER_FILE).unwrap();
        let mut cipher_text = String::new();
        file.read_to_string(&mut cipher_text).unwrap();
        let cipher_text = cipher_text.replace("\n", "");
        let cipher_bytes = base64::decode(cipher_text).unwrap();
        let decrypted_bytes = decrypt_aes_128_cbc(&cipher_bytes, KEY, IV).unwrap();
        let decrypted_text = str::from_utf8(&decrypted_bytes).unwrap();
        assert_eq!(DECRYPTED, decrypted_text);
    }
}
