use mygrep::Config;
use std::env;
use std::process;

fn main() {
    let config = Config::new(env::args()).unwrap_or_else(|err| {
        eprintln!("A problem with: {}", err);
        process::exit(1);
    });
    if let Err(e) = mygrep::run(config) {
        eprintln!("{}", e);
        process::exit(1);
    }
}
