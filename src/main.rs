use std::io;

fn repl() {
    println!("roxin> ");

    let mut line = String::new();

    let result = io::stdin().read_line(&mut line);

    println!("{:?}", result);
}

fn main() {
    repl();
}
