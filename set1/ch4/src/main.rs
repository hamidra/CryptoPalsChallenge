use set1_ch3::*;
use std::env::current_dir;
use std::fs::File;
use std::io::{self, BufRead};
use std::path::Path;

fn read_lines<P>(filename: P) -> io::Result<io::Lines<io::BufReader<File>>>
where
    P: AsRef<Path>,
{
    let file = File::open(filename)?;
    Ok(io::BufReader::new(file).lines())
}

fn read_cipher(file_path: &str) {
    println!("In file {}", file_path);

    /*let contents = fs::read_to_string(file_path).expect("Something went wrong reading the file");
    println!("With text:\n{}", contents);*/

    // read line by line
    let mut result: (Vec<char>, i32) = (Vec::new(), 0i32);
    if let Ok(lines) = read_lines(file_path) {
        for line in lines {
            if let Ok(cipher_text) = line {
                let decrypted = brute_force_table_with_english_letter_frequency_score(&cipher_text);
                for dec in decrypted.into_iter() {
                    if dec.1 > result.1 {
                        result = (dec.0, dec.1)
                    }
                }
            }
        }
    }
    println!("{:?}", result);
}
fn main() {
    let pwd = current_dir().expect("could not read the current working directory path");
    println!("current working dir: {}", pwd.display());
    read_cipher("/Users/hra/Code/CryptoPalsChallenge/set1/ch4/cipher.txt");
}
