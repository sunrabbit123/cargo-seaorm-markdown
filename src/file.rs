use std::fs;

use syn::{parse_file, File};

pub fn read_file(file_path: &std::path::Path) -> Result<File, Box<dyn std::error::Error>> {
    let content = fs::read_to_string(file_path)?;

    Ok(parse_file(&content)?)
}
