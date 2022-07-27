use crate::ch3::*;
use std::fs::File;
use std::io::{self, BufRead};
use std::path::Path;

pub fn read_lines<P>(filename: P) -> io::Result<io::Lines<io::BufReader<File>>>
where
    P: AsRef<Path>,
{
    let file = File::open(filename)?;
    Ok(io::BufReader::new(file).lines())
}

pub fn find_cipher(file_path: &Path) -> (Vec<u8>, f32, usize, String) {
    /*let contents = fs::read_to_string(file_path).expect("Something went wrong reading the file");
    println!("With text:\n{}", contents);*/

    // read line by line
    let mut result: (Vec<u8>, f32, usize, String) = (Vec::new(), 0f32, 0usize, String::from(""));
    if let Ok(lines) = read_lines(file_path) {
        for (idx, line) in lines.enumerate() {
            if let Ok(cipher_text) = line {
                let bytes = hex::decode(&cipher_text).unwrap();
                let decrypted = brute_force_table_with_english_letter_frequency_score(&bytes, 5);
                for dec in decrypted.into_iter() {
                    if dec.1 > result.1 {
                        result = (dec.0, dec.1, idx + 1, cipher_text.clone())
                    }
                }
            }
        }
    }
    result
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::path::Path;
    use std::str;
    #[test]
    fn find_cipher_test() {
        let cipher_path = Path::new("src/ch4/cipher.txt");
        let result = find_cipher(cipher_path);
        let expected = ("Now that the party is jumping\n", 171);
        let actual = (str::from_utf8(&result.0).unwrap(), result.2);
        assert_eq!(expected, actual);
    }
}
