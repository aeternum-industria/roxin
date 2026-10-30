use std::fs;
use std::io::{self, Write};
use std::path::PathBuf;

use clap::Parser;

use roxin::interpreter::Model;
use roxin::parser::parse;
use roxin::run::{Job, Status};
use roxin::token::Span;

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

fn report(source: &str, span: Span, message: &str) {
    let (line, col) = line_col(source, span.start);
    eprintln!("error at line {line}, column {col}: {message}");
}

fn print_job(job: &Job) {
    let params: Vec<String> = job
        .overrides
        .iter()
        .map(|(n, v)| format!("{n} = {v}"))
        .collect();
    println!("job {} {}", job.index, params.join(", "));

    for (step, row) in job.rows.iter().enumerate() {
        println!("  {step:>3}  {row}");
    }

    if let Status::Stopped { at_step, error } = &job.status {
        println!("  stopped at step {at_step}: {error}");
    }
}

fn print_run_results(source: &str) {
    let module = match parse(source) {
        Ok(module) => module,
        Err(e) => {
            report(source, e.span, &e.message);
            return;
        }
    };

    let model = Model::from_module(module);

    for run in &model.runs {
        match model.run(run) {
            Ok(jobs) => jobs.iter().for_each(print_job),
            Err(e) => report(source, e.span, &e.message),
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
            print_run_results(&exp);
        }
    }
}

fn read_file(path: PathBuf) {
    match fs::read_to_string(path) {
        Ok(content) => {
            print_run_results(&content);
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
