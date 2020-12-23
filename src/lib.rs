use std::env;
use std::error::Error;
use std::fs;

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn insufficient_arguments() {
        let args = [String::from("one"), String::from("two")];
        assert_eq!(Config::new(&args), Err("not enough arguments"));
    }

    #[test]
    fn enough_arguments() {
        let args = [
            String::from("one"),
            String::from("two"),
            String::from("three"),
        ];
        if let Err(_) = Config::new(&args) {
            panic!("the instance was not created");
        }
    }

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

#[derive(Debug, PartialEq)]
pub struct Config {
    pub query: String,
    pub filename: String,
    pub case_sensitive: bool,
}

impl Config {
    pub fn new(args: &[String]) -> Result<Config, &'static str> {
        if args.len() < 3 {
            return Err("not enough arguments");
        }

        let query = args[1].clone();
        let filename = args[2].clone();

        let mut case_sensitive = env::var("CASE_INSENSITIVE").is_err();

        if args.len() > 3 {
            let flag = &args[3];
            if flag == "--case-insensitive" {
                case_sensitive = false;
            } else if flag == "--case-sensitive" {
                case_sensitive = true;
            } else {
                return Err("invalid flag");
            }
        }

        Ok(Config {
            query,
            filename,
            case_sensitive,
        })
    }
}

pub fn run(config: Config) -> Result<(), Box<dyn Error>> {
    let contents = fs::read_to_string(config.filename)?;

    let results = if config.case_sensitive {
        search(&config.query, &contents)
    } else {
        search_case_insensitive(&config.query, &contents)
    };

    for line in results {
        println!("{}", line);
    }

    Ok(())
}

pub fn search<'a>(query: &str, contents: &'a str) -> Vec<&'a str> {
    let mut results = Vec::new();
    for line in contents.lines() {
        if line.contains(query) {
            results.push(line);
        }
    }

    results
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
