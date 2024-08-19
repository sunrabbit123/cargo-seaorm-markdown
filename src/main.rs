use syn::spanned::Spanned;
use syn::Error;
use std::fs;
use std::path::Path;
use syn::{ parse_file, Attribute, Item};
use walkdir::WalkDir;

fn main() {
    find_sea_orm_entities_in_project();
}
fn find_sea_orm_entities_in_project() {
    let mut total_sea_orm_entities = 0;
    let mut file_count = 0;

    for entry in WalkDir::new(".").into_iter().filter_map(|e| e.ok()) {
        let path = entry.path();
        if !path.is_file() || path.extension().map_or(true, |ext| ext != "rs") {
            continue;
        }

        match find_sea_orm_entities_in_file(path) {
            Ok(entities) => {
                file_count += 1;
                if entities.is_empty() {
                    continue;
                }

                println!("파일: {:?}", path);
                for entity_name in entities {
                    println!("  - {}", entity_name);
                    total_sea_orm_entities += 1;
                }
                println!();
            }
            Err(e) => eprintln!("파일 처리 중 오류 발생 {:?}: {}", path, e),
        }
    }

    println!("프로젝트 분석 결과:");
    println!("분석된 Rust 파일 수: {}", file_count);
    println!("Sea-ORM Entity 총 개수: {}", total_sea_orm_entities);
}

fn find_sea_orm_entities_in_file(file_path: &Path) -> Result<Vec<String>, Box<dyn std::error::Error>> {
    let content = fs::read_to_string(file_path)?;
    let syntax = parse_file(&content)?;

    let sea_orm_entities = syntax.items.iter()
        .filter_map(|item| {
            if let Item::Struct(item_struct) = item {
                if item_struct.attrs.iter().any(has_entity_derive) {
                    return Some(item_struct.ident.to_string())
                } 
            }
            
            None
        })
        .collect();

    Ok(sea_orm_entities)
}

fn has_entity_derive(attr: &Attribute) -> bool {
    if !attr.path().is_ident("derive") {
        return false;
    }

    let meta = attr.meta.clone();

    let list = match meta {
        syn::Meta::List(list) => list,
        _ => return false,
    };

    list.parse_nested_meta(|nested_meta|{ 
        if nested_meta.path.is_ident("DeriveEntityModel") {
            Ok(())
        }  else {
            Err(Error::new(attr.meta.span(), "Expected Debug"))
        }}).is_ok()
}
