

///This table used for first permutations after slitting initial bytes into blocks
pub static TABLE1: [usize; 64] = [
    57, 49, 41, 33, 25, 17, 9, 1,
    59, 51, 43, 35, 27, 19, 11, 3,
    61, 53, 45, 37, 29, 21, 13, 5,
    63, 55, 47, 39, 31, 23, 15, 7,
    56, 48, 40, 32, 24, 16, 8, 0,
    58, 50, 42, 34, 26, 18, 10, 2,
    60, 52, 44, 36, 28, 20, 12, 4,
    62, 54, 46, 38, 30, 22, 14, 6
];

pub fn split_into_64bits_blocks(bytes_string: Vec<u8>) -> Vec<[u8; 8]> {
    let mut blocks: Vec<[u8; 8]> = Vec::new();

    for block in bytes_string.chunks(8) {
        let mut new_block: [u8; 8] = [0u8; 8];
        let block_len = block.len();
        new_block[8-block_len..].copy_from_slice(block);
        blocks.push(new_block);
    }

    blocks
}

pub fn do_permutations(table: &[usize; 64], block: &[u8; 8]) -> [u8; 8] {

    let mut new_block: [u8; 8] = [0u8; 8];
    for (new_position, old_position) in table.iter().enumerate() {
        let original_byte_position = old_position / 8;
        let original_bit_position = old_position % 8;

        let new_byte_position = new_position / 8;
        let new_bit_position = new_position % 8;

        let old_bit = block[original_byte_position] >> (7 - original_bit_position) & 1;

        new_block[new_byte_position] |= old_bit << (7 - new_bit_position)
    }

    new_block
}

