use code;
use regex::Regex;
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

    for line in filtered_lines {
        let bin = code_to_bin(line);
        println!("{}", bin);
    }

    Ok(())
}

fn remove_all_white_space_and_comments(lines: Vec<&str>) -> Vec<&str> {
    let mut result: Vec<&str> = Vec::new();
    for line in lines {
        let subs: Vec<&str> = line.split("//").collect();
        let trimmed = subs[0].trim();
        if !trimmed.is_empty() {
            result.push(trimmed);
        }
    }
    return result
}

#[derive(PartialEq, Debug)]
enum Command {
    AType {
        value: i16,
    },

    // dest=comp;jump
    CType {
        dest: String,
        comp: String,
        jump: String,
    },
}

fn build_command(line: &str) -> Command {
    if Some('@') == line.chars().nth(0) {
        let value = line[1..].parse::<i16>().unwrap();
        return Command::AType { value: value };
    }
    let re = Regex::new(r"^((?P<dest>[AMD]*)=)?(?P<comp>[^;]*)(;(?P<jump>\w{3}))?$").unwrap();
    let captures = re.captures(line).unwrap();
    return Command::CType {
        dest: captures
            .name("dest")
            .map_or("".to_string(), |m| m.as_str().to_string()),
        comp: captures["comp"].to_string(),
        jump: captures
            .name("jump")
            .map_or("".to_string(), |m| m.as_str().to_string()),
    };
}

fn c_command_to_bin(dest: String, comp: String, jump: String) -> String {
    let mut result = String::new();
    result.push_str("111");
    result.push_str(code::comp_to_bin(&comp));
    result.push_str(code::dest_to_bin(&dest));
    result.push_str(code::jump_to_bin(&jump));
    return result;
}

fn code_to_bin(code: &str) -> String {
    let command = build_command(code);
    match command {
        Command::AType { value } => format!("{:0>16b}", value),
        Command::CType { dest, comp, jump } => c_command_to_bin(dest, comp, jump),
    }
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
            "    M=D+M  // i am here",
        ];
        let actual = remove_all_white_space_and_comments(comments);
        assert_eq!(actual, ["@2", "D=M", "M=D+M"]);
    }

    #[test]
    fn test_build_command_to_a_type() {
        assert_eq!(Command::AType { value: 2 }, build_command("@2"));
        assert_eq!(Command::AType { value: 133 }, build_command("@133"));
    }

    #[test]
    fn test_build_command_to_c_type() {
        assert_eq!(
            Command::CType {
                dest: "D".to_string(),
                comp: "A".to_string(),
                jump: "".to_string(),
            },
            build_command("D=A")
        );
        assert_eq!(
            Command::CType {
                dest: "AM".to_string(),
                comp: "M-1".to_string(),
                jump: "".to_string(),
            },
            build_command("AM=M-1")
        );
        assert_eq!(
            Command::CType {
                dest: "".to_string(),
                comp: "0".to_string(),
                jump: "JEQ".to_string(),
            },
            build_command("0;JEQ")
        );
    }

    #[test]
    fn test_code_to_bin() {
        assert_eq!("0000000000000010", code_to_bin("@2"));
        assert_eq!("0000000010000101", code_to_bin("@133"));
        assert_eq!("1110110000010000", code_to_bin("D=A"));
        assert_eq!("1110000010010000", code_to_bin("D=D+A"));
        assert_eq!("1110001100001000", code_to_bin("M=D"));
        assert_eq!("1110001100000001", code_to_bin("D;JGT"));
    }
}
