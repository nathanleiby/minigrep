use minigrep::Args;
use std::env;
use std::process;

fn main() {
    let args: Vec<String> = env::args().collect();

    let cfg = Args::build(&args).unwrap_or_else(|err| {
        println!("error parsing args: {err}");
        process::exit(1);
    });

    if let Err(e) = minigrep::run(cfg) {
        println!("application error: {e}");
        process::exit(1);
    }
}
