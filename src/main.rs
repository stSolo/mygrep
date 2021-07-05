use mygrep::Config;
use std::env;
use std::process;

fn main() {
    let args: Vec<String> = env::args().collect();
    let config = Config::new(&args).unwrap_or_else(|err| {
        println!("Problem with: {}", err);
        process::exit(1);
    });
    if let Err(e) = mygrep::run(config) {
        println!("{}", e);
        process::exit(1);
    }
}
