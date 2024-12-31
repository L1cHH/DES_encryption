

///This table used for first permutations after splitting initial bytes into blocks
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

///This table used for Permuted Choice 1 (after spitting secret key into 64 bits blocks)
pub static PC1: [usize; 56] = [
    57, 49, 41, 33, 25, 17, 9,
    1, 58, 50, 42, 34, 26, 18,
    10,  2, 59, 51, 43, 35, 27,
    19, 11,  3, 60, 52, 44, 36,
    63, 55, 47, 39, 31, 23, 15,
    7, 62, 54, 46, 38, 30, 22,
    14,  6, 61, 53, 45, 37, 29,
    21, 13,  5, 28, 20, 12,  4
];

pub static SHIFT_TABLE: [usize; 16] = [
    1, 1, 2, 2,
    2, 2, 2, 2,
    1, 2, 2, 2,
    2, 2, 2, 1
];

static PC2_TABLE: [usize; 48] = [
    14, 17, 11, 24,  1,  5,  3, 28,
    15,  6, 21, 10, 23, 19, 12,  4,
    26,  8, 16,  7, 27, 20, 13,  2,
    41, 52, 31, 37, 47, 55, 30, 40,
    51, 45, 33, 48, 44, 49, 39, 56,
    34, 53, 46, 42, 50, 36, 29, 32,
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

pub fn split_into_32bits_blocks(blocks64bits: Vec<[u8; 8]>) -> Vec<([u8; 4], [u8; 4])> {
    let mut blocks32bits = Vec::new();

    for block in blocks64bits.into_iter() {
        let splitted_block= block.split_at(4);
        ///We can use unwrap because we are sure that we split 64bits array into two 32bits array
        let l: [u8; 4] = splitted_block.0.try_into().unwrap();
        let r: [u8; 4] = splitted_block.1.try_into().unwrap();
        blocks32bits.push((l, r))
    }
    blocks32bits
}

pub fn do_permutations(table: &[usize; 64], block: &mut [u8; 8]) {

    let mut new_block: [u8; 8] = [0u8; 8];
    for (new_position, old_position) in table.iter().enumerate() {
        let original_byte_position = old_position / 8;
        let original_bit_position = old_position % 8;

        let new_byte_position = new_position / 8;
        let new_bit_position = new_position % 8;

        let old_bit = block[original_byte_position] >> (7 - original_bit_position) & 1;

        new_block[new_byte_position] |= old_bit << (7 - new_bit_position)
    }
    block[0..].copy_from_slice(&new_block);
}

pub fn permuted_choice1(table: &[usize; 56], secret_key64bits_blocks: Vec<[u8; 8]>) -> Vec<[u8; 7]> {
    let mut secret_key56bits_blocks: Vec<[u8; 7]> = Vec::new();

    for block64bits in secret_key64bits_blocks {

        let mut new_56bits_block = [0u8; 7];

        for (new_position, old_position) in table.iter().enumerate() {

            let original_byte_position = old_position / 8;
            let original_bit_position = old_position % 8;

            let new_byte_position = new_position / 8;
            let new_bit_position = new_position % 8;

            let bit_value = block64bits[original_byte_position] >> (7 - original_bit_position) & 1;

            new_56bits_block[new_byte_position] |= bit_value << (7 - new_bit_position)

        }
        secret_key56bits_blocks.push(new_56bits_block);
    }
    secret_key56bits_blocks
}

pub fn key_as_28bits_values(secret_key56bits_blocks: Vec<[u8; 7]>) -> Vec<(u32, u32)> {

    let mut secret_keys28bits: Vec<(u32, u32)> = Vec::new();

    for secret_key56bits in secret_key56bits_blocks {

        let l : u32 = (secret_key56bits[0] as u32) << 20
            | (secret_key56bits[1] as u32) << 12
            | (secret_key56bits[2] as u32) << 4
            | (secret_key56bits[3] as u32) >> 4;

        let r: u32 = ((secret_key56bits[3] as u32) & 0b00001111) << 24
            | (secret_key56bits[4] as u32) << 16
            | (secret_key56bits[5] as u32) << 8
            | (secret_key56bits[6] as u32);

        secret_keys28bits.push((l, r));
    }

    secret_keys28bits
}

pub fn key_as_56bits_value(secret_keys28bits: (u32, u32)) -> [u8; 7] {
    let mut key56bits: [u8; 7] = [0u8; 7];

    let left = secret_keys28bits.0;
    let right = secret_keys28bits.1;

    key56bits[0] = ((left >> 20) & 255u32) as u8;
    key56bits[1] = ((left >> 12) & 255u32) as u8;
    key56bits[2] = ((left >> 4) & 255u32) as u8;
    key56bits[3] = ((left << 4) & 255u32) as u8 | ((right) >> 24) as u8 & 0b0000_1111;
    key56bits[4] = ((right >> 16) & 255u32) as u8;
    key56bits[5] = ((right >> 8) & 255u32) as u8;
    key56bits[6] = (right & 255u32) as u8;

    key56bits
}

pub fn get_16_keys(keys_28bits_values: Vec<(u32, u32)>) -> Vec<[u8; 6]> {

    let mut secret_keys: Vec<[u8; 6]> = Vec::new();

    for key in keys_28bits_values {

    }

    secret_keys
}


fn get_48bits_keys(secret_key_28bits: &mut (u32, u32), amount_keys_to_generate: i8) -> Vec<[u8; 6]> {

    let mut keys = Vec::new();

    for index_shift in 0..amount_keys_to_generate {
        let shift_value = SHIFT_TABLE[index_shift as usize];
        let first_28bits = secret_key_28bits.0;
        let second_28bits = secret_key_28bits.1;

        let new_first_28bits = first_28bits << shift_value | first_28bits >> (8 - shift_value);
        let new_second_28bits = second_28bits << shift_value | second_28bits >> (8 - shift_value);
        secret_key_28bits.0 = new_first_28bits;
        secret_key_28bits.1 = new_second_28bits;

        let key_56bits: [u8; 7] = key_as_56bits_value((new_first_28bits, new_second_28bits));
        let key_48bits: [u8; 6] = into_48bits_key(key_56bits);
        keys.push(key_48bits)
    }

    keys
}

fn into_48bits_key(key_56bits: [u8; 7]) -> [u8; 6] {
    let mut key48bits: [u8; 6] = [0u8; 6];

    for (new_position, old_position) in PC2_TABLE.iter().enumerate() {
        let original_byte_position = old_position / 8;
        let original_bit_position = old_position % 8;

        let new_byte_position = new_position / 8;
        let new_bit_position = new_position % 8;

        let bit_value = key_56bits[original_byte_position] >> (7 - original_bit_position) & 1;
        key48bits[new_byte_position] |= bit_value << (7 - new_bit_position);
    }

    key48bits
}