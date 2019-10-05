use std::error::Error;
use std::fs;

pub struct Config {
    filename: String,
}

impl Config {
    pub fn new(args: &[String]) -> Result<Config, &'static str> {
        if args.len() < 2 {
            return Err("not enough arguments");
        }
        let filename = args[1].clone();
        Ok(Config { filename })
    }
}

pub fn run(config: Config) -> Result<(), Box<dyn Error>> {
    let contents = fs::read_to_string(config.filename)?;
    let lines: Vec<&str> = contents.split_terminator("\n").collect();
    let filtered_lines: Vec<&str> = remove_all_white_space_and_comments(lines);

    println!("With filtered:\n{:?}", filtered_lines);

    Ok(())
}

fn remove_all_white_space_and_comments(lines: Vec<&str>) -> Vec<&str> {
    return lines
        .into_iter()
        .filter(|s| !s.trim().is_empty() && !s.starts_with("//"))
        .collect();
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_remove_all_white_space_and_comments() {
        let comments = vec![
            "// this",
            "\n\n",
            "// is",
            "// a comment.",
            "\n",
            "@2",
            "D=M",
        ];
        let actual = remove_all_white_space_and_comments(comments);
        assert_eq!(actual, ["@2", "D=M"]);
    }
}
