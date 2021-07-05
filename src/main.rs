use std::env;
use std::error::Error;
use std::fs;
use std::process;

fn main() {
    let args: Vec<String> = env::args().collect();
    let config = Config::new(&args).unwrap_or_else(|err| {
        println!("Problem with: {}", err);
        process::exit(1);
    });
    if let Err(e) = run(config) {
        println!("{}", e);
        process::exit(1);
    }
}

fn run(config: Config) -> Result<(), Box<dyn Error>> {
    let file = fs::read_to_string(&config.filename)?;

    println!("{} \n{}", config.filename, file);
    Ok(())
}

struct Config {
    pattern: String,
    filename: String,
}

impl Config {
    fn new(args: &[String]) -> Result<Config, &'static str> {
        if args.len() < 3 {
            return Err("Not enougth args!");
        }
        let pattern = args[1].clone();
        let filename = args[2].clone();
        Ok(Config { pattern, filename })
    }
}
