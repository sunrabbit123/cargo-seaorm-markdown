use syn::spanned::Spanned;
use syn::Error;
use std::fs;
use std::path::Path;
use syn::{ parse_file, Attribute, Item};
use walkdir::WalkDir;

fn main() {
    find_debug_structs_in_project();
}

fn find_debug_structs_in_project() {
    let mut total_debug_structs = 0;
    let mut file_count = 0;

    for entry in WalkDir::new(".").into_iter().filter_map(|e| e.ok()) {
        let path = entry.path();
        if path.is_file() && path.extension().map_or(false, |ext| ext == "rs") {
            match find_debug_structs_in_file(path) {
                Ok(structs) => {
                    if !structs.is_empty() {
                        println!("File: {:?}", path);
                        for struct_name in structs {
                            println!("  - {}", struct_name);
                            total_debug_structs += 1;
                        }
                        println!();
                    }
                    file_count += 1;
                }
                Err(e) => eprintln!("Error processing file {:?}: {}", path, e),
            }
        }
    }

    println!("Project analysis result:");
    println!("Number of Rust files analyzed: {}", file_count);
    println!("Total number of structs with derived Debug trait: {}", total_debug_structs);
}

fn find_debug_structs_in_file(file_path: &Path) -> Result<Vec<String>, Box<dyn std::error::Error>> {
    let content = fs::read_to_string(file_path)?;
    let syntax = parse_file(&content)?;

    let debug_structs = syntax.items.iter()
        .filter_map(|item| {
            if let Item::Struct(item_struct) = item {
                if item_struct.attrs.iter().any(has_debug_derive) {
                    Some(item_struct.ident.to_string())
                } else {
                    None
                }
            } else {
                None
            }
        })
        .collect();

    Ok(debug_structs)
}

fn has_debug_derive(attr: &Attribute) -> bool {
    if !attr.path().is_ident("derive") {
        return false;
    }

    let meta = attr.meta.clone();

    let list = match meta {
        syn::Meta::List(list) => list,
        _ => return false,
    };

    list.parse_nested_meta(|nested_meta|{ 
        if nested_meta.path.is_ident("Debug") {
            Ok(())
        }  else {
            Err(Error::new(attr.meta.span(), "Expected Debug"))
        }}).is_ok()
}