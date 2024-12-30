mod encryption;
mod utils;

use std::process::exit;
use crate::utils::{read_env_args, read_file};

fn main() {

    if read_env_args().is_none() {
        eprintln!("File path was not provided by env args");
        exit(1)
    }

    let file_path = read_env_args().unwrap();
    let data_to_encrypt = read_file(&file_path);

    match data_to_encrypt {
        Ok(data) => {

        }
        Err(e) => {
            eprintln!("Error occurred while reading file {file_path}, error: {e}");
            exit(1)
        }
    }
}



