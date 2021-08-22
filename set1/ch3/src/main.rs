use hex::decode;
use std::collections::HashMap;

fn most_frequent_as_key(hex_str: &str) -> Vec<char> {
    // convert hex to a u8 buffer
    let bytes: Vec<u8> = decode(hex_str).expect("Invalid hex string");

    // count the frequency of each byte value
    let mut ch_freq: HashMap<u8, u8> = HashMap::new();
    for byte in bytes.iter() {
        let freq = ch_freq.entry(*byte).or_insert(0);
        *freq += 1;
    }

    // sort the frequency vector
    let mut ch_freq_vec: Vec<(u8, u8)> = ch_freq.into_iter().collect();
    ch_freq_vec.sort_by(|a, b| b.1.cmp(&a.1));

    // pick the most frequent value as the key (is this a good guess. probably not, but worked for the example above!)
    let key = ch_freq_vec[0].0;

    // decrypt the message by xor-ing each value with the selected key.
    let mut decrypted: Vec<char> = Vec::new();
    for byte in bytes.iter() {
        decrypted.push((byte ^ key) as char);
    }
    decrypted
}

fn brute_force_table_with_score(hex_str: &str) -> Vec<(Vec<char>, u8, u8)> {
    // convert hex to a u8 buffer
    let bytes: Vec<u8> = decode(hex_str).expect("Invalid hex string");

    // iterate through all valid ascii values and use them as key to decrypt
    let mut decrypted_table: Vec<(Vec<char>, u8, u8)> = Vec::new();
    for key in 0..=127 {
        let mut decrypted: Vec<char> = Vec::new();
        let mut score = 0;
        for byte in bytes.iter() {
            let dec = byte ^ key;
            decrypted.push(dec as char);
            // calculate the score as the number of valid alphabet letters [a-z]/[A-Z] in the decryoted message
            if (dec <= 90 && dec >= 65) || (dec <= 122 && dec >= 97) {
                score += 1
            }
        }
        decrypted_table.push((decrypted, score, key));
    }
    // sort the encrypted messages based on score
    decrypted_table.sort_by(|a, b| b.1.cmp(&a.1));
    // return the 5 decryptions with the highest score as candidates
    decrypted_table.truncate(5);
    decrypted_table
}
fn main() {
    let hex_str = "1b37373331363f78151b7f2b783431333d78397828372d363c78373e783a393b3736";
    // print!("{:?}", most_frequent_as_key(hex_str));
    let decrypted_table = brute_force_table_with_score(hex_str);
    for decrypted in decrypted_table.iter() {
        println!("{:?}", decrypted);
        println!("============================================================================");
    }
}
