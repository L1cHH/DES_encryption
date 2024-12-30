use std::io::Read;

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