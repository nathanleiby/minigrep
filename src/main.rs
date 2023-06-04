use std::env;
use std::fs;

fn main() {
    let args: Vec<String> = env::args().collect();

    let cfg = parse_args(&args).expect("expect args should parse");
    println!("Reading file: {}!", cfg.file_path);

    let contents = fs::read_to_string(&cfg.file_path).expect("Should have been able to read file");
    println!("With text:\n{}", contents);
}

#[derive(Debug)]
struct Args {
    query: String,
    file_path: String,
}

fn parse_args(args: &[String]) -> Result<Args, &'static str> {
    if args.len() < 3 {
        return Err("not enough args");
    }

    let query = args[1].clone();
    let file_path = args[2].clone();

    return Ok(Args { query, file_path });
}
