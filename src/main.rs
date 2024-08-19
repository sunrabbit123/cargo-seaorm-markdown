use std::fs;
use syn::Error;
use syn::{parse_file, Attribute, Item};
use syn::{spanned::Spanned, Path};
use walkdir::WalkDir;

fn main() {
    find_sea_orm_entities_in_project();
}

fn find_sea_orm_entities_in_project() {
    let mut total_structs = 0;
    let mut file_count = 0;
    let mut output = String::new();

    for entry in WalkDir::new(".").into_iter().filter_map(|e| e.ok()) {
        let path = entry.path();
        if !path.is_file() || path.extension().map_or(true, |ext| ext != "rs") {
            continue;
        }

        match find_structs_with_attribute(path, |p| p.is_ident("DeriveEntityModel")) {
            Ok(structs) => {
                file_count += 1;
                if structs.is_empty() {
                    continue;
                }

                output.push_str(&format!("파일: {:?}\n", path));
                for struct_name in structs {
                    output.push_str(&format!("  - {}\n", struct_name));
                    total_structs += 1;
                }
                output.push('\n');
            }
            Err(e) => eprintln!("파일 처리 중 오류 발생 {:?}: {}", path, e),
        }
    }

    output.push_str("프로젝트 분석 결과:\n");
    output.push_str(&format!("분석된 Rust 파일 수: {}\n", file_count));
    output.push_str(&format!(
        "특정 속성을 가진 구조체 총 개수: {}\n",
        total_structs
    ));

    println!("{}", output);
}

fn find_structs_with_attribute<F>(
    file_path: &std::path::Path,
    predicate: F,
) -> Result<Vec<String>, Box<dyn std::error::Error>>
where
    F: Fn(&Path) -> bool,
{
    let content = fs::read_to_string(file_path)?;
    let syntax = parse_file(&content)?;

    let structs = syntax
        .items
        .iter()
        .filter_map(|item| {
            if let Item::Struct(item_struct) = item {
                if item_struct
                    .attrs
                    .iter()
                    .any(|attr| has_specific_attribute(attr, &predicate))
                {
                    return Some(item_struct.ident.to_string());
                }
            }

            None
        })
        .collect();

    Ok(structs)
}

fn has_specific_attribute<F>(attr: &Attribute, predicate: F) -> bool
where
    F: Fn(&Path) -> bool,
{
    let meta = attr.meta.clone();

    let list = match meta {
        syn::Meta::List(list) => list,
        _ => return false,
    };

    list.parse_nested_meta(|nested_meta| {
        if predicate(&nested_meta.path) {
            Ok(())
        } else {
            Err(Error::new(attr.meta.span(), "Expected specific attribute"))
        }
    })
    .is_ok()
}
