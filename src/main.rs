fn repl() {
    println!("repl");
}

fn readfile() {
    println!("readfile");
}

fn main() {
    if true == false {
        readfile();
    } else {
        repl();
    }
}
