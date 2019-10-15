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

    let mut parser = Parser::new(lines);
    parser.run();

    for bin in &parser.binaries {
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
    return result;
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
            .map_or("".into(), |m| m.as_str().into()),
        comp: captures["comp"].into(),
        jump: captures
            .name("jump")
            .map_or("".into(), |m| m.as_str().into()),
    };
}

fn code_to_bin(command: &Command) -> String {
    match command {
        Command::AType { value } => format!("{:0>16b}", value),
        Command::CType { dest, comp, jump } => {
            code::to_bin(dest, comp, jump)
        }
    }
}

struct Parser {
    commands: Vec<Command>,
    cursor: usize,
    binaries: Vec<String>,
}

impl Parser {
    pub fn new(lines: Vec<&str>) -> Parser {
        let filtered_lines: Vec<&str> = remove_all_white_space_and_comments(lines);
        let commands: Vec<Command> = filtered_lines.iter().map(|x| build_command(x)).collect();
        return Parser {
            commands: commands,
            cursor: 0,
            binaries: Vec::new(),
        };
    }

    pub fn has_more_commands(&self) -> bool {
        return self.cursor < self.commands.len();
    }

    pub fn current_command(&self) -> &Command {
        return &self.commands[self.cursor];
    }

    pub fn advance(&mut self) {
        self.cursor += 1;
    }

    pub fn run(&mut self) {
        while self.has_more_commands() {
            let command = self.current_command();
            let bin = code_to_bin(command);
            self.binaries.push(bin);
            self.advance();
        }
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
                dest: "D".into(),
                comp: "A".into(),
                jump: "".into(),
            },
            build_command("D=A")
        );
        assert_eq!(
            Command::CType {
                dest: "AM".into(),
                comp: "M-1".into(),
                jump: "".into(),
            },
            build_command("AM=M-1")
        );
        assert_eq!(
            Command::CType {
                dest: "".into(),
                comp: "0".into(),
                jump: "JEQ".into(),
            },
            build_command("0;JEQ")
        );
    }

    #[test]
    fn test_code_to_bin() {
        let mut command = build_command("@2");
        assert_eq!("0000000000000010", code_to_bin(&command));
        command = build_command("@133");
        assert_eq!("0000000010000101", code_to_bin(&command));
        command = build_command("D=A");
        assert_eq!("1110110000010000", code_to_bin(&command));
        command = build_command("D=D+A");
        assert_eq!("1110000010010000", code_to_bin(&command));
        command = build_command("M=D");
        assert_eq!("1110001100001000", code_to_bin(&command));
        command = build_command("D;JGT");
        assert_eq!("1110001100000001", code_to_bin(&command));
    }

    #[test]
    fn test_parser() {
        let lines = vec![
            "// this",
            "\n\n",
            "// is",
            "// a comment.",
            "\n",
            "@2",
            "D=A",
            "@3",
            "D=D+A",
            "@0",
            "M=D",
        ];
        let expected = vec![
            "0000000000000010",
            "1110110000010000",
            "0000000000000011",
            "1110000010010000",
            "0000000000000000",
            "1110001100001000",
        ];

        let mut parser = Parser::new(lines);
        parser.run();

        assert_eq!(expected.len(), parser.binaries.len());
        assert_eq!(expected, parser.binaries);
    }
}
