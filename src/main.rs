mod erd;
mod file;
mod project;

use std::io::Write;

use clap::Parser;

use crate::file::extract_schemes_from_file;



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
    let path_list = project::get_rust_files_path_in_project(args.project_root.as_deref());
    let table_list = extract_schemes_from_file(path_list);
    let msg = format!("
```mermaid
erdiagram
    
{}
```
", table_list.iter().map(|v| v.to_mermaid_erd_text()).collect::<Vec<String>>().join("\n"));
    let mut file = std::fs::File::create(".test.md").expect("create failed");
    file.write(msg.as_bytes())?;
    println!("{}", msg);
    Ok(())
}
