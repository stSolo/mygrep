mod tests;

use std::env;
use std::error::Error;
use std::fs;

pub fn run(config: Config) -> Result<(), Box<dyn Error>> {
    let file = fs::read_to_string(&config.filename)?;
    let result = if config.case_sensitive {
        search(&config.pattern, &file)
    } else {
        search_case_insentitve(&config.pattern, &file)
    };
    for line in result {
        println!("{}", line);
    }
    Ok(())
}

pub fn search<'a>(query: &str, content: &'a str) -> Vec<&'a str> {
    content
        .lines()
        .filter(|line| line.contains(&query))
        .collect()
}

pub fn search_case_insentitve<'a>(query: &str, content: &'a str) -> Vec<&'a str> {
    let query = query.to_lowercase();
    content
        .lines()
        .filter(|line| line.to_lowercase().contains(&query))
        .collect()
}

pub struct Config {
    pub pattern: String,
    pub filename: String,
    case_sensitive: bool,
}

impl Config {
    pub fn new(mut args: env::Args) -> Result<Config, &'static str> {
        args.next();
        let pattern = match args.next() {
            Some(v) => v,
            None => return Err("Pattern wasn't found!"),
        };
        let filename = match args.next() {
            Some(v) => v,
            None => return Err("File not found!"),
        };
        let case_sensitive = env::var("CASE_INSENSITIVE").is_err();
        Ok(Config {
            pattern,
            filename,
            case_sensitive,
        })
    }
}
