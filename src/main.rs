mod encryption;
mod utils;

use std::process::exit;
use crate::encryption::{do_permutations, key_as_28bits_values, PC1, permuted_choice1, split_into_32bits_blocks, split_into_64bits_blocks, TABLE1};
use crate::utils::{read_env_args, read_file};

fn main() {

    let byte: u8 = 0b1011_0110;
    let result: u8 = 0b1011_0101;
    println!("В ручную: {:08b}", result);

    let after_left: u8 = 0b1011_0000;

    let after_right: u8 = 0b0000_0101;
    println!("Програмно: {:08b}", after_left | after_right);

    match read_env_args() {
        Some(args) => {
            let (file_path, secret_key) = args;
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

                    ///secret key -> 64bits blocks
                    let secret_key = split_into_64bits_blocks(Vec::from(secret_key));

                    ///secret key must be interpreted as 56bits blocks
                    let secret_key = permuted_choice1(&PC1, secret_key);

                    ///secret key as two 28bits value (L... and R...) needs to create 16 48bits_key
                    let mut secret_key28bits: Vec<(u32, u32)> = key_as_28bits_values(secret_key);

                    let secret_keys48bits: Vec<[u8; 6]> = Vec::new();

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



