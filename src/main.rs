use std::path::PathBuf;

use clap::Parser;

#[derive(Parser, Debug)]
#[command(
    name = "roxin",
    version,
    about = "The Roxin interpreter and REPL",
    disable_help_flag = false,
    disable_version_flag = false
)]

struct Args {
    #[arg(value_name = "SOURCEFILE")]
    source_file: Option<PathBuf>,
}

fn main() {
    let args = Args::parse();

    if let Some(path) = args.source_file {
        println!("Interpreting: {}", path.display());
    } else {
        println!("Roxin REPL");
    }
}
