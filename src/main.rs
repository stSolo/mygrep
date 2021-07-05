use std::env;
use std::fs;

fn main() {
    let args: Vec<String> = env::args().collect();
    let config = parse_confing(&args);

    let file = fs::read_to_string(&config.filename).expect("File not found");

    println!("{} \n{}", config.filename, file);
}

struct Config {
    pattern: String,
    filename: String,
}

fn parse_confing(args: &[String]) -> Config {
    let pattern = args[1].clone();
    let filename = args[2].clone();
    Config { pattern, filename }
}
