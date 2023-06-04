use std::error::Error;
use std::fs;

pub struct Args {
    pub query: String,
    pub file_path: String,
}

impl Args {
    pub fn build(args: &[String]) -> Result<Args, &'static str> {
        if args.len() < 3 {
            return Err("not enough args");
        }

        let query = args[1].clone();
        let file_path = args[2].clone();

        return Ok(Args { query, file_path });
    }
}

pub fn run(cfg: Args) -> Result<(), Box<dyn Error>> {
    // println!("Reading file: {}!", cfg.file_path);
    let contents = fs::read_to_string(&cfg.file_path)?;

    for line in search(&cfg.query, &contents) {
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

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn one_result() {
        let query = "duct";
        let contents = "\
Rust;
safe, fast, productive.
pick 3.";

        assert_eq!(vec!["safe, fast, productive."], search(query, contents));
    }
}
