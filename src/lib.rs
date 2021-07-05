use std::error::Error;
use std::fs;

pub fn run(config: Config) -> Result<(), Box<dyn Error>> {
    let file = fs::read_to_string(&config.filename)?;

    println!("{} \n{}", config.filename, file);
    Ok(())
}

pub struct Config {
    pub pattern: String,
    pub filename: String,
}

impl Config {
    pub fn new(args: &[String]) -> Result<Config, &'static str> {
        if args.len() < 3 {
            return Err("Not enougth args!");
        }
        let pattern = args[1].clone();
        let filename = args[2].clone();
        Ok(Config { pattern, filename })
    }
}
