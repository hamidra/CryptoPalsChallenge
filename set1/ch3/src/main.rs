pub mod lib;
use lib::*;
fn main() {
    let hex_str = "1b37373331363f78151b7f2b783431333d78397828372d363c78373e783a393b3736";
    // print!("{:?}", most_frequent_as_key(hex_str));
    let decrypted_table = brute_force_table_with_english_letter_frequency_score(hex_str);
    for decrypted in decrypted_table.iter() {
        println!("{:?}", decrypted);
        println!("============================================================================");
    }
}
