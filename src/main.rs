mod cli;

use cli::*;

fn main() {
    let args: ParsedArgs = parse_args(&mut std::env::args());

    println!("{:?}", args);
}
