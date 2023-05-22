use std::env;
use std::{error::Error, fs};

pub struct Config {
    pub query: String,
    pub file_path: String,
    pub options: Vec<Option>,
}

#[derive(PartialEq)]
pub enum Option {
    IgnoreCase,
}
impl Config {
    pub fn build(args: impl Iterator<Item = String>) -> Result<Config, &'static str> {
        let args = args.collect::<Vec<String>>();
        if args.len() < 3 {
            return Err("Not enough args");
        }

        let query = args[args.len() - 2].clone();
        let file_path = args[args.len() - 1].clone();

        // parse options
        let mut options: Vec<Option> = vec![];
        if args.len() > 3 {
            // `-x` options located at args[1..len-2]
            // TODO oargs in a single arg like `-ixs`
            let oargs = &args[1..args.len() - 2];
            for oarg in oargs {
                match oarg {
                    o if o == "-i" => options.push(Option::IgnoreCase),
                    _ => {
                        return Err("Invalid option argument.");
                    }
                }
            }
        }

        // if options not set, default to envvar
        if !options.contains(&Option::IgnoreCase) == true && env::var("IGNORE_CASE").is_ok() {
            options.push(Option::IgnoreCase);
        };

        Ok(Config {
            query,
            file_path,
            options,
        })
    }
}

pub fn print_usage() {
    println!("Usage: cargo run -- [-i, ...] <query> <file>");
}

pub fn run(config: Config) -> Result<String, Box<dyn Error>> {
    let contents = fs::read_to_string(config.file_path)?;

    let results = if config.options.contains(&Option::IgnoreCase) {
        search_case_insensitive(&config.query, &contents)
    } else {
        search(&config.query, &contents)
    };

    for line in results {
        println!("{line}");
    }

    Ok(contents)
}

pub fn search<'a>(query: &str, contents: &'a str) -> Vec<&'a str> {
    contents
        .lines()
        .filter(|line| line.contains(query))
        .collect()
}

pub fn search_case_insensitive<'a>(query: &str, contents: &'a str) -> Vec<&'a str> {
    let query = query.to_lowercase();
    let mut results = Vec::new();
    for line in contents.lines() {
        if line.to_lowercase().contains(&query) {
            results.push(line);
        }
    }

    results
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn case_sensitive() {
        let query = "duct";
        let contents = "\
Rust:
safe, fast, productive.
Pick three.
Duct tape.";

        assert_eq!(vec!["safe, fast, productive."], search(query, contents));
    }
    #[test]
    fn case_insensitive() {
        let query = "rUsT";
        let contents = "\
Rust:
safe, fast, productive.
Pick three.
Trust me.";

        assert_eq!(
            vec!["Rust:", "Trust me."],
            search_case_insensitive(query, contents)
        );
    }
}
