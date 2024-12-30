mod encryption;
mod utils;

use std::process::exit;
use crate::encryption::{do_permutations, split_into_32bits_blocks, split_into_64bits_blocks, TABLE1};
use crate::utils::{read_env_args, read_file};

fn main() {
    match read_env_args() {
        Some(args) => {
            let (file_path, _) = args;
            let data_to_encrypt = read_file(&file_path);

            match data_to_encrypt {
                Ok(data) => {
                    ///Convert bytes string into 64bits blocks, then we are doing permutations with TABLE1
                    let mut blocks64bits = split_into_64bits_blocks(Vec::from(data));
                    for mut block in blocks64bits.iter_mut() {
                        do_permutations(&TABLE1, &mut block);
                    }

                    ///Split 64bits blocks into two 32bits blocks(L and R), then we will mutate them
                    let blocks32bits: Vec<([u8; 4], [u8; 4])> = split_into_32bits_blocks(blocks64bits);

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



