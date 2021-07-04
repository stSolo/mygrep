use std::env;
use std::fs;

fn main() {
    let args: Vec<String> = env::args().collect();
    let (pattern, filename) = parse_confing(&args);

    let file = fs::read_to_string(filename).expect("File not found");

    println!("{} \n{}", filename, file);
}

fn parse_confing(args: &[String]) -> (&str, &str) {
    let pattern = &args[1];
    let filename = &args[2];
    (pattern, filename)
}
