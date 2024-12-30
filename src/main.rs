mod encryption;
mod utils;

use std::process::exit;
use crate::encryption::split_into_64bits_blocks;
use crate::utils::{read_env_args, read_file};

fn main() {
    match read_env_args() {
        Some(args) => {
            let (file_path, secret_key) = args;
            let data_to_encrypt = read_file(&file_path);

            match data_to_encrypt {
                Ok(data) => {
                    println!("{}", data);
                    let bytes_string = Vec::from(data);
                    println!("{:?}", bytes_string);
                    let mut blocks = split_into_64bits_blocks(bytes_string);
                    println!("{:?}", blocks);
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



