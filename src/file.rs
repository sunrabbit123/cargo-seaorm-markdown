use std::{fs, path::PathBuf};

use syn::{parse_file, File, Item, ItemStruct};

use crate::erd::Table;

pub fn read_file(file_path: &std::path::Path) -> Result<File, Box<dyn std::error::Error>> {
    let content = fs::read_to_string(file_path)?;

    Ok(parse_file(&content)?)
}

pub fn extract_schemes_from_file(path_list: Vec<PathBuf>) -> Vec<Table> {
    let mut tables = Vec::new();

	for path in path_list {
        _ = read_file(&path).map(|file| {
            extract_item_struct_from_file(file)
                .into_iter()
                .for_each(|structure| tables.push(Table::from_item_struct(structure)))
        });
    }

	tables
}


fn extract_item_struct_from_file(file: File) -> Vec<ItemStruct> {
    file.items
        .into_iter()
        .filter_map(|item| match item {
            Item::Struct(item) => {
                println!("{:?}", item.attrs);
                Some(item)
            }
            _ => None,
        })
        .collect()
}