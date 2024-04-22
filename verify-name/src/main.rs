use std::env;
use std::fs;

fn main () {
    if env::args().len() < 2 {
        eprint!("two arguments required, filepath and name");
        std::process::exit(1);
    }

    let path = env::args().nth(1).unwrap();
    let name = env::args().nth(2).unwrap();

    let content = fs::read_to_string(path).unwrap();

    for line in content.lines() {
        if line == name {
            println!{"name is found "}
            return;
        } 
    }
    println!("not found")
    }