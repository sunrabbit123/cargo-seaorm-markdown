use std::io::Read;

use walkdir::WalkDir;

pub fn get_rust_files_path_in_project(root: Option<&str>) -> Vec<std::path::PathBuf> {
    let mut path_list = Vec::new();

    for entry in WalkDir::new(root.unwrap_or(".")).into_iter().filter_map(|e| e.ok()) {
        let path = entry.path();
        if !path.is_file() || path.extension().map_or(true, |ext| ext != "rs") {
            continue;
        }

        path_list.push(path.to_path_buf());
    }

    path_list
}

pub fn get_name_from_cargo_toml(root: Option<&str>) -> Option<String> {
    let mut cargo_toml = std::fs::File::open(format!("{}/Cargo.toml", root.unwrap_or("."))).expect("Cannot open Cargo.toml file.");
    let mut contents = String::new();
    cargo_toml.read_to_string(&mut contents).expect("Cannot read Cargo.toml file.");
    
    contents
        .lines()
        .find(|line| line.trim().starts_with("name"))
        .and_then(|line| line.split('=').nth(1))
        .map(|name| name.trim().trim_matches('"').into())
}