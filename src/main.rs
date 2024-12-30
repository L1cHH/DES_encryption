mod encryption;
mod utils;

use std::process::exit;
use crate::encryption::{do_permutations, split_into_64bits_blocks, TABLE1};
use crate::utils::{read_env_args, read_file};

fn main() {
    match read_env_args() {
        Some(args) => {
            let (file_path, _) = args;
            let data_to_encrypt = read_file(&file_path);

            match data_to_encrypt {
                Ok(data) => {
                    let blocks = split_into_64bits_blocks(Vec::from(data));

                    let mut blocks_after_perm: Vec<[u8; 8]> = Vec::new();
                    for block in blocks.iter() {
                        let mixed_block = do_permutations(&TABLE1, &block);
                        blocks_after_perm.push(mixed_block);

                    }


                }
                Err(e) => {
                    eprintln!("Error occurred while reading file {file_path}, error: {e}");
                    exit(1)
                }
            }
        }
        None => {
            eprintln!("File path was not provided by env args");
            exit(1)
        }
    }
}



