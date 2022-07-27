use std::vec::Vec;

pub fn add_padding(mut block: Vec<u8>, block_size: usize) -> Vec<u8> {
    let padding_size = block_size - block.len() % block_size;
    block.append(&mut vec![padding_size as u8; padding_size]);
    block
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn pkcs7_padding_success() {
        let block = b"YELLOW SUBMARINE";
        let block_size: usize = 20;
        let padded = add_padding(block.to_vec(), block_size);
        let actual = &padded[..];
        let expected = b"YELLOW SUBMARINE\x04\x04\x04\x04";
        assert_eq!(expected, actual);
    }
}
