mod erd;
mod file;
mod project;

use clap::Parser;
use erd::Table;
use file::read_file;
use std::path::PathBuf;
use syn::File;
use syn::{Item, ItemStruct};



fn main() {
	let args = Args::parse();
	match &args.command {
		Command::SeaormMarkdown(args) => run(args),
	};
}


/// The command line interface for setting up a Bottlerocket TestSys cluster and running tests.
#[derive(Debug, Parser)]
#[clap(author, version, about)]
struct Args {
    #[clap(subcommand)]
    command: Command,
}

#[derive(Debug, Parser)]
enum Command {
    SeaormMarkdown(SeaormMarkdownArgs),
}

#[derive(Debug,Parser)]
#[clap(author, version, about)]
struct SeaormMarkdownArgs {
	#[clap(long="project-root", short = 'r')]
	project_root: Option<String>,
}

fn run(args: &SeaormMarkdownArgs ) -> Result<(), Box<dyn std::error::Error>> {
    let path_list = project::get_rust_files_path_in_project(args.project_root.as_deref());

	println!("{:?}", path_list);
    //let _process = process(path_list);

    Ok(())
}

fn process(path_list: Vec<PathBuf>) -> Result<(), Box<dyn std::error::Error>> {
    let mut tables = Vec::new();

	for path in path_list {
        read_file(&path).map(|file| {
            extract_item_struct_from_file(file)
                .into_iter()
                .for_each(|structure| tables.push(Table::from_item_struct(structure)))
        })?;
    }

	Ok(())
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

#[cfg(test)]
mod tests {
    use super::*;
    use syn::parse_quote;

    #[test]
    fn test_extract_item_struct_from_file() {
        let file: File = parse_quote! {
            /// @type string
            /// minitems 1
			#[sea_orm(table_name = "cake")]
            struct Struct1;

            fn some_function() {}

            /**
             * @namespace abc
             */
             #[sea_orm(table_name = "cake")]
            struct Struct2 {
                field: i32,
            }
            enum SomeEnum {
                Variant1,
                Variant2,
            }
        };

        let result = extract_item_struct_from_file(file);

        assert_eq!(result.len(), 2);
        assert_eq!(result[0].ident, "Struct1");
        assert_eq!(result[1].ident, "Struct2");
    }
}
