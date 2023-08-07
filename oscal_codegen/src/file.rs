use std::fs::{create_dir_all, File, OpenOptions};
use std::io::{Read, Write};

use crate::Result;

pub fn gen_txt_file(path: &str, contents: &str) -> Result<()> {
    let mut file = File::create(path)?;
    file.write_all(contents.as_bytes())?;
    Ok(())
}

pub fn append_txt_file(path: &str, contents: &str) -> Result<()> {
    let mut file = OpenOptions::new().append(true).open(path)?;
    file.write_all(contents.as_bytes())?;
    Ok(())
}

pub fn create_folder(path: &str) -> Result<()> {
    create_dir_all(path)?;
    Ok(())
}

pub fn read_file_to_string(path: &str) -> Result<String> {
    let mut file = File::open(path)?;
    let mut json = String::new();
    let _bytes_read = file.read_to_string(&mut json)?;
    Ok(json)
}

pub fn remove_file(path: &str) -> Result<()> {
    Ok(std::fs::remove_file(path)?)
}
