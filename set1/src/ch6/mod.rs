use crate::ch3;
use itertools::Itertools;
use std::error::Error;
use std::fs::File;
use std::io::BufReader;
use std::io::Read;
use std::iter::zip;
use std::path::Path;
use std::str;

pub fn calc_hamming_dist(bytes1: &[u8], bytes2: &[u8]) -> u32 {
    let bytes_zipped = zip(bytes1, bytes2);
    let mut dist = 0;
    for (b1, b2) in bytes_zipped {
        dist += (b1 ^ b2).count_ones();
    }
    dist
}

pub fn read_b64_content_as_bytes(file_path: &str) -> Result<Vec<u8>, Box<dyn Error>> {
    // read the content
    let file = File::open(Path::new(file_path))?;
    let mut buf_reader = BufReader::new(file);
    let mut content_b64 = String::new();
    buf_reader.read_to_string(&mut content_b64)?;

    // filter new lines
    content_b64 = content_b64
        .chars()
        .filter(|c| *c != '\n')
        .collect::<String>();
    let content_bytes = base64::decode(content_b64)?;
    Ok(content_bytes)
}

pub fn find_keysize(content: &Vec<u8>) -> Result<Vec<usize>, Box<dyn Error>> {
    let result_count = 1;
    let sample_size = 4;
    let mut keysizes = Vec::new();
    for k in 2..=40 {
        let mut samples = Vec::new();
        (0..sample_size).for_each(|i| samples.push(&content[i * k..(i + 1) * k]));

        let mut dist = 0;
        samples.iter().tuple_combinations().for_each(|(s1, s2)| {
            dist += calc_hamming_dist(s1, s2);
        });
        let normalized_dist = dist as f64 / (sample_size * k) as f64;
        keysizes.push((k, normalized_dist));
    }
    keysizes.sort_by(|a, b| a.1.partial_cmp(&b.1).unwrap());
    keysizes.truncate(result_count);
    let result: Vec<usize> = keysizes.into_iter().map(|val| val.0).collect();
    Ok(result)
}

pub fn into_transposed_blocks(bytes: &Vec<u8>, block_size: usize) -> Vec<Vec<u8>> {
    let mut result = Vec::new();
    (0..block_size).for_each(|_| result.push(Vec::new()));
    let block_chunks = bytes.chunks(block_size);
    block_chunks.for_each(|chunk| {
        chunk
            .iter()
            .enumerate()
            .for_each(|(i, byte)| result[i].push(byte.clone()))
    });
    return result;
}
pub fn merge_transposed_blocks(blocks: Vec<Vec<u8>>) -> Vec<u8> {
    let mut merge_result = Vec::new();
    for i in 0..blocks[0].len() {
        for j in 0..blocks.len() {
            if i < blocks[j].len() {
                merge_result.push(blocks[j][i]);
            }
        }
    }
    merge_result
}
pub fn crack_repeating_key_xor(file_path: &str) -> Result<String, Box<dyn Error>> {
    let content = read_b64_content_as_bytes(file_path).unwrap();
    let top_key_sizes = find_keysize(&content).unwrap();
    let mut cracked_blocks = Vec::new();
    let key_size = top_key_sizes[0];
    let blocks = into_transposed_blocks(&content, key_size);
    for block in blocks {
        let mut cracking_result =
            ch3::brute_force_table_with_english_letter_frequency_score(&block, 1);
        let (cracked_block, _, _) = cracking_result.pop().unwrap_or_default();
        cracked_blocks.push(cracked_block);
    }
    let cracked_bytes = merge_transposed_blocks(cracked_blocks);
    let cracked_text = cracked_bytes
        .into_iter()
        .map(|byte| byte as char)
        .collect::<String>();

    Ok(cracked_text)
}

#[cfg(test)]
mod tests {
    use super::*;
    const FILE: &str = "src/ch6/encrypted.txt";
    const CRACKED: &str = "I'm back and I'm ringin' the bell \nA rockin' on the mike while the fly girls yell \nIn ecstasy in the back of me \nWell that's my DJ Deshay cuttin' all them Z's \nHittin' hard and the girlies goin' crazy \nVanilla's on the mike, man I'm not lazy. \n\nI'm lettin' my drug kick in \nIt controls my mouth and I begin \nTo just let it flow, let my concepts go \nMy posse's to the side yellin', Go Vanilla Go! \n\nSmooth 'cause that's the way I will be \nAnd if you don't give a damn, then \nWhy you starin' at me \nSo get off 'cause I control the stage \nThere's no dissin' allowed \nI'm in my own phase \nThe girlies sa y they love me and that is ok \nAnd I can dance better than any kid n' play \n\nStage 2 -- Yea the one ya' wanna listen to \nIt's off my head so let the beat play through \nSo I can funk it up and make it sound good \n1-2-3 Yo -- Knock on some wood \nFor good luck, I like my rhymes atrocious \nSupercalafragilisticexpialidocious \nI'm an effect and that you can bet \nI can take a fly girl and make her wet. \n\nI'm like Samson -- Samson to Delilah \nThere's no denyin', You can try to hang \nBut you'll keep tryin' to get my style \nOver and over, practice makes perfect \nBut not if you're a loafer. \n\nYou'll get nowhere, no place, no time, no girls \nSoon -- Oh my God, homebody, you probably eat \nSpaghetti with a spoon! Come on and say it! \n\nVIP. Vanilla Ice yep, yep, I'm comin' hard like a rhino \nIntoxicating so you stagger like a wino \nSo punks stop trying and girl stop cryin' \nVanilla Ice is sellin' and you people are buyin' \n'Cause why the freaks are jockin' like Crazy Glue \nMovin' and groovin' trying to sing along \nAll through the ghetto groovin' this here song \nNow you're amazed by the VIP posse. \n\nSteppin' so hard like a German Nazi \nStartled by the bases hittin' ground \nThere's no trippin' on mine, I'm just gettin' down \nSparkamatic, I'm hangin' tight like a fanatic \nYou trapped me once and I thought that \nYou might have it \nSo step down and lend me your ear \n'89 in my time! You, '90 is my year. \n\nYou're weakenin' fast, YO! and I can tell it \nYour body's gettin' hot, so, so I can smell it \nSo don't be mad and don't be sad \n'Cause the lyrics belong to ICE, You can call me Dad \nYou're pitchin' a fit, so step back and endure \nLet the witch doctor, Ice, do the dance to cure \nSo come up close and don't be square \nYou wanna battle me -- Anytime, anywhere \n\nYou thought that I was weak, Boy, you're dead wrong \nSo come on, everybody and sing this song \n\nSay -- Play that funky music Say, go white boy, go white boy go \nplay that funky music Go white boy, go white boy, go \nLay down and boogie and play that funky music till you die. \n\nPlay that funky music Come on, Come on, let me hear \nPlay that funky music white boy you say it, say it \nPlay that funky music A little louder now \nPlay that funky music, white boy Come on, Come on, Come on \nPlay that funky music \n";
    #[test]
    fn hamming_distance_succuss() {
        let str1 = "this is a test";
        let str2 = "wokka wokka!!!";
        let actual_hamming_dist = calc_hamming_dist(&str1.as_bytes(), &str2.as_bytes());
        let expected_hamming_dist = 37;
        assert_eq!(actual_hamming_dist, expected_hamming_dist);

        let str1 = "a b this is a test";
        let str2 = "b c this is a test";
        let actual_hamming_dist = calc_hamming_dist(&str1.as_bytes(), &str2.as_bytes());
        let expected_hamming_dist = 3;
        assert_eq!(actual_hamming_dist, expected_hamming_dist);
    }

    #[test]
    fn find_keysize_success() {
        let text = crack_repeating_key_xor(FILE).unwrap();
        assert_eq!(&text, CRACKED);
    }
}
