use std::fs;
use std::io::{self, Write};
use std::path::PathBuf;

use clap::Parser;

use roxin::layout::Layout;
use roxin::parser::parse;
use roxin::token::Token;

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

fn print_layout_results(source: &str) {
    for token in Layout::new(source) {
        print_token(source, &token);
    }
}

fn print_token(source: &str, token: &Token) {
    let text = &source[token.span.start..token.span.end];
    println!("{:?} <{:?}>", text, token.kind);
}

fn print_parse_results(source: &str) {
    match parse(source) {
        Ok(module) => println!("{module:#?}"),
        Err(e) => {
            let (line, col) = line_col(source, e.span.start);
            eprintln!("error at line {line}, column {col}: {}", e.message);
        }
    }
}

fn line_col(source: &str, offset: usize) -> (usize, usize) {
    let before = &source[..offset.min(source.len())];
    let line = before.matches('\n').count() + 1;
    let col = before
        .rfind('\n')
        .map_or(before.len(), |i| before.len() - i - 1)
        + 1;
    (line, col)
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
            print_layout_results(&exp);
            print_parse_results(&exp);
        }
    }
}

fn read_file(path: PathBuf) {
    match fs::read_to_string(path) {
        Ok(content) => {
            print_layout_results(&content);
            print_parse_results(&content);
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
