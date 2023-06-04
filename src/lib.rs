use std::env;
use std::error::Error;
use std::fs;

pub struct Args {
    pub query: String,
    pub file_path: String,
    pub case_insensitive: bool,
}

impl Args {
    pub fn build(args: &[String]) -> Result<Args, &'static str> {
        if args.len() < 3 {
            return Err("not enough args");
        }

        let query = args[1].clone();
        let file_path = args[2].clone();
        let case_insensitive = env::var("IGNORE_CASE").is_ok();

        return Ok(Args {
            query,
            file_path,
            case_insensitive,
        });
    }
}

pub fn run(cfg: Args) -> Result<(), Box<dyn Error>> {
    // println!("Reading file: {}!", cfg.file_path);
    let contents = fs::read_to_string(&cfg.file_path)?;

    let results = if cfg.case_insensitive {
        search_case_insensitive(&cfg.query, &contents)
    } else {
        search(&cfg.query, &contents)
    };

    for line in results {
        println!("{line}");
    }
    Ok(())
}

pub fn search<'a>(query: &str, contents: &'a str) -> Vec<&'a str> {
    let mut result = vec![];
    for line in contents.lines() {
        if line.contains(query) {
            result.push(line);
        }
    }
    result
}

pub fn search_case_insensitive<'a>(query: &str, contents: &'a str) -> Vec<&'a str> {
    let mut result = vec![];
    let lower = query.to_lowercase();
    for line in contents.lines() {
        if line.to_lowercase().contains(&lower) {
            result.push(line);
        }
    }
    result
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn case_sensitive() {
        let query = "duct";
        let contents = "\
Rust;
safe, fast, productive.
pick 3.";

        assert_eq!(vec!["safe, fast, productive."], search(query, contents));
    }

    #[test]
    fn case_insensitive() {
        let query = "RuSt";
        let contents = "\
Rust;
safe, fast, productive.
pick 3.";

        assert_eq!(vec!["Rust;"], search_case_insensitive(query, contents));
    }
}
