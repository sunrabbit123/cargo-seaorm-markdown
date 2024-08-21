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
