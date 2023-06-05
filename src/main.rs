use minigrep::Args;
use std::env;
use std::process;

fn main() {
    let cfg = Args::build(env::args()).unwrap_or_else(|err| {
        eprintln!("error parsing args: {err}");
        process::exit(1);
    });

    if let Err(e) = minigrep::run(cfg) {
        eprintln!("application error: {e}");
        process::exit(1);
    }
}
