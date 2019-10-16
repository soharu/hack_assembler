use parser;
use std::env;
use std::error::Error;
use std::fs;
use std::process;

fn main() {
    let args: Vec<String> = env::args().collect();
    let config = Config::new(&args).unwrap_or_else(|err| {
        println!("Problem parsing arguments: {}", err);
        process::exit(1);
    });

    match file_to_lines(config.filename.as_str()) {
        Ok(lines) => {
            let lines_as_ref = lines.iter().map(AsRef::as_ref).collect();
            let result = parser::binary_code_from(lines_as_ref);
            for bin in result {
                println!("{}", bin);
            }
        }
        Err(e) => {
            println!("Application error: {}", e);
            process::exit(1);
        }
    }
}

struct Config {
    filename: String,
}

impl Config {
    fn new(args: &[String]) -> Result<Config, &'static str> {
        if args.len() < 2 {
            return Err("not enough arguments");
        }
        let filename = args[1].clone();
        Ok(Config { filename })
    }
}

fn file_to_lines(filename: &str) -> Result<Vec<String>, Box<dyn Error>> {
    let contents = fs::read_to_string(filename)?;
    let lines: Vec<String> = contents.split_terminator("\n").map(|x| x.into()).collect();
    return Ok(lines);
}
