use std::env;
use std::fs;

fn main() {
    let args: Vec<String> = env::args().collect();
    let pattern = &args[1];
    let filename = &args[2];

    let file = fs::read_to_string(filename).expect("File not found");

    println!("{} \n{}", filename, file);
}
