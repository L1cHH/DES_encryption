use std::{env, fs, io};


///Func that reads content of file
pub fn read_file(file_path: &String) -> Result<String, io::Error> {
    let file_content = fs::read_to_string(file_path)?;
    Ok(file_content)
}

pub fn read_env_args() -> Option<String> {
    let mut env_args = env::args();

    println!("count: {}", env_args.len());

    if env_args.len() != 2 {
        return None
    }

    let file_path = env_args.nth(1).unwrap();

    Some(file_path)
}