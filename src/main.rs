use std::fs;
use std::io::{self, Write};
use std::path::PathBuf;

use clap::Parser;

use roxin::lexer::Lexer;

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

fn print_lexer_results(source: &str) {
    for token in Lexer::new(source) {
        println!("{:?} <{:?}>", token.text(source), token.kind);
    }
}

fn repl() {
    loop {
        let mut exp: String = String::new();
        let quit: String = String::from("quit");

        print!("roxin -> ");

        io::stdout().flush().unwrap();
        io::stdin()
            .read_line(&mut exp)
            .expect("Couldn't read input.");

        if exp.trim() == quit {
            break;
        } else {
            print_lexer_results(&exp);
        }
    }
}

fn read_file(path: PathBuf) {
    match fs::read_to_string(path) {
        Ok(content) => {
            print_lexer_results(&content);
        }
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
