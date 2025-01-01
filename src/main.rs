mod encryption;
mod utils;

use std::process::exit;
use crate::encryption::{decrypt, encrypt};
use crate::utils::{read_env_args, read_file, vec_to_string};

fn main() {
    match read_env_args() {
        Some(args) => {
            let (file_path, secret_key) = args;
            let data_to_encrypt = read_file(&file_path);

            match data_to_encrypt {
                Ok(data) => {

                    println!("Raw data: {}", data);

                    let (encrypted_blocks, init_l_and_r, keys) = encrypt(data, secret_key);

                    println!("Encrypted data: {:?}", encrypted_blocks);

                    let decrypted_blocks = decrypt(init_l_and_r, keys);

                    println!("Decrypted data: {:?}", decrypted_blocks);

                    println!("{}", vec_to_string(decrypted_blocks));
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



