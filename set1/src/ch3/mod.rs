use lazy_static::lazy_static;
use std::collections::HashMap;
use std::str;

lazy_static! {
    static ref OCCURANCE_ENGLISH: HashMap<char, f32> = [
        ('a', 8.2389258),
        ('b', 1.5051398),
        ('c', 2.8065007),
        ('d', 4.2904556),
        ('e', 12.813865),
        ('f', 2.2476217),
        ('g', 2.0327458),
        ('h', 6.1476691),
        ('i', 6.1476691),
        ('j', 0.1543474),
        ('k', 0.7787989),
        ('l', 4.0604477),
        ('m', 2.4271893),
        ('n', 6.8084376),
        ('o', 7.5731132),
        ('p', 1.9459884),
        ('q', 0.0958366),
        ('r', 6.0397268),
        ('s', 6.3827211),
        ('t', 9.1357551),
        ('u', 2.7822893),
        ('v', 0.9866131),
        ('w', 2.3807842),
        ('x', 0.1513210),
        ('y', 1.9913847),
        ('z', 0.0746517),
        (' ', 13.0),
    ]
    .iter()
    .cloned()
    .collect();
}

pub fn brute_force_table_with_number_of_valid_char_score(
    bytes: &Vec<u8>,
    return_count: usize,
) -> Vec<(Vec<u8>, f32, u8)> {
    // iterate through all valid ascii values and use them as key to decrypt
    let mut decrypted_table: Vec<(Vec<u8>, f32, u8)> = Vec::new();
    for key in 0..=127 {
        let mut decrypted = Vec::new();
        let mut score = 0i32;
        for byte in bytes.iter() {
            let dec = byte ^ key;
            decrypted.push(dec);
            // calculate the score as the number of valid alphabet letters [a-z]/[A-Z] in the decryoted message
            if (dec <= 90 && dec >= 65) || (dec <= 122 && dec >= 97) {
                score += 1
            }
        }
        decrypted_table.push((decrypted.into_iter().collect(), score as f32, key));
    }
    // sort the encrypted messages based on score
    decrypted_table.sort_by(|a, b| b.1.partial_cmp(&a.1).unwrap());

    // return top decryptions with the highest score as candidates
    decrypted_table.truncate(return_count);
    decrypted_table
}

pub fn brute_force_table_with_english_letter_frequency_score(
    bytes: &Vec<u8>,
    return_count: usize,
) -> Vec<(Vec<u8>, f32, u8)> {
    // convert hex to a u8 buffer
    let mut decrypted_table: Vec<(Vec<u8>, f32, u8)> = Vec::new();

    // iterate through all valid ascii values and use them as key to decrypt
    for key in 0..=127 {
        let mut decrypted = Vec::new();
        let mut score = 0f32;
        for byte in bytes.iter() {
            let dec = byte ^ key;
            decrypted.push(dec);
            // consider the score for each character as it's occurence frequency in english text
            // ref: https://en.wikipedia.org/wiki/Letter_frequency
            score += OCCURANCE_ENGLISH
                .get(&(dec as char).to_ascii_lowercase())
                .unwrap_or(&0f32);
        }
        decrypted_table.push((decrypted.into_iter().collect(), score, key));
    }
    // sort the encrypted messages based on score
    decrypted_table.sort_by(|a, b| b.1.partial_cmp(&a.1).unwrap());
    decrypted_table.truncate(return_count);
    decrypted_table
}

#[cfg(test)]
mod tests {
    use super::*;
    const ENCRYPTED: &str = "1b37373331363f78151b7f2b783431333d78397828372d363c78373e783a393b3736";
    #[test]
    fn brute_force_number_of_valid_char_score() {
        let encrypted = ENCRYPTED;
        let bytes: Vec<u8> = hex::decode(encrypted).expect("Invalid hex string");
        let decrypted_table = brute_force_table_with_number_of_valid_char_score(&bytes, 5);

        /*
        println!("==================================top==================================");
        for decrypted in decrypted_table.iter() {
            println!("{:?}", decrypted);
        }
        println!("========================================================================");
        */

        let expected = "Cooking MC's like a pound of bacon";
        let actual = str::from_utf8(&decrypted_table[0].0).unwrap();
        assert_eq!(expected, actual);
    }

    #[test]
    fn brute_force_english_letter_frequency_score() {
        let encrypted = ENCRYPTED;
        let bytes: Vec<u8> = hex::decode(encrypted).expect("Invalid hex string");
        let decrypted_table = brute_force_table_with_english_letter_frequency_score(&bytes, 5);

        /*
        println!("==================================top==================================");
        for decrypted in decrypted_table.iter() {
            println!("{:?}", decrypted);
        }
        println!("========================================================================");
        */

        let expected = "Cooking MC's like a pound of bacon";
        let actual = str::from_utf8(&decrypted_table[0].0).unwrap();
        assert_eq!(expected, actual);
    }
}
