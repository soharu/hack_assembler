use std::env;
use std::process;
use parser;

fn main() {
    let args: Vec<String> = env::args().collect();
    let config = parser::Config::new(&args).unwrap_or_else(|err| {
        println!("Problem parsing arguments: {}", err);
        process::exit(1);
    });

    if let Err(e) = parser::run(config) {
        println!("Application error: {}", e);
        process::exit(1);
    }
}
