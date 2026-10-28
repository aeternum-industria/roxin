use std::fs;
use std::io::{self, Write};
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

fn repl() {
    loop {
        let mut exp: String = String::new();
        let quit: String = String::from("quit");

        print!("> ");

        io::stdout().flush().unwrap();
        io::stdin()
            .read_line(&mut exp)
            .expect("Couldn't read input.");

        if exp.trim() == quit {
            break;
        } // TODO: Lex
    }
}

fn read_file(path: PathBuf) {
    match fs::read_to_string(path) {
        Ok(content) => println!("{}", content), // TODO: Lex
        Err(error) => eprintln!("Error reading source file: {}", error),
    }
}

fn main() {
    let args = Args::parse();

    if let Some(path) = args.source_file {
        read_file(path);
    } else {
        repl();
    }
}
