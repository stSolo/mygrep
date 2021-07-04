use std::env;

fn main() {
    let args: Vec<String> = env::args().collect();
    let pattern = &args[1];
    let filename = &args[2];
    println!("{} {}", pattern, filename);
}
