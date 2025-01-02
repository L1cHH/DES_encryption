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
                    println!("Data to encrypt: {}", data);

                    let (encrypted_blocks, keys) = encrypt(data, secret_key);

                    println!("Encrypted data: {:?}", encrypted_blocks);

                    let decrypted_blocks = decrypt(encrypted_blocks, keys);

                    println!("Decrypted data: {}", vec_to_string(decrypted_blocks));
                }
                Err(e) => {
                    eprintln!("Error occurred while reading the file {file_path}, error: {e}");
                    exit(1)
                }
            }
        }
        None => {
            eprintln!("File path or secret key were not provided by args...\nUse next boilerplate: cargo run --bin name_of_bin_crate -- file_path secret_key");
            exit(1)
        }
    }
}



