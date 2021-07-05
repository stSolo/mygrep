use std::env;
use std::fs;

fn main() {
    let args: Vec<String> = env::args().collect();
    let config = Config::new(&args);

    let file = fs::read_to_string(&config.filename).expect("File not found");

    println!("{} \n{}", config.filename, file);
}

struct Config {
    pattern: String,
    filename: String,
}

impl Config {
    fn new(args: &[String]) -> Config {
        if args.len() < 3 {
            panic!("Not enougth args!")
        }
        let pattern = args[1].clone();
        let filename = args[2].clone();
        Config { pattern, filename }
    }
}
