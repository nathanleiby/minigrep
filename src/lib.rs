use std::env;
use std::error::Error;
use std::fs;

pub struct Args {
    pub query: String,
    pub file_path: String,
    pub case_insensitive: bool,
}

impl Args {
    pub fn build(mut args: impl Iterator<Item = String>) -> Result<Args, &'static str> {
        args.next();

        let query = match args.next() {
            Some(v) => v,
            None => return Err("didn't get a query string"),
        };

        let file_path = match args.next() {
            Some(v) => v,
            None => return Err("didn't get a file_path"),
        };

        let case_insensitive = env::var("IGNORE_CASE").is_ok();

        return Ok(Args {
            query,
            file_path,
            case_insensitive,
        });
    }
}

pub fn run(cfg: Args) -> Result<(), Box<dyn Error>> {
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
    contents.lines().filter(|l| l.contains(query)).collect()
}

pub fn search_case_insensitive<'a>(query: &str, contents: &'a str) -> Vec<&'a str> {
    let lower_query = query.to_lowercase();
    contents
        .lines()
        .filter(|l| l.to_lowercase().contains(&lower_query))
        .collect()
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
