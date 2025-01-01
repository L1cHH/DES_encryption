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

///Used for shifting 28bits part of 56bits key
pub static SHIFT_TABLE: [usize; 16] = [
    1, 1, 2, 2,
    2, 2, 2, 2,
    1, 2, 2, 2,
    2, 2, 2, 1
];

///Used for creating one of sixteen 48bits secret keys (from 56bits to 48bits)
static PC2_TABLE: [usize; 48] = [
    13, 16, 10, 23,  0,  4,  2, 27,
    14,  5, 20,  9, 22, 18, 11,  3,
    25,  7, 15,  6, 26, 19, 12,  1,
    40, 51, 30, 36, 46, 54, 29, 39,
    50, 44, 32, 47, 43, 48, 38, 55,
    33, 52, 45, 41, 49, 35, 28, 31,
];

///R... from 32bits to 48bits
static E_BIT_SELECTION_TABLE: [usize; 48] = [
    31,  0,  1,  2,  3,  4,
    3,  4,  5,  6,  7,  8,
    7,  8,  9, 10, 11, 12,
    11, 12, 13, 14, 15, 16,
    15, 16, 17, 18, 19, 20,
    19, 20, 21, 22, 23, 24,
    23, 24, 25, 26, 27, 28,
    27, 28, 29, 30, 31,  0,
];

///TABLE8, used for change bytes from 6bits into 4bits num
static S_BOX: [[[u8; 16]; 4]; 8] = [
    [
        [14,  4, 13,  1,  2, 15, 11,  8,  3, 10,  6, 12,  5,  9,  0,  7],
        [ 0, 15,  7,  4, 14,  2, 13,  1, 10,  6, 12, 11,  9,  5,  3,  8],
        [ 4,  1, 14,  8, 13,  6,  2, 11, 15, 12,  9,  7,  3, 10,  5,  0],
        [15, 12,  8,  2,  4,  9,  1,  7,  5, 11,  3, 14, 10,  0,  6, 13],
    ],
    [
        [15,  1,  8, 14,  6, 11,  3,  4,  9,  7,  2, 13, 12,  0,  5, 10],
        [ 3, 13,  4,  7, 15,  2,  8, 14, 12,  0,  1, 10,  6,  9, 11,  5],
        [ 0, 14,  7, 11, 10,  4, 13,  1,  5,  8, 12,  6,  9,  3,  2, 15],
        [13,  8, 10,  1,  3, 15,  4,  2, 11,  6,  7, 12,  0,  5, 14,  9],
    ],
    [
        [10,  0,  9, 14,  6,  3, 15,  5,  1, 13, 12,  7, 11,  4,  2,  8],
        [13,  7,  0,  9,  3,  4,  6, 10,  2,  8,  5, 14, 12, 11, 15,  1],
        [13,  6,  4,  9,  8, 15,  3,  0, 11,  1,  2, 12,  5, 10, 14,  7],
        [ 1, 10, 13,  0,  6,  9,  8,  7,  4, 15, 14,  3, 11,  5,  2, 12],
    ],
    [
        [ 7, 13, 14,  3,  0,  6,  9, 10,  1,  2,  8,  5, 11, 12,  4, 15],
        [13,  8, 11,  5,  6, 15,  0,  3,  4,  7,  2, 12,  1, 10, 14,  9],
        [10,  6,  9,  0, 12, 11,  7, 13, 15,  1,  3, 14,  5,  2,  8,  4],
        [ 3, 15,  0,  6, 10,  1, 13,  8,  9,  4,  5, 11, 12,  7,  2, 14],
    ],
    [
        [ 2, 12,  4,  1,  7, 10, 11,  6,  8,  5,  3, 15, 13,  0, 14,  9],
        [14, 11,  2, 12,  4,  7, 13,  1,  5,  0, 15, 10,  3,  9,  8,  6],
        [ 4,  2,  1, 11, 10, 13,  7,  8, 15,  9, 12,  5,  6,  3,  0, 14],
        [11,  8, 12,  7,  1, 14,  2, 13,  6, 15,  0,  9, 10,  4,  5,  3],
    ],
    [
        [12,  1, 10, 15,  9,  2,  6,  8,  0, 13,  3,  4, 14,  7,  5, 11],
        [10, 15,  4,  2,  7, 12,  9,  5,  6,  1, 13, 14,  0, 11,  3,  8],
        [ 9, 14, 15,  5,  2,  8, 12,  3,  7,  0,  4, 10,  1, 13, 11,  6],
        [ 4,  3,  2, 12,  9,  5, 15, 10, 11, 14,  1,  7,  6,  0,  8, 13],
    ],
    [
        [ 4, 11,  2, 14, 15,  0,  8, 13,  3, 12,  9,  7,  5, 10,  6,  1],
        [13,  0, 11,  7,  4,  9,  1, 10, 14,  3,  5, 12,  2, 15,  8,  6],
        [ 1,  4, 11, 13, 12,  3,  7, 14, 10, 15,  6,  8,  0,  5,  9,  2],
        [ 6, 11, 13,  8,  1,  4, 10,  7,  9,  5,  0, 15, 14,  2,  3, 12],
    ],
    [
        [13,  2,  8,  4,  6, 15, 11,  1, 10,  9,  3, 14,  5,  0, 12,  7],
        [ 1, 15, 13,  8, 10,  3,  7,  4, 12,  5,  6, 11,  0, 14,  9,  2],
        [ 7, 11,  4,  1,  9, 12, 14,  2,  0,  6, 10, 13, 15,  3,  5,  8],
        [ 2,  1, 14,  7,  4, 10,  8, 13, 15, 12,  9,  0,  3,  5,  6, 11],
    ],
];

static TABLE5: [usize; 32] = [
    15, 6, 19, 20,
    28, 11, 27, 16,
    0, 14, 22, 25,
    4, 17, 30, 9,
    1, 7, 23, 13,
    31, 26, 2, 8,
    18, 12, 29, 5,
    21, 10, 3, 24,
];

static FINAL_PERMUTATION_TABLE: [usize; 64] = [
    39, 7, 47, 15, 55, 23, 63, 31,
    38, 6, 46, 14, 54, 22, 62, 30,
    37, 5, 45, 13, 53, 21, 61, 29,
    36, 4, 44, 12, 52, 20, 60, 28,
    35, 3, 43, 11, 51, 19, 59, 27,
    34, 2, 42, 10, 50, 18, 58, 26,
    33, 1, 41,  9, 49, 17, 57, 25,
    32, 0, 40,  8, 48, 16, 56, 24,
];

pub fn split_data_into_64bits_blocks(bytes_string: Vec<u8>) -> Vec<[u8; 8]> {
    let mut blocks: Vec<[u8; 8]> = Vec::new();

    for block in bytes_string.chunks(8) {
        let mut new_block: [u8; 8] = [0u8; 8];
        let block_len = block.len();
        new_block[8-block_len..].copy_from_slice(block);
        blocks.push(new_block);
    }
    blocks
}

pub fn key_into_64bits(bytes_string: Vec<u8>) -> [u8; 8] {

    if bytes_string.len() > 8 {
        eprintln!("Secret key cant be more than 64bits in size")
    }

    let mut key_64bits: [u8; 8] = [0u8; 8];

    for block in bytes_string.chunks(8) {
        let block_len = block.len();
        key_64bits[8-block_len..].copy_from_slice(block);
    }
    key_64bits
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

pub fn do_permutations(block: &mut [u8; 8]) {

    let mut new_block: [u8; 8] = [0u8; 8];
    for (new_position, old_position) in TABLE1.iter().enumerate() {
        let original_byte_position = old_position / 8;
        let original_bit_position = old_position % 8;

        let new_byte_position = new_position / 8;
        let new_bit_position = new_position % 8;

        let old_bit = block[original_byte_position] >> (7 - original_bit_position) & 1;

        new_block[new_byte_position] |= old_bit << (7 - new_bit_position)
    }
    block[0..].copy_from_slice(&new_block);
}

pub fn permuted_choice1(secret_key64bits: [u8; 8]) -> [u8; 7] {

    let mut key_56bits = [0u8; 7];

    for (new_position, old_position) in PC1.iter().enumerate() {

        let original_byte_position = old_position / 8;
        let original_bit_position = old_position % 8;

        let new_byte_position = new_position / 8;
        let new_bit_position = new_position % 8;

        let bit_value = secret_key64bits[original_byte_position] >> (7 - original_bit_position) & 1;

        key_56bits[new_byte_position] |= bit_value << (7 - new_bit_position)

    }
    key_56bits
}

pub fn key_as_28bits_values(secret_key56bits: [u8; 7]) -> (u32, u32) {

    let l : u32 = (secret_key56bits[0] as u32) << 20
        | (secret_key56bits[1] as u32) << 12
        | (secret_key56bits[2] as u32) << 4
        | (secret_key56bits[3] as u32) >> 4;

    let r: u32 = ((secret_key56bits[3] as u32) & 0b00001111) << 24
        | (secret_key56bits[4] as u32) << 16
        | (secret_key56bits[5] as u32) << 8
        | (secret_key56bits[6] as u32);

    (l, r)
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

pub fn get_16_keys(mut keys_28bits_values: (u32, u32)) -> Vec<[u8; 6]> {

    let mut keys = Vec::new();

    for index_shift in 0..16 {
        let shift_value = SHIFT_TABLE[index_shift as usize];
        let first_28bits = keys_28bits_values.0;
        let second_28bits = keys_28bits_values.1;

        let new_first_28bits = first_28bits << shift_value | first_28bits >> (8 - shift_value);
        let new_second_28bits = second_28bits << shift_value | second_28bits >> (8 - shift_value);
        keys_28bits_values.0 = new_first_28bits;
        keys_28bits_values.1 = new_second_28bits;

        let key_56bits: [u8; 7] = key_as_56bits_value((new_first_28bits, new_second_28bits));
        let key_48bits: [u8; 6] = into_48bits_key(key_56bits);
        keys.push(key_48bits)
    }

    keys
}

fn into_48bits_key(key_56bits: [u8; 7]) -> [u8; 6] {
    let mut key48bits: [u8; 6] = [0u8; 6];

    for (new_position, mut old_position) in PC2_TABLE.iter().enumerate() {

        let original_byte_position = old_position / 8;
        let original_bit_position = old_position % 8;

        let new_byte_position = new_position / 8;
        let new_bit_position = new_position % 8;

        let bit_value = key_56bits[original_byte_position] >> (7 - original_bit_position) & 1;
        key48bits[new_byte_position] |= bit_value << (7 - new_bit_position);
    }

    key48bits
}

fn r_to_48bits(prev_r: [u8; 4]) -> [u8; 6] {
    let mut new_r: [u8; 6] = [0u8; 6];

    for (new_position, old_position) in E_BIT_SELECTION_TABLE.iter().enumerate() {
        let original_byte_position = old_position / 8;
        let original_bit_position = old_position % 8;

        let new_byte_position = new_position / 8;
        let new_bit_position = new_position % 8;

        let bit_value = prev_r[original_byte_position] >> (7 - original_bit_position) & 1;
        new_r[new_byte_position] |= bit_value << (7 - new_bit_position);
    }

    new_r
}

pub fn r_xor_48bits_key(r: [u8; 6], secret_key: [u8; 6]) -> [u8; 6] {

    let mut xor_result = [0u8; 6];

    for step in 0..6 {
        xor_result[step] = r[step] ^ secret_key[step]
    }

    xor_result
}

///After XOR we need to group our 48bits value into 6bits groups
/// I use array of 8bits, but two high_order bits are always 00
pub fn into_groups_by_6bits(xor_result: [u8; 6]) -> [u8; 8] {

    let xor_result_as_48bits_num: u64 = (xor_result[0] as u64) << 40
        | (xor_result[1] as u64) << 32
        | (xor_result[2] as u64) << 24
        | (xor_result[3] as u64) << 16
        | (xor_result[4] as u64) << 8
        | (xor_result[5] as u64);

    split_into_groups(xor_result_as_48bits_num)
}

///Shift our 48bits value to right and multiply(& and) by (0b0011_1111 as u64) and convert to u8
///also reducing our shift value by 6 after every iteration over array
fn split_into_groups(xor_result: u64) -> [u8; 8] {
    let mut groups_by_6bits = [0u8; 8];
    let mut shift_value = 42;

    for step in 0..8 {
        groups_by_6bits[step] = ((xor_result >> shift_value) & 0b0011_1111u64) as u8;
        shift_value -= 6
    }

    groups_by_6bits
}

fn groups_by_6bits_into_32bits(groups: [u8; 8]) -> [u8; 4] {

    let mut final_value = [0u8; 4];
    let mut final_32bits_value = 0u32;
    let mut shift:usize = 24;
    let mut bytes = [0u8; 8];

    for (s_box_index, byte) in groups.into_iter().enumerate() {

        let needed_s = S_BOX[s_box_index];

        let number_of_string = (byte & 1 | (byte >> 4 & 2u8)) as usize;
        let number_of_column = ((byte & 0b0001_1110) >> 1) as usize;

        let needed_string = needed_s[number_of_string];
        let needed_4bits = needed_string[number_of_column] << 4;

        bytes[s_box_index] = needed_4bits;
    }
    final_32bits_value = (bytes[0] as u32) << 24
        | (bytes[1] as u32) << 20
        | (bytes[2] as u32) << 16
        | (bytes[3] as u32) << 12
        | (bytes[4] as u32) << 8
        | (bytes[5] as u32) << 4
        | (bytes[6] as u32) << 0
        | (bytes[7] >> 4) as u32;


    final_value[0] = ((final_32bits_value >> 24) & 0b1111_1111u32) as u8;
    final_value[1] = ((final_32bits_value >> 16) & 0b1111_1111u32) as u8;
    final_value[2] = ((final_32bits_value >> 8) & 0b1111_1111u32) as u8;
    final_value[3] = (final_32bits_value & 0b1111_1111u32) as u8;

    final_value
}

pub fn permute_32bits_r(key: [u8; 4]) -> [u8; 4] {
    let mut new_key = [0u8; 4];

    for (new_position, old_position) in TABLE5.iter().enumerate() {
        let original_byte_position = old_position / 8;
        let original_bit_position = old_position % 8;

        let new_byte_position = new_position / 8;
        let new_bit_position = new_position % 8;

        let bit_value = key[original_byte_position] >> (7 - original_bit_position) & 1;

        new_key[new_byte_position] |= bit_value << (7 - new_bit_position);
    }

    new_key
}

fn xor_left_and_right(left: [u8; 4], result_r: [u8; 4]) -> [u8; 4] {
    let mut final_r = [0u8; 4];

    for step in 0..3 {
        final_r[step] = left[step] ^ result_r[step]
    }

    final_r
}

///Takes old L and R, returns L + 1, R + 1, until L15 and R15
pub fn do_round(mut old_l: &mut [u8; 4], mut old_r: &mut [u8; 4], secret_key: [u8; 6]) {

    ///Expand right part to 48 bits
    let r48bits = r_to_48bits(*old_r);

    ///After xor secret key and r48bits
    let r48bits_after_xor = r_xor_48bits_key(r48bits, secret_key);

    ///Split into 8 groups by 6bits
    let groups_by_6bits = into_groups_by_6bits(r48bits_after_xor);

    ///Concat into 32 bits by S_BOX table
    let r32bits = groups_by_6bits_into_32bits(groups_by_6bits);

    ///Permute by Table5
    let r32bits_permuted = permute_32bits_r(r32bits);

    ///Xor left part and mutated right part
    let final_r = xor_left_and_right(*old_l, r32bits_permuted);

    *old_l = *old_r;
    *old_r = final_r;
}

pub fn encrypt(data_to_encrypt: Vec<([u8; 4], [u8; 4])>, secret_keys48bits: &Vec<[u8; 6]>) -> Vec<[u8; 8]> {
    let mut encrypted_data_blocks: Vec<[u8; 8]> = Vec::new();

    for (block_index, mut data_block) in data_to_encrypt.into_iter().enumerate() {

        for step in 0..16 {
            let secret_key = &secret_keys48bits[0];

            do_round(&mut data_block.0, &mut data_block.1, *secret_key);
        }

        ///Swap L and R, then concatenate to 64 bits

        let mut empty_block = [0u8; 8];
        empty_block[0] = data_block.1[0];
        empty_block[1] = data_block.1[1];
        empty_block[2] = data_block.1[2];
        empty_block[3] = data_block.1[3];

        empty_block[4] = data_block.0[0];
        empty_block[5] = data_block.0[1];
        empty_block[6] = data_block.0[2];
        empty_block[7] = data_block.0[3];

        encrypted_data_blocks.push(last_permutation(empty_block));
    }

    encrypted_data_blocks
}

fn last_permutation(block64bits: [u8; 8]) -> [u8; 8] {
    let mut new_block: [u8; 8] = [0u8; 8];

    for (new_position, old_position) in FINAL_PERMUTATION_TABLE.iter().enumerate() {
        let original_byte_position = old_position / 8;
        let original_bit_position = old_position % 8;

        let new_byte_position = new_position / 8;
        let new_bit_position = new_position % 8;

        let bit_value = block64bits[original_byte_position] >> (7 - original_bit_position) & 1;

        new_block[new_byte_position] |= bit_value << (7 - new_bit_position)
    }

    new_block
}