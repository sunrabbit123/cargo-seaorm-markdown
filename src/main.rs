mod erd;
mod file;
mod project;

use std::io::Write;

use clap::Parser;

use crate::{file::extract_schemes_from_file, project::get_name_from_cargo_toml};



fn main() {
	let args = Args::parse();
	let _ = match &args.command {
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
    let title = get_name_from_cargo_toml(args.project_root.as_deref()).expect("not found name on Cargo.toml");
    let path_list = project::get_rust_files_path_in_project(args.project_root.as_deref());
    let table_list = extract_schemes_from_file(path_list).iter().map(|v| v.to_mermaid_erd_text()).collect::<Vec<String>>().join("");
    let msg = format!("# {title}

```mermaid
erdiagram

{table_list}
```
");

    let mut file = std::fs::File::create(format!("{}/.test.md", args.project_root.as_deref().unwrap_or("."))).expect("create failed");
    file.write(msg.as_bytes())?;
    println!("{}", msg);
    Ok(())
}
